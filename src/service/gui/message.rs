use crate::service::gui::{
    enums::{EventMessage, PathPythonState},
    structs::TaskId,
    sync::ReceiverHandle,
    update::{dev::DevMessage, guide::GuideMessage, install::InstallMessage},
    widgets::modal::ModalMessage,
};

#[derive(Debug, Clone)]
pub enum Message {
    EventRecieved(EventMessage),
    EventBusClosed,
    TaskFinished(TaskId),
    TaskStarted { handle: ReceiverHandle<Message> },
    ModalMessage(ModalMessage),
    HideModal,
    HomeInstallMessage(InstallMessage),
    GuideMessage(GuideMessage),
    PathPythonVersion(PathPythonState),
    RestartNeeded,
    Home,
    Log(String),
    SaveLog,
    Dev,
    Link(String),
    DevMessage(DevMessage),
}
