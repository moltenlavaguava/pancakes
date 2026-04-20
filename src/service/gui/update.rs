use iced::{Task, window};

use crate::service::gui::{MultiApp, MultiAppKind, enums::Page, message::Message};

pub mod dev;
pub mod guide;
pub mod install;

pub fn update(mapp: &mut MultiApp, msg: Message) -> Task<Message> {
    // get main window id and get the content there
    let MultiAppKind::Normal(app) = mapp
        .windows
        .get_mut(&mapp.main_id)
        .expect("Failed to get main window state")
    else {
        panic!("Main window state was incorrect type");
    };
    match msg {
        Message::EventRecieved(msg) => {
            todo!()
        }
        Message::EventBusClosed => {
            log::info!("event bus closed");
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
        Message::Log(s) => {
            // add string to the list
            app.logs.push(s);
            Task::none()
        }
        Message::Dev => {
            app.data.page = Page::Dev;
            Task::none()
        }
        Message::Link(l) => {
            log::info!("link clicked: {l}");
            let _ = open::that(l);
            Task::none()
        }
        Message::DevMessage(m) => dev::update(app, m),
        Message::Window(_, _) => {
            log::warn!("Window message update called twice");
            Task::none()
        }
        Message::CloseWindow(id) => {
            mapp.windows.remove(&id);
            println!("closing window");
            if mapp.windows.is_empty() {
                iced::exit()
            } else {
                window::close(id)
            }
        }
    }
}
