use std::ffi::OsString;
use std::process::Stdio;

use anyhow::Result;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;

use crate::service::process::enums::CompletedProgram;

use super::enums::ChildMessage;

pub async fn stream_process(
    cmd: OsString,
    args: Vec<OsString>,
    output_stream: mpsc::Sender<ChildMessage>,
) {
    let mut child = Command::new(&cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("Failed to spawn child process");

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
}
pub async fn run_process(cmd: OsString, args: Vec<OsString>) -> Result<CompletedProgram> {
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
    stream_process(cmd, args, tx).await;

    // get the program from the task
    let p = task.await??;
    Ok(p)
}

pub async fn install_uv() -> Result<()> {
    let (cmd, args) = if cfg!(target_os = "windows") {
        (
            OsString::from("powershell"),
            [
                "-ExecutionPolicy",
                "ByPass",
                "-c",
                "\"irm https://astral.sh/uv/install.ps1 | iex\"",
            ]
            .map(OsString::from),
        )
    } else {
        (
            OsString::from("curl"),
            ["-LsSf", "https://astral.sh/uv/install.sh", "|", "sh"].map(OsString::from),
        )
    };

    todo!()
}
