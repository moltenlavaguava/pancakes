use std::collections::HashMap;

use iced::Length;
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
use crate::service::gui::widgets::container::menu_content;
use crate::service::gui::widgets::text::default_text;
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

#[derive(Debug, Clone)]
pub struct App {
    communication: GuiCommunication,
    management: GuiManagement,
    data: GuiGeneralData,
    logs: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum MultiAppKind {
    Normal(App),
    Guide(u32),
}
pub struct MultiApp {
    windows: HashMap<window::Id, MultiAppKind>,
    main_id: window::Id,
    theme: Theme,
}

fn new(flags: GuiFlags) -> (MultiApp, Task<Message>) {
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
    let app = App {
        communication,
        management,
        data,
        logs: vec![],
    };

    let icon = window::icon::from_file_data(include_bytes!("../../icon.png"), None).ok();
    let (main_id, task) = window::open(window::Settings {
        icon,
        ..Default::default()
    });
    (
        MultiApp {
            windows: HashMap::new(),
            theme: Theme::Dark,
            main_id,
        },
        task.map(move |wid| Message::Window(wid, MultiAppKind::Normal(app.clone()))),
    )
}
fn update(mapp: &mut MultiApp, msg: Message) -> Task<Message> {
    // handle the window creation state first
    if let Message::Window(wid, state) = msg {
        println!("updating state here");

        let task = if let MultiAppKind::Normal(app) = &state {
            // get current path python version once this loads
            util::path_python_version(app.communication.process_sender.clone())
        } else {
            Task::none()
        };

        mapp.windows.insert(wid, state);
        return task;
    }
    update::update(mapp, msg)
}
fn view<'a>(mapp: &'a MultiApp, id: window::Id) -> Element<'a, Message> {
    let theme = &mapp.theme;
    let Some(state) = mapp.windows.get(&id) else {
        return menu_content(default_text("Loading!", theme, true, true), theme)
            .center(Length::Fill)
            .into();
    };
    let Some(MultiAppKind::Normal(app)) = &mapp.windows.get(&mapp.main_id) else {
        panic!("Failed to get app from main window entry")
    };
    match state {
        MultiAppKind::Normal(app) => match &app.data.page {
            Page::Home => home::view(app, theme),
            Page::Dev => dev::view(app, theme),
        },
        MultiAppKind::Guide(g) => guide::view(*g, app, theme, id),
    }
}
fn subscription(mapp: &MultiApp) -> Subscription<Message> {
    let Some(MultiAppKind::Normal(app)) = &mapp.windows.get(&mapp.main_id) else {
        return Subscription::none();
    };
    let bus = app.communication.event_receiver.watch(
        |_id, msg| Message::EventRecieved(msg),
        |_id| Message::EventBusClosed,
    );
    let tasks = Subscription::batch(
        app.communication
            .active_tasks
            .values()
            .map(|handle| handle.watch(|_id, msg| msg, |id| Message::TaskFinished(id))),
    );

    Subscription::batch(vec![bus, tasks])
}
fn theme(mapp: &MultiApp, id: window::Id) -> Theme {
    mapp.theme.clone()
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

    let app = iced::daemon(move || new(flags.clone()), update, view)
        .subscription(subscription)
        .theme(theme)
        .title("pancakes")
        .font(include_bytes!("../../fonts/pancakeicons.ttf").as_slice());
    app.run()
}
