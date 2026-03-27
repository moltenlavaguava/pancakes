use std::path::PathBuf;

use crate::service::{
    file::{FileSender, enums::FileMessage},
    request::structs::PythonReleaseData,
};
use anyhow::Result;
use reqwest::Client;
use tokio::sync::oneshot;

pub async fn get_python_versions(client: &Client) -> Result<Vec<PythonReleaseData>> {
    const VERSION_API_URL: &str = "https://endoflife.date/api/python.json";
    println!("Requesting versions..");
    let response = client
                    .get(VERSION_API_URL)
                    .header("User-Agent", "pancakes/0.1 (Educational tool for Python learning; more info: https://github.com/moltenlavaguava/pancakes)")
                    .send()
                    .await?;
    println!("got response: {:?}", response);
    Ok(serde_json::from_str(
        &response.error_for_status()?.text().await?,
    )?)
}
pub async fn get_cache_dir(file_sender: FileSender) -> Result<PathBuf> {
    let (tx, rx) = oneshot::channel();
    file_sender
        .send(FileMessage::GetCacheDir { response: tx })
        .await?;
    let p = rx.await?;
    Ok(p)
}
