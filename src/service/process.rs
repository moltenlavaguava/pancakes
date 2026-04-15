use tokio::sync::mpsc;

use crate::service::{
    gui::enums::EventSender,
    logic::ServiceLogic,
    process::util::{
        get_current_release_data, install_python, install_uv, path_python_version, setup_project,
        stream_process, verify_uv,
    },
};
use enums::ProcessMessage;

pub mod enums;
pub mod structs;
mod util;

// easier types
pub type ProcessSender = mpsc::Sender<ProcessMessage>;

pub struct ProcessService {
    _event_sender: EventSender,
}

impl ProcessService {
    pub fn new(event_sender: EventSender) -> Self {
        Self {
            _event_sender: event_sender,
        }
    }
}

#[async_trait::async_trait]
impl ServiceLogic<ProcessMessage> for ProcessService {
    fn name(&self) -> &'static str {
        "ProcessService"
    }
    async fn handle_message(&mut self, msg: ProcessMessage) {
        match msg {
            ProcessMessage::SpawnProcess {
                cmd,
                args,
                output_stream,
                spawn_result,
            } => {
                tokio::spawn(async move {
                    let _ = spawn_result
                        .send(stream_process(cmd, args, None::<(&str, &str)>, output_stream).await);
                });
            }
            ProcessMessage::InstallUV { result_sender } => {
                tokio::spawn(async move {
                    let install_result = install_uv().await;
                    let _ = result_sender.send(install_result);
                });
            }
            ProcessMessage::VerifyUV { result_sender } => {
                tokio::spawn(async move {
                    let _ = result_sender.send(verify_uv(true).await);
                });
            }
            ProcessMessage::UVCurrentReleaseData { result_sender } => {
                tokio::spawn(async move {
                    let _ = result_sender.send(get_current_release_data().await);
                });
            }
            ProcessMessage::PathPythonVersion { result_sender } => {
                tokio::spawn(async move {
                    let _ = result_sender.send(path_python_version().await);
                });
            }
            ProcessMessage::InstallPython {
                version,
                result_sender,
            } => {
                tokio::spawn(async move {
                    let _ = result_sender.send(install_python(version).await);
                });
            }
            ProcessMessage::SetupProject {
                path,
                version,
                result_sender,
            } => {
                tokio::spawn(async move {
                    let _ = result_sender.send(setup_project(path, version).await);
                });
            }
        }
    }
}
