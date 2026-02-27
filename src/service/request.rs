use reqwest::Client;
use tokio::sync::mpsc;

use crate::service::{
    gui::enums::EventSender, logic::ServiceLogic, request::enums::RequestMessage,
};

pub mod enums;
pub mod structs;
mod util;

// easier types
pub type RequestSender = mpsc::Sender<RequestMessage>;

/// Handles file paths.
pub struct RequestService {
    _event_sender: EventSender,
    client: Client,
}

impl RequestService {
    pub fn new(event_sender: EventSender) -> Self {
        Self {
            _event_sender: event_sender,
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
        }
    }
}
