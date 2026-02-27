use std::collections::HashMap;

use crate::service::{
    gui::{enums::EventMessage, message::Message, sync::ReceiverHandle},
    request::{
        RequestSender,
        structs::{PythonReleaseData, Release},
    },
};

#[derive(Clone)]
pub struct Counter {
    n: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

impl Counter {
    fn new() -> Counter {
        Self { n: 0 }
    }
    fn next(&mut self) -> u64 {
        self.n += 1;
        self.n
    }
}

#[derive(Clone)]
pub struct IdCounter {
    counter: Counter,
}
impl IdCounter {
    pub fn new() -> Self {
        Self {
            counter: Counter::new(),
        }
    }
    pub fn next(&mut self) -> TaskId {
        TaskId(self.counter.next())
    }
}

pub struct GuiCommunication {
    pub event_receiver: ReceiverHandle<EventMessage>,
    pub active_tasks: HashMap<TaskId, ReceiverHandle<Message>>,
    pub request_sender: RequestSender,
}
pub struct GuiManagement {
    pub task_id_counter: IdCounter,
}

pub struct GuiGeneralData {
    pub python_version_data: Option<Vec<PythonReleaseData>>,
    pub selected_python_version: Option<Release>,
}
