use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Stdio;
use std::str::FromStr;

use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use pep440_rs::Version;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use which::which;

use crate::IncludeFiles;
use crate::service::process::enums::CompletedProgram;
use crate::service::process::enums::UVVerifyResult;
use crate::service::process::structs::CurrentReleaseData;
use crate::service::process::structs::UVRawVersionOutput;

use super::enums::ChildMessage;

const NO_ENV: Option<(&str, &str)> = None;

pub async fn stream_process(
    cmd: impl AsRef<OsStr>,
    args: impl IntoIterator<Item: AsRef<OsStr>>,
    envs: impl IntoIterator<Item = (impl AsRef<OsStr>, impl AsRef<OsStr>)>,
    output_stream: mpsc::Sender<ChildMessage>,
) -> Result<()> {
    let mut command = Command::new(&cmd);

    command
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .envs(envs)
        .kill_on_drop(true);

    #[cfg(windows)]
    {
        // hide terminal windows on windows
        use std::os::windows::process::CommandExt;
        command.as_std_mut().creation_flags(0x08000000);
    }

    let mut child = command.spawn()?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let output_std = output_stream.clone();
    let output_err = output_stream.clone();

    let std_handle = tokio::spawn(async move {
        let mut reader = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            if output_std.send(ChildMessage::StdOut(line)).await.is_err() {
                return;
            }
        }
    });
    let err_handle = tokio::spawn(async move {
        let mut reader = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            if output_err.send(ChildMessage::StdErr(line)).await.is_err() {
                return;
            }
        }
    });

    let _ = std_handle.await;
    let _ = err_handle.await;
    let status = child.wait().await.unwrap();
    let _ = output_stream.send(ChildMessage::Exit(status)).await;

    Ok(())
}
pub async fn run_process(
    cmd: impl AsRef<OsStr>,
    args: impl IntoIterator<Item: AsRef<OsStr>>,
    envs: impl IntoIterator<Item = (impl AsRef<OsStr>, impl AsRef<OsStr>)>,
) -> Result<CompletedProgram> {
    let (tx, mut rx) = mpsc::channel(100);

    // spawn mini task listening to this output + consolidate it
    let task = tokio::spawn(async move {
        let mut total = Vec::new();
        while let Some(m) = rx.recv().await {
            total.push(m);
        }

        // create the completed program
        CompletedProgram::try_from(total)
    });

    // run the actual process
    stream_process(cmd, args, envs, tx).await?;

    // get the program from the task
    let p = task.await??;
    Ok(p)
}

pub fn uv_bin_dir() -> Result<PathBuf> {
    Ok(home::home_dir()
        .ok_or_else(|| anyhow!("No home dir"))?
        .join(".local")
        .join("bin"))
}
pub fn uv_exec_path() -> Result<PathBuf> {
    // search path env var first
    if let Ok(p) = which("uv") {
        Ok(p)
    } else {
        uv_bin_dir().map(|p| {
            p.join(if cfg!(target_os = "windows") {
                "uv.exe"
            } else {
                "uv"
            })
        })
    }
}

pub async fn install_uv() -> Result<()> {
    let (cmd, args) = if cfg!(target_os = "windows") {
        (
            OsString::from("powershell"),
            vec![
                "-ExecutionPolicy",
                "ByPass",
                "-Command",
                "$ProgressPreference = 'SilentlyContinue'; irm https://astral.sh/uv/install.ps1 | iex",
            ],
        )
    } else {
        (
            OsString::from("sh"),
            vec!["-c", "curl -LsSf https://astral.sh/uv/install.sh | sh"],
        )
    };

    // build env vars
    let bin_dir = uv_bin_dir().expect("Failed to get home directory");

    // run the command and wait for it to finish
    let prog = run_process(cmd, args, [("UV_INSTALL_DIR", &bin_dir)]).await?;
    println!("{prog:?}");
    if prog.was_successful() {
        Ok(())
    } else {
        Err(anyhow!("Process returned nonzero exit code"))
    }
}

pub async fn verify_uv(update: bool) -> UVVerifyResult {
    // if fresh install, use direct path instead of relying on environment variables
    let cmd = match uv_exec_path() {
        Ok(c) => c,
        Err(_) => return UVVerifyResult::Error,
    };

    let r = run_process(cmd.clone(), ["--version"], None::<(&str, &str)>).await;
    println!("{r:?}");
    let r = match r {
        Ok(r) => r,
        Err(e) => {
            // check to see if the program was not found
            if let Some(ie) = e.downcast_ref::<std::io::Error>()
                && ie.kind() == std::io::ErrorKind::NotFound
            {
                // uv not found, assume it doesn't exist
                return UVVerifyResult::NotFound;
            } else {
                eprintln!("An error occured while verifying uv: {e}");
                return UVVerifyResult::Error;
            }
        }
    };
    let up = run_process(cmd, ["self", "update"], None::<(&str, &str)>).await;
    if let Ok(p) = up
        && !p.was_successful()
    {
        return UVVerifyResult::Error;
    };
    if r.was_successful() {
        UVVerifyResult::Ok
    } else {
        eprintln!("An error occured while running uv: {:?}", r);
        UVVerifyResult::Error
    }
}
pub async fn get_current_release_data() -> Result<CurrentReleaseData> {
    let cmd = uv_exec_path()?;
    let args = ["python", "list", "--output-format", "json"];
    let prog = run_process(cmd, args, None::<(&str, &str)>).await?;

    // get json text. 1st output should be the text
    let json_txt = prog
        .messages
        .get(0)
        .ok_or(anyhow!("First child msg of uv version list must be json"))?
        .text();
    parse_uv_version_json(json_txt)
}
pub async fn path_python_version() -> Result<Option<Version>> {
    // check if there's even a python on path
    let Ok(cmd) = which("python").or_else(|_| which("python3")) else {
        return Ok(None);
    };
    // make sure this 'python' isn't just the ms store python redirector
    if cmd.to_string_lossy().contains("WindowsApps") {
        return Ok(None);
    }
    let args = [
        "-c",
        "import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}')",
    ];
    let prog = (match run_process(cmd, args, None::<(&str, &str)>).await {
        Ok(p) => Ok(p),
        Err(e) => {
            if let Some(ie) = e.downcast_ref::<std::io::Error>()
                && ie.kind() == std::io::ErrorKind::NotFound
            {
                return Ok(None);
            } else {
                Err(e)
            }
        }
    })?;

    // check the version string
    let version_txt = prog
        .messages
        .get(0)
        .ok_or(anyhow!(
            "First child message of python test output must be text"
        ))?
        .text();
    let version = Version::from_str(version_txt)?;
    Ok(Some(version))
}
pub fn parse_uv_version_json(json: &str) -> Result<CurrentReleaseData> {
    let raw_data: Vec<UVRawVersionOutput> = serde_json::from_str(json)?;
    Ok(CurrentReleaseData::from_uv_raw_version_output(raw_data))
}
pub async fn install_python(version: Version) -> Result<()> {
    let cmd = uv_exec_path()?;
    println!("Version: {version}");
    // also update the shell just in case this is the first time
    // the program runs
    let args = [
        "python",
        "install",
        &version.to_string(),
        "-r",
        "--default",
        "--preview-features",
        "python-install-default",
    ];

    let prog = run_process(cmd.clone(), args, None::<(&str, &str)>).await?;

    // update shell
    let upd_args = ["python", "update-shell"];
    let upd_prog = run_process(cmd, upd_args, None::<(&str, &str)>).await?;
    if !upd_prog.was_successful() {
        bail!("Failed to update system PATH")
    }

    println!("{prog:?}");
    Ok(())
}
pub async fn setup_project(path: PathBuf, version: Version) -> Result<()> {
    let cmd = uv_exec_path()?;
    let vp = path.join(".venv");
    let ver_string = version.to_string();
    let args = [
        OsStr::new("venv"),
        vp.as_os_str(),
        OsStr::new("-p"),
        OsStr::new(&ver_string),
    ];
    let p1 = run_process(cmd, args, NO_ENV).await?;
    println!("p1: {p1:?}");
    // set execution policy
    let cmd = "powershell";
    let args = [
        "-NoProfile",
        "-Command",
        "Set-ExecutionPolicy RemoteSigned -Scope CurrentUser -Force",
    ];
    let p2 = run_process(cmd, args, NO_ENV).await?;
    println!("p2: {p2:?}");

    // copy in include files
    for file_path in IncludeFiles::iter() {
        let file = IncludeFiles::get(&file_path)
            .ok_or(anyhow!("Failed to get file to include in project"))?;
        let output_path = path.join(file_path.as_ref());
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(output_path, file.data).await?;
    }

    Ok(())
}
