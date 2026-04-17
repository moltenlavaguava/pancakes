use iced::{Task, clipboard, wgpu::wgc::validation};

use crate::service::gui::{App, message::Message};

#[derive(Debug, Clone)]
pub enum DevMessage {
    LogScroll(f32),
    CopyLog,
    LogCopied,
}
impl Into<Message> for DevMessage {
    fn into(self) -> Message {
        Message::DevMessage(self)
    }
}

pub fn update(app: &mut App, msg: DevMessage) -> Task<Message> {
    match msg {
        DevMessage::LogScroll(offset) => {
            println!("scroll offset: {offset}");
            app.data.dev_data.log_scroll_offset = offset;
            Task::none()
        }
        DevMessage::CopyLog => {
            let t = app.logs.join("\n");
            clipboard::write(t).map(|_: ()| DevMessage::LogCopied.into())
        }
        DevMessage::LogCopied => {
            log::info!("Log copied");
            Task::none()
        }
    }
}
