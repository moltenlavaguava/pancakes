use iced::Task;

use crate::service::gui::{App, enums::Page, message::Message};

#[derive(Debug, Clone)]
pub enum GuideMessage {
    SearchText(String),
    OpenGuide(u32), // guide id
    MarkdownInteraction(String),
}

impl Into<Message> for GuideMessage {
    fn into(self) -> Message {
        Message::GuideMessage(self)
    }
}

pub fn update(app: &mut App, msg: GuideMessage) -> Task<Message> {
    match msg {
        GuideMessage::SearchText(t) => {
            app.data.learn_data.home_search = t;
            Task::none()
        }
        GuideMessage::OpenGuide(g) => {
            app.data.page = Page::Guide(g);
            Task::none()
        }
        GuideMessage::MarkdownInteraction(s) => {
            println!("interaction: {s}");
            let _ = open::that(s);
            Task::none()
        }
    }
}
