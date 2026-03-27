use iced::Task;

use crate::service::gui::{
    message::Message,
    widgets::{
        modal::{AbstractModal, Modal},
        text::default_text,
    },
};

#[derive(Debug, Clone)]
pub enum PMIMessage {}

#[derive(Debug, Clone)]
pub struct PMIModal {}

impl AbstractModal<Message> for PMIModal {
    type ModalMsg = PMIMessage;

    fn view(
        &self,
        theme: &iced::Theme,
    ) -> iced::Element<'_, super::AbstractModalMessage<Self::ModalMsg, Message>> {
        default_text("tiny text!", theme, true, true).into()
    }

    fn update(
        &mut self,
        message: Self::ModalMsg,
    ) -> iced::Task<super::AbstractModalMessage<Self::ModalMsg, Message>> {
        Task::none()
    }
}
impl Into<Modal> for PMIModal {
    fn into(self) -> Modal {
        Modal::PMI(self)
    }
}
