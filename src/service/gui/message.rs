use anyhow::Result;

use crate::service::{
    gui::{enums::EventMessage, structs::TaskId, sync::ReceiverHandle},
    request::structs::{PythonReleaseData, Release},
};

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    EventRecieved(EventMessage),
    EventBusClosed,
    TaskFinished(TaskId),
    TaskStarted {
        handle: ReceiverHandle<Message>,
    },
    RequestPythonVersions,
    PythonVersionsReceived {
        result: Option<Vec<PythonReleaseData>>,
    },
    PythonVersionSelected {
        selection: Release,
    },
}
