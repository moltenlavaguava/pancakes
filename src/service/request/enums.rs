use anyhow::Result;
use tokio::sync::oneshot;

use crate::service::request::structs::PythonReleaseData;

pub enum RequestMessage {
    QueryPythonVersions {
        response_tx: oneshot::Sender<Result<Vec<PythonReleaseData>>>,
    },
}
