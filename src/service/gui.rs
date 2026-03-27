use std::collections::HashMap;

use iced::widget::{button, column, pick_list, row};
use iced::{Element, Subscription, Task, Theme};
use tokio::sync::mpsc;

use crate::service::file::FileSender;
use crate::service::gui::enums::EventMessage;
use crate::service::gui::message::Message;
use crate::service::gui::structs::{GuiCommunication, GuiGeneralData, GuiManagement, IdCounter};
use crate::service::gui::sync::ReceiverHandle;
use crate::service::request::RequestSender;
use crate::service::request::structs::PythonReleaseData;

pub mod enums;
mod external;
mod icons;
pub mod message;
pub mod structs;
mod styling;
pub mod sync;
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
        let data = GuiGeneralData {
            python_version_data: None,
            selected_python_version_data: None,
            modal: None,
        };
        let app = Self {
            n: 0,
            communication,
            management,
            data,
        };
        // request python version data, if it exists
        let file_sender = app.communication.file_sender.clone();
        let task =
            Task::future(external::load_python_release_data(file_sender)).then(|r| match r {
                Ok(d) => Task::done(Message::PythonVersionsLoaded {
                    result: Some(d),
                    disallow_save: true,
                }),
                Err(e) => {
                    println!("An error occured while loading the python versions from file: {e}");
                    Task::none()
                }
            });
        (app, task)
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
                    |result| Message::PythonVersionsLoaded {
                        result,
                        disallow_save: false,
                    },
                )
            }
            Message::PythonVersionsLoaded {
                result,
                disallow_save,
            } => {
                // if this result is different from what is cached, save it
                if (self.data.python_version_data != result) && !disallow_save {
                    let version_data = match &result {
                        Some(d) => d.clone(),
                        None => Vec::new(),
                    };
                    self.data.python_version_data = result;
                    println!("Saving python data..");
                    let file_sender = self.communication.file_sender.clone();
                    Task::perform(
                        external::save_python_release_data(version_data, file_sender),
                        |r| {
                            if let Err(e) = r {
                                println!("An error occured while saving python release data: {e}")
                            }
                        },
                    )
                    .discard()
                } else {
                    self.data.python_version_data = result;
                    Task::none()
                }
            }
            Message::PythonVersionSelected { selection } => {
                self.data.selected_python_version_data = Some(selection);
                Task::none()
            }
            Message::DownloadSelectedPython => {
                let request_sender = self.communication.request_sender.clone();
                match &self.data.selected_python_version_data {
                    Some(d) => Task::perform(
                        external::download_selected_python(request_sender, d.clone()),
                        |_| {},
                    )
                    .discard(),
                    None => Task::none(),
                }
            }
            Message::HideModal => {
                self.data.modal = None;
                Task::none()
            }
            Message::ModalMessage(m) => {
                if let Some(modal) = &mut self.data.modal {
                    modal.update(m)
                } else {
                    Task::none()
                }
            }
        }
    }
    fn view<'a>(&'a self) -> Element<'a, Message> {
        let options = if let Some(d) = &self.data.python_version_data {
            util::filter_compiled_python_versions(d).collect()
        } else {
            Vec::new()
        };
        column![
            row![
                button("Request Versions").on_press(Message::RequestPythonVersions),
                button("Download Python").on_press(Message::DownloadSelectedPython)
            ],
            pick_list(
                options,
                self.data.selected_python_version_data.as_ref(),
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
