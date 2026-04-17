use std::path::PathBuf;

use iced::Task;
use pep440_rs::Version;

use crate::service::{
    gui::{
        external,
        message::Message,
        widgets::modal::{ModalMessage, environment::EnvMessage, install::InstallModalMsg},
    },
    process::{ProcessSender, enums::UVVerifyResult},
};

// Sends the request to verify uv and maps the result to the install modal
pub fn verify_uv_to_modal(process_sender: ProcessSender) -> Task<Message> {
    Task::perform(external::verify_uv(process_sender), |r| {
        let im =
            InstallModalMsg::UVStatusReceived(r.expect("Failed to verify uv installation").into());
        let mm: ModalMessage = im.into();
        mm.into()
    })
}
pub fn install_uv_to_modal(process_sender: ProcessSender) -> Task<Message> {
    Task::perform(external::install_uv(process_sender), |r| {
        let vr = match r {
            Ok(_) => UVVerifyResult::Ok,
            Err(e) => {
                log::error!("An error occured while installing uv: {e}");
                UVVerifyResult::Error
            }
        };
        let im = InstallModalMsg::UVStatusReceived(vr.into());
        let mm: ModalMessage = im.into();
        mm.into()
    })
}
pub fn install_python_to_modal(process_sender: ProcessSender, version: Version) -> Task<Message> {
    Task::perform(external::install_python(process_sender, version), |r| {
        let im = InstallModalMsg::PythonInstalled(r.is_ok());
        let mm: ModalMessage = im.into();
        mm.into()
    })
}
pub fn current_release_data_to_modal(process_sender: ProcessSender) -> Task<Message> {
    Task::perform(external::uv_current_release_data(process_sender), |r| {
        let crd = r.expect("Failed to get current release data from uv");
        let im = InstallModalMsg::ReleaseDataReceived(crd.into());
        let mm: ModalMessage = im.into();
        mm.into()
    })
}
pub fn path_python_version(process_sender: ProcessSender) -> Task<Message> {
    Task::perform(external::path_python_version(process_sender), |r| {
        Message::PathPythonVersion(r.into())
    })
}
pub fn setup_project_to_modal(
    process_sender: ProcessSender,
    path: PathBuf,
    version: Version,
) -> Task<Message> {
    Task::perform(
        external::setup_project(process_sender, path, version),
        |r| {
            let em = EnvMessage::SetupDone(r.is_ok());
            let mm: ModalMessage = em.into();
            mm.into()
        },
    )
}
