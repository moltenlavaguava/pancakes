use std::collections::HashMap;

use futures::channel::mpsc::UnboundedReceiver;
use iced::{Element, Subscription, Task, Theme, window};
use tokio::sync::mpsc;

use crate::service::file::FileSender;
use crate::service::gui::enums::{EventMessage, Page, PathPythonState};
use crate::service::gui::learn::LearnData;
use crate::service::gui::message::Message;
use crate::service::gui::page::guide::{self, GuideRegistry};
use crate::service::gui::page::{dev, home};
use crate::service::gui::structs::{
    DevLogData, GuiCommunication, GuiGeneralData, GuiManagement, IdCounter, ImageRegistry,
};
use crate::service::gui::sync::ReceiverHandle;
use crate::service::process::ProcessSender;
use crate::service::request::RequestSender;

pub mod enums;
mod external;
mod icons;
mod learn;
pub mod logging;
pub mod message;
mod page;
pub mod structs;
mod styling;
pub mod sync;
mod update;
mod util;
mod widgets;

pub struct App {
    communication: GuiCommunication,
    management: GuiManagement,
    data: GuiGeneralData,
    logs: Vec<String>,
    theme: Theme,
}
impl App {
    fn new(flags: GuiFlags) -> (Self, Task<Message>) {
        // create log listening task
        let mut active_tasks = HashMap::new();
        active_tasks.insert(flags.log_rx.id(), flags.log_rx);

        let communication = GuiCommunication {
            event_receiver: flags.receiver_handle,
            active_tasks,
            request_sender: flags.request_sender,
            file_sender: flags.file_sender,
            process_sender: flags.process_sender,
        };
        let management = GuiManagement {
            task_id_counter: flags.task_id_counter,
        };
        let ir = ImageRegistry::new();
        let gr = GuideRegistry::new();
        let learn_data = LearnData {
            home_search: String::new(),
            search_match_guide_ids: gr.guides.keys().map(|k| *k).collect(),
        };
        let data = GuiGeneralData {
            modal: None,
            page: Page::Home,
            learn_data,
            path_python_version: PathPythonState::Unknown,
            restart_needed: false,
            image_registry: ir,
            guide_registry: gr,
            dev_data: DevLogData::new(),
        };
        let app = Self {
            communication,
            management,
            data,
            logs: vec![],
            theme: Theme::Dark,
        };
        // get current path python version
        let task = util::path_python_version(app.communication.process_sender.clone());
        (app, task)
    }
    fn update(&mut self, msg: Message) -> Task<Message> {
        update::update(self, msg)
    }
    fn view<'a>(&'a self) -> Element<'a, Message> {
        match &self.data.page {
            Page::Home => home::view(self),
            Page::Guide(id) => guide::view(*id, self),
            Page::Dev => dev::view(self),
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
        self.theme.clone()
    }
}

#[derive(Clone)]
struct GuiFlags {
    receiver_handle: ReceiverHandle<EventMessage>,
    request_sender: RequestSender,
    process_sender: ProcessSender,
    file_sender: FileSender,
    task_id_counter: IdCounter,
    log_rx: ReceiverHandle<Message>,
}

pub fn run_gui(
    event_reciever: mpsc::Receiver<EventMessage>,
    request_sender: RequestSender,
    file_sender: FileSender,
    process_sender: ProcessSender,
    log_rx: mpsc::UnboundedReceiver<String>,
) -> iced::Result {
    // convert basic event receiver to handle
    let mut task_id_counter = IdCounter::new();
    let receiver_handle = ReceiverHandle::new(task_id_counter.next(), event_reciever);

    // create logging rh
    let log_rx = ReceiverHandle::new_unbounded(task_id_counter.next(), log_rx).map(Message::Log);

    let flags = GuiFlags {
        receiver_handle,
        task_id_counter,
        request_sender,
        file_sender,
        process_sender,
        log_rx,
    };

    let icon = window::icon::from_file_data(include_bytes!("../../icon.png"), None).ok();

    let app = iced::application(move || App::new(flags.clone()), App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .title("pancakes")
        .font(include_bytes!("../../fonts/pancakeicons.ttf").as_slice())
        .window(window::Settings {
            icon,
            ..Default::default()
        })
        .exit_on_close_request(true);
    app.run()
}
