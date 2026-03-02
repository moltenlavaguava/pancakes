use anyhow::Result;
use tokio::sync::oneshot;

use crate::service::{
    file::{
        FileSender,
        enums::{Directory, FileMessage},
    },
    request::{RequestSender, enums::RequestMessage, structs::PythonReleaseData},
};

pub async fn request_python_versions(
    request_sender: RequestSender,
) -> Option<Vec<PythonReleaseData>> {
    let (tx, rx) = oneshot::channel();
    let m = RequestMessage::QueryPythonVersions { response_tx: tx };
    let r = request_sender.send(m).await;
    if let Ok(_) = r {
        let r = rx.await;
        println!("(g) response: {:?}", r);
        match r {
            Ok(r) => match r {
                Ok(r) => Some(r),
                Err(_) => None,
            },
            Err(_) => None,
        }
    } else {
        None
    }
}

const PYTHON_VERSION_DATA_FILENAME: &str = "python-versions.json";

pub async fn save_python_release_data(
    version_data: Vec<PythonReleaseData>,
    file_sender: FileSender,
) -> Result<()> {
    let (tx, rx) = oneshot::channel();
    // serialize stuff to text
    let txt = serde_json::to_string(&version_data)?;
    let _ = file_sender
        .send(FileMessage::SaveString {
            response: tx,
            directory: Directory::Cache(String::from(PYTHON_VERSION_DATA_FILENAME)),
            data: txt,
        })
        .await?;
    let _ = rx.await?;
    Ok(())
}
pub async fn load_python_release_data(file_sender: FileSender) -> Result<Vec<PythonReleaseData>> {
    let (tx, rx) = oneshot::channel();
    let _ = file_sender
        .send(FileMessage::LoadString {
            response: tx,
            directory: Directory::Cache(String::from(PYTHON_VERSION_DATA_FILENAME)),
        })
        .await?;
    let txt = rx.await??;
    let data: Vec<PythonReleaseData> = serde_json::from_str(&txt)?;
    Ok(data)
}
