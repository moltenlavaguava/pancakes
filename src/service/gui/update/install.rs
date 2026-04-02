use iced::Task;

use crate::service::gui::{App, message::Message};

#[derive(Debug, Clone)]
pub enum InstallMessage {
    InstallPython,
    TestPython,
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
            Task::none()
        }
        InstallMessage::TestPython => {
            println!("Test Python");
            Task::none()
        }
    }
}
