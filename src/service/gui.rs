use std::collections::HashMap;

use iced::{Element, Subscription, Task, Theme};
use tokio::sync::mpsc;

use crate::service::file::FileSender;
use crate::service::gui::enums::{EventMessage, Page};
use crate::service::gui::learn::LearnData;
use crate::service::gui::message::Message;
use crate::service::gui::page::home;
use crate::service::gui::structs::{GuiCommunication, GuiGeneralData, GuiManagement, IdCounter};
use crate::service::gui::sync::ReceiverHandle;
use crate::service::request::RequestSender;

pub mod enums;
mod external;
mod icons;
mod learn;
pub mod message;
mod page;
pub mod structs;
mod styling;
pub mod sync;
mod update;
mod util;
mod widgets;

pub struct App {
    n: i32,
    communication: GuiCommunication,
    management: GuiManagement,
    data: GuiGeneralData,
}
impl App {
    fn new(flags: GuiFlags) -> (Self, Task<Message>) {
        let communication = GuiCommunication {
            event_receiver: flags.receiver_handle,
            active_tasks: HashMap::new(),
            request_sender: flags.request_sender,
            file_sender: flags.file_sender,
        };
        let management = GuiManagement {
            task_id_counter: flags.task_id_counter,
        };
        let learn_data = LearnData {
            home_search: String::new(),
        };
        let data = GuiGeneralData {
            modal: None,
            page: Page::Home,
            learn_data,
        };
        let app = Self {
            n: 0,
            communication,
            management,
            data,
        };
        // request python version data, if it exists
        let file_sender = app.communication.file_sender.clone();
        let task = Task::none();
        (app, task)
    }
    fn update(&mut self, msg: Message) -> Task<Message> {
        update::update(self, msg)
    }
    fn view<'a>(&'a self) -> Element<'a, Message> {
        match &self.data.page {
            Page::Home => home::view(self),
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        let bus = self.communication.event_receiver.watch(
            |_id, msg| Message::EventRecieved(msg),
            |_id| Message::EventBusClosed,
        );
        let tasks = Subscription::batch(
            self.communication
                .active_tasks
                .values()
                .map(|handle| handle.watch(|_id, msg| msg, |id| Message::TaskFinished(id))),
        );

        Subscription::batch(vec![bus, tasks])
    }
    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

#[derive(Clone)]
struct GuiFlags {
    receiver_handle: ReceiverHandle<EventMessage>,
    request_sender: RequestSender,
    file_sender: FileSender,
    task_id_counter: IdCounter,
}

pub fn run_gui(
    event_reciever: mpsc::Receiver<EventMessage>,
    request_sender: RequestSender,
    file_sender: FileSender,
) -> iced::Result {
    // convert basic event receiver to handle
    let mut task_id_counter = IdCounter::new();
    let receiver_handle = ReceiverHandle::new(task_id_counter.next(), event_reciever);

    let flags = GuiFlags {
        receiver_handle,
        task_id_counter,
        request_sender,
        file_sender,
    };

    let app = iced::application(move || App::new(flags.clone()), App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .title("pancakes")
        .exit_on_close_request(true);
    app.run()
}
