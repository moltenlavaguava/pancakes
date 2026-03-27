use crate::service::{
    gui::{
        enums::EventMessage, structs::TaskId, sync::ReceiverHandle, widgets::modal::ModalMessage,
    },
    request::structs::PythonReleaseData,
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
    PythonVersionsLoaded {
        result: Option<Vec<PythonReleaseData>>,
        disallow_save: bool,
    },
    PythonVersionSelected {
        selection: PythonReleaseData,
    },
    DownloadSelectedPython,
    ModalMessage(ModalMessage),
    HideModal,
}
