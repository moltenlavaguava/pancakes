use iced::window;

use crate::service::gui::{
    MultiAppKind,
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
    Dev,
    Link(String),
    DevMessage(DevMessage),
    // 1st: window, 2nd: guide
    Window(window::Id, MultiAppKind),
    CloseWindow(window::Id),
}
