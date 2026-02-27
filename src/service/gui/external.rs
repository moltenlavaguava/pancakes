use tokio::sync::oneshot;

use crate::service::request::{RequestSender, enums::RequestMessage, structs::PythonReleaseData};

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
