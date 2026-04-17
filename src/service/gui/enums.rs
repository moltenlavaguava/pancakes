use anyhow::Result;
use pep440_rs::Version;
use tokio::sync::mpsc;

use crate::service::gui::page::guide::Guide;

pub type EventSender = mpsc::Sender<EventMessage>;

#[derive(Debug, Clone)]
pub enum EventMessage {}

pub enum Page {
    Home,
    Guide(u32), // guide id
    Dev,
}

#[derive(Debug, Clone)]
pub enum PathPythonState {
    NotFound,
    Version(Version),
    Error,
    Unknown,
}
impl Into<PathPythonState> for Result<Option<Version>> {
    fn into(self) -> PathPythonState {
        match self {
            Ok(o) => match o {
                Some(v) => PathPythonState::Version(v),
                None => PathPythonState::NotFound,
            },
            Err(_) => PathPythonState::Error,
        }
    }
}
