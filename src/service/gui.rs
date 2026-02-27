use std::collections::HashMap;

use iced::widget::{button, column, pick_list, text};
use iced::{Element, Subscription, Task, Theme};
use tokio::sync::mpsc;

use crate::service::gui::enums::EventMessage;
use crate::service::gui::message::Message;
use crate::service::gui::structs::{GuiCommunication, GuiGeneralData, GuiManagement, IdCounter};
use crate::service::gui::sync::ReceiverHandle;
use crate::service::request::RequestSender;
use crate::service::request::structs::Release;

pub mod enums;
mod external;
pub mod message;
pub mod structs;
pub mod sync;

pub struct App {
    n: i32,
    communication: GuiCommunication,
    management: GuiManagement,
    data: GuiGeneralData,
}
impl App {
    fn new(flags: GuiFlags) -> Self {
        let communication = GuiCommunication {
            event_receiver: flags.receiver_handle,
            active_tasks: HashMap::new(),
            request_sender: flags.request_sender,
        };
        let management = GuiManagement {
            task_id_counter: flags.task_id_counter,
        };
        let data = GuiGeneralData {
            python_version_data: None,
            selected_python_version: None,
        };
        App {
            n: 0,
            communication,
            management,
            data,
        }
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
            Message::EventRecieved(msg) => {
                todo!()
            }
            Message::EventBusClosed => {
                println!("event bus closed");
                Task::none()
            }
            Message::TaskFinished(id) => {
                self.communication.active_tasks.remove(&id);
                Task::none()
            }
            Message::TaskStarted { handle } => {
                self.communication.active_tasks.insert(handle.id(), handle);
                Task::none()
            }
            Message::RequestPythonVersions => {
                let request_sender = self.communication.request_sender.clone();
                Task::perform(
                    external::request_python_versions(request_sender),
                    |result| Message::PythonVersionsReceived { result },
                )
            }
            Message::PythonVersionsReceived { result } => {
                self.data.python_version_data = result;
                Task::none()
            }
            Message::PythonVersionSelected { selection } => {
                //
                self.data.selected_python_version = Some(selection);
                Task::none()
            }
        }
    }
    fn view<'a>(&'a self) -> Element<'a, Message> {
        let options: Vec<&Release> = if let Some(d) = &self.data.python_version_data {
            d.iter().map(|data| &data.major_release).collect()
        } else {
            Vec::new()
        };
        column![
            button("Request Versions").on_press(Message::RequestPythonVersions),
            pick_list(
                options,
                self.data.selected_python_version.as_ref(),
                |selection| {
                    Message::PythonVersionSelected {
                        selection: selection.clone(),
                    }
                }
            ),
        ]
        .into()
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
    task_id_counter: IdCounter,
}

pub fn run_gui(
    event_reciever: mpsc::Receiver<EventMessage>,
    request_sender: RequestSender,
) -> iced::Result {
    // convert basic event receiver to handle
    let mut task_id_counter = IdCounter::new();
    let receiver_handle = ReceiverHandle::new(task_id_counter.next(), event_reciever);

    let flags = GuiFlags {
        receiver_handle,
        task_id_counter,
        request_sender,
    };

    let app = iced::application(move || App::new(flags.clone()), App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .title("pancakes")
        .exit_on_close_request(true);
    app.run()
}
