use std::collections::HashMap;

use crate::service::{
    file::FileSender,
    gui::{
        enums::{EventMessage, Page},
        learn::LearnData,
        message::Message,
        sync::ReceiverHandle,
        widgets::modal::Modal,
    },
    request::RequestSender,
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
    pub file_sender: FileSender,
}
pub struct GuiManagement {
    pub task_id_counter: IdCounter,
}

pub struct GuiGeneralData {
    pub modal: Option<Modal>,
    pub learn_data: LearnData,
    pub page: Page,
}
