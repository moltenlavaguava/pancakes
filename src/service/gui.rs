use iced::widget::{button, column, text};
use iced::{Element, Subscription, Task, Theme};

use crate::service::gui::message::Message;

pub mod enums;
pub mod message;
pub mod structs;
pub mod sync;

pub struct App {
    n: i32,
}
impl App {
    fn new() -> Self {
        App { n: 0 }
    }
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Decrement => {
                self.n -= 1;
                Task::none()
            }
            Message::Increment => {
                self.n += 1;
                Task::none()
            }
        }
    }
    fn view<'a>(&'a self) -> Element<'a, Message> {
        column![
            button("more").on_press(Message::Increment),
            text(self.n),
            button("less").on_press(Message::Decrement)
        ]
        .into()
    }
    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

pub fn run_gui() -> iced::Result {
    let app = iced::application(move || App::new(), App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .title("pancakes")
        .exit_on_close_request(true);
    app.run()
}
