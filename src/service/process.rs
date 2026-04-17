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
                    let r = stream_process(cmd, args, None::<(&str, &str)>, output_stream).await;
                    if let Err(e) = &r {
                        log::error!("An error occured while spawning a process: {e}");
                    };
                    let _ = spawn_result.send(r);
                });
            }
            ProcessMessage::InstallUV { result_sender } => {
                tokio::spawn(async move {
                    let install_result = install_uv().await;
                    if let Err(e) = &install_result {
                        log::error!("An error occured while installing uv: {e}");
                    };
                    let _ = result_sender.send(install_result);
                });
            }
            ProcessMessage::VerifyUV { result_sender } => {
                tokio::spawn(async move {
                    let r = verify_uv(true).await;
                    let _ = result_sender.send(r);
                });
            }
            ProcessMessage::UVCurrentReleaseData { result_sender } => {
                let r = get_current_release_data().await;
                if let Err(e) = &r {
                    log::error!("An error occured while getting uv current release data: {e}");
                };
                tokio::spawn(async move {
                    let _ = result_sender.send(r);
                });
            }
            ProcessMessage::PathPythonVersion { result_sender } => {
                tokio::spawn(async move {
                    let r = path_python_version().await;

                    if let Err(e) = &r {
                        log::error!("An error occured while getting path python version: {e}");
                    };

                    let _ = result_sender.send(r);
                });
            }
            ProcessMessage::InstallPython {
                version,
                result_sender,
            } => {
                tokio::spawn(async move {
                    let r = install_python(version).await;
                    if let Err(e) = &r {
                        log::error!("An error occured while installing python: {e}");
                    };
                    let _ = result_sender.send(r);
                });
            }
            ProcessMessage::SetupProject {
                path,
                version,
                result_sender,
            } => {
                tokio::spawn(async move {
                    let r = setup_project(path, version).await;
                    if let Err(e) = &r {
                        log::error!("An error occured while setting up project: {e}");
                    };
                    let _ = result_sender.send(r);
                });
            }
        }
    }
}
