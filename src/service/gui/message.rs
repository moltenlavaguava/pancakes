use crate::service::gui::{
    enums::EventMessage,
    structs::TaskId,
    sync::ReceiverHandle,
    update::{install::InstallMessage, learn::LearnMessage},
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
    HomeLearnMessage(LearnMessage),
}
