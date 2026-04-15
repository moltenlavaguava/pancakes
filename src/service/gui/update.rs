use iced::Task;

use crate::service::gui::{App, enums::Page, message::Message};

pub mod guide;
pub mod install;

pub fn update(app: &mut App, msg: Message) -> Task<Message> {
    match msg {
        Message::EventRecieved(msg) => {
            todo!()
        }
        Message::EventBusClosed => {
            println!("event bus closed");
            Task::none()
        }
        Message::TaskFinished(id) => {
            app.communication.active_tasks.remove(&id);
            Task::none()
        }
        Message::TaskStarted { handle } => {
            app.communication.active_tasks.insert(handle.id(), handle);
            Task::none()
        }
        Message::HideModal => {
            app.data.modal = None;
            Task::none()
        }
        Message::ModalMessage(m) => {
            if let Some(mut modal) = app.data.modal.take() {
                let t = modal.update(app, m);
                app.data.modal = Some(modal);
                t
            } else {
                Task::none()
            }
        }
        Message::HomeInstallMessage(m) => install::update(app, m),
        Message::GuideMessage(m) => guide::update(app, m),
        Message::PathPythonVersion(v) => {
            app.data.path_python_version = v;
            Task::none()
        }
        Message::RestartNeeded => {
            app.data.restart_needed = true;
            Task::none()
        }
        Message::Home => {
            app.data.page = Page::Home;
            Task::none()
        }
    }
}
