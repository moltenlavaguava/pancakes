use reqwest::Client;
use tokio::sync::mpsc;

use crate::service::{
    file::FileSender, gui::enums::EventSender, logic::ServiceLogic, request::enums::RequestMessage,
};

pub mod enums;
mod installer;
pub mod structs;
mod util;

// easier types
pub type RequestSender = mpsc::Sender<RequestMessage>;

/// Handles file paths.
pub struct RequestService {
    _event_sender: EventSender,
    file_sender: FileSender,
    client: Client,
}

impl RequestService {
    pub fn new(event_sender: EventSender, file_sender: FileSender) -> Self {
        Self {
            _event_sender: event_sender,
            file_sender,
            client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl ServiceLogic<RequestMessage> for RequestService {
    fn name(&self) -> &'static str {
        "RequestService"
    }
    // Note: currently this service only handles one request at a time.
    // More requests can be done at a time by Arcing the client, but to save
    // bandwith this is not being done
    async fn handle_message(&mut self, msg: RequestMessage) {
        match msg {
            RequestMessage::QueryPythonVersions { response_tx } => {
                let _ = response_tx.send(util::get_python_versions(&self.client).await);
            }
            RequestMessage::DownloadPython {
                release_data,
                response_tx,
            } => {
                // request cache data dir from file serivce
                let mut installer_dest_folder =
                    match util::get_cache_dir(self.file_sender.clone()).await {
                        Ok(f) => f,
                        Err(e) => {
                            let _ = response_tx.send(Err(e));
                            return;
                        }
                    };
                installer_dest_folder.push("installers");
                let _ = response_tx.send(
                    installer::download_python(&release_data, &self.client, installer_dest_folder)
                        .await,
                );
            }
        }
    }
}
