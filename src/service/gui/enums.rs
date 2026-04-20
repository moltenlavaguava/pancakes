use anyhow::Result;
use pep440_rs::Version;
use tokio::sync::mpsc;

pub type EventSender = mpsc::Sender<EventMessage>;

#[derive(Debug, Clone)]
pub enum EventMessage {}

#[derive(Debug, Clone)]
pub enum Page {
    Home,
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
