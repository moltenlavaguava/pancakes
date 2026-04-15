use std::path::PathBuf;

use anyhow::Result;
use pep440_rs::Version;
use tokio::sync::oneshot;

use crate::service::process::{
    ProcessSender,
    enums::{ProcessMessage, UVVerifyResult},
    structs::CurrentReleaseData,
};

pub async fn verify_uv(process_sender: ProcessSender) -> Result<UVVerifyResult> {
    let (tx, rx) = oneshot::channel();
    process_sender
        .send(ProcessMessage::VerifyUV { result_sender: tx })
        .await?;
    let r = rx.await?;
    Ok(r)
}
pub async fn install_uv(process_sender: ProcessSender) -> Result<()> {
    let (tx, rx) = oneshot::channel();
    process_sender
        .send(ProcessMessage::InstallUV { result_sender: tx })
        .await?;
    let r = rx.await??;
    Ok(r)
}
pub async fn uv_current_release_data(process_sender: ProcessSender) -> Result<CurrentReleaseData> {
    let (tx, rx) = oneshot::channel();
    process_sender
        .send(ProcessMessage::UVCurrentReleaseData { result_sender: tx })
        .await?;
    let r = rx.await??;
    Ok(r)
}
pub async fn path_python_version(process_sender: ProcessSender) -> Result<Option<Version>> {
    let (tx, rx) = oneshot::channel();
    process_sender
        .send(ProcessMessage::PathPythonVersion { result_sender: tx })
        .await?;
    let r = rx.await??;
    Ok(r)
}
pub async fn install_python(process_sender: ProcessSender, version: Version) -> Result<()> {
    let (tx, rx) = oneshot::channel();
    process_sender
        .send(ProcessMessage::InstallPython {
            version,
            result_sender: tx,
        })
        .await?;
    let r = rx.await??;
    Ok(r)
}
pub async fn setup_project(
    process_sender: ProcessSender,
    path: PathBuf,
    version: Version,
) -> Result<()> {
    let (tx, rx) = oneshot::channel();
    process_sender
        .send(ProcessMessage::SetupProject {
            version,
            path,
            result_sender: tx,
        })
        .await?;
    let r = rx.await??;
    Ok(r)
}
