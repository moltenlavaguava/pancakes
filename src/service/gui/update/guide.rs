use iced::Task;

use crate::service::gui::{App, enums::Page, message::Message};

#[derive(Debug, Clone)]
pub enum GuideMessage {
    SearchText(String),
    OpenGuide(u32), // guide id
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

            // filter guides
            let include_guide_ids: Vec<u32> = app
                .data
                .guide_registry
                .guides
                .iter()
                .filter(|(_, guide)| {
                    let lower_name = guide.name.to_lowercase();
                    let mut lower_tags = guide.tags.iter().map(|t| t.to_lowercase());
                    let lower_search = app.data.learn_data.home_search.to_lowercase();
                    if lower_name.contains(&lower_search) {
                        return true;
                    }
                    if lower_tags.any(|t| t.contains(&lower_search)) {
                        return true;
                    }
                    false
                })
                .map(|(id, _)| *id)
                .collect();

            app.data.learn_data.search_match_guide_ids = include_guide_ids;

            Task::none()
        }
        GuideMessage::OpenGuide(g) => {
            app.data.page = Page::Guide(g);
            Task::none()
        }
    }
}
