use iced::Task;

use crate::service::gui::{App, external, message::Message};

pub mod install;
pub mod learn;

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
        Message::HomeLearnMessage(m) => learn::update(app, m),
    }
}
