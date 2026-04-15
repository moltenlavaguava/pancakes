use iced::Task;

use crate::service::gui::{
    App,
    message::Message,
    util,
    widgets::modal::{Modal, environment::EnvModal, install::InstallModal},
};

#[derive(Debug, Clone)]
pub enum InstallMessage {
    InstallPython,
    Environment,
}
impl Into<Message> for InstallMessage {
    fn into(self) -> Message {
        Message::HomeInstallMessage(self)
    }
}

pub fn update(app: &mut App, msg: InstallMessage) -> Task<Message> {
    match msg {
        InstallMessage::InstallPython => {
            println!("Install python");
            // create the install python modal
            let modal = InstallModal::new();
            app.data.modal = Some(Modal::Install(modal));
            // check for uv installation for modal
            util::verify_uv_to_modal(app.communication.process_sender.clone())
        }
        InstallMessage::Environment => {
            println!("Setup virtual environment");
            let modal = EnvModal::new();
            app.data.modal = Some(Modal::Environment(modal));
            Task::none()
        }
    }
}
