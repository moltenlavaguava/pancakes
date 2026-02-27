use tokio::sync::mpsc;

pub type EventSender = mpsc::Sender<EventMessage>;

#[derive(Debug, Clone)]
pub enum EventMessage {}
