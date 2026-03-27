use std::{env, path::PathBuf};

use anyhow::{Result, bail};
use reqwest::Client;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::service::request::structs::{PythonReleaseData, Release};

#[derive(Debug, Clone)]
pub enum DownloadMethod {
    Official { url: String, filename: String },
    Portable { title: String },
}

fn get_download_method(release: &Release) -> Result<DownloadMethod> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    let release_str = &release.to_string();
    // match combo of operating system + architecture
    match (os, arch) {
        // 90% of users will have this
        ("windows", "x86_64") => {
            let filename = format!("python-{}-amd64.exe", release_str);
            let url = format!(
                "https://www.python.org/ftp/python/{}/{}",
                release_str, filename
            );
            Ok(DownloadMethod::Official { url, filename })
        }
        // very few people have this
        ("windows", "aarch64") => {
            let filename = format!("python-{}-aarch64.exe", release_str);
            let url = format!(
                "https://www.python.org/ftp/python/{}/{}",
                release_str, filename
            );
            Ok(DownloadMethod::Official { url, filename })
        }
        ("macos", "x86_64") | ("macos", "aarch64") => {
            // make sure the version is more than python 3.10
            if !(release.major >= 3 && release.minor > 10) {
                bail!("Outdated python version for macos")
            }
            let filename = format!("python-{}-macos11.pkg", release_str);
            let url = format!(
                "https://www.python.org/ftp/python/{}/{}",
                release_str, filename,
            );
            Ok(DownloadMethod::Official { url, filename })
        }
        ("linux", "x86_64") => Ok(DownloadMethod::Portable {
            title: String::from("x86_64-unknown-linux-gnu"),
        }),
        ("linux", "aarch64") => Ok(DownloadMethod::Portable {
            title: String::from("aarch64-unknown-linux-gnu"),
        }),
        _ => bail!("Unsupported system"),
    }
}
async fn download_python_installer(
    installer_dest_folder: PathBuf,
    filename: String,
    url: String,
    client: &Client,
) -> Result<()> {
    let response = client.get(url).send().await?;
    let download_size = response.content_length().unwrap_or(0);
    let installer_path_buf = installer_dest_folder.join(filename);
    let mut file = File::create(installer_path_buf).await?;
    let mut downloaded: u64 = 0;

    let mut response_stream = response;
    while let Ok(maybe_chunk) = response_stream.chunk().await {
        let chunk = match maybe_chunk {
            Some(c) => c,
            None => break,
        };
        // write chunk to disc
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        println!(
            "{}% downloaded",
            (downloaded as f32) / (download_size as f32) * 100.0
        )
    }
    println!("Done downloading");
    Ok(())
}

pub async fn download_python(
    release: &PythonReleaseData,
    client: &Client,
    installer_dest_folder: PathBuf,
) -> Result<()> {
    let download_method = get_download_method(&release.latest)?;
    match download_method {
        DownloadMethod::Official { url, filename } => {
            download_python_installer(installer_dest_folder, filename, url, client).await
        }
        DownloadMethod::Portable { title } => {
            todo!()
        }
    }
}
