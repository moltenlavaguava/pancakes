use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

use crate::service::request::structs::PythonReleaseData;

pub enum RequestMessage {
    QueryPythonVersions {
        response_tx: oneshot::Sender<Result<Vec<PythonReleaseData>>>,
    },
    DownloadPython {
        release_data: PythonReleaseData,
        response_tx: oneshot::Sender<Result<()>>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DateBool {
    Date(NaiveDate),
    Bool(bool),
}
