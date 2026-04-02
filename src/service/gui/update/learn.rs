use iced::Task;

use crate::service::gui::{App, message::Message};

#[derive(Debug, Clone)]
pub enum LearnMessage {
    SearchText(String),
}
impl Into<Message> for LearnMessage {
    fn into(self) -> Message {
        Message::HomeLearnMessage(self)
    }
}

pub fn update(app: &mut App, msg: LearnMessage) -> Task<Message> {
    match msg {
        LearnMessage::SearchText(t) => {
            app.data.learn_data.home_search = t;
            Task::none()
        }
    }
}
