use std::{collections::HashMap, path::Path};

use iced::widget::{image, markdown};

use crate::service::{
    file::FileSender,
    gui::{
        enums::{EventMessage, Page, PathPythonState},
        learn::LearnData,
        message::Message,
        page::guide::GuideRegistry,
        sync::ReceiverHandle,
        widgets::modal::Modal,
    },
    process::ProcessSender,
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

#[derive(Debug, Clone)]
pub struct DevLogData {
    pub log_scroll_offset: f32,
    pub info_markdown: Vec<markdown::Item>,
}
impl DevLogData {
    pub fn new() -> Self {
        let info_txt = "**Note:** this page is only for debugging! It only exists to get more information about errors. \
    If you do not know what you are doing, click the back arrow above.";
        let info_markdown: Vec<markdown::Item> = markdown::parse(info_txt).collect();

        Self {
            log_scroll_offset: 0.0,
            info_markdown,
        }
    }
}

pub struct GuiCommunication {
    pub event_receiver: ReceiverHandle<EventMessage>,
    pub active_tasks: HashMap<TaskId, ReceiverHandle<Message>>,
    pub request_sender: RequestSender,
    pub file_sender: FileSender,
    pub process_sender: ProcessSender,
}
pub struct GuiManagement {
    pub task_id_counter: IdCounter,
}

pub struct GuiGeneralData {
    pub modal: Option<Modal>,
    pub learn_data: LearnData,
    pub page: Page,
    pub restart_needed: bool,
    pub path_python_version: PathPythonState,
    pub image_registry: ImageRegistry,
    pub guide_registry: GuideRegistry,
    pub dev_data: DevLogData,
}

// image registry handling
#[derive(rust_embed::RustEmbed)]
#[folder = "images/"]
struct Asset;

pub struct ImageRegistry {
    images: HashMap<String, image::Handle>,
}

impl ImageRegistry {
    pub fn new() -> Self {
        let mut images = HashMap::new();

        for file_path in Asset::iter() {
            if let Some(f) = Asset::get(&file_path) {
                // get filename
                let key = Path::new(file_path.as_ref())
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(file_path.as_ref())
                    .to_string();

                let handle = image::Handle::from_bytes(f.data.into_owned());

                images.insert(key, handle);
            }
        }

        Self { images }
    }
    pub fn get(&self, key: &str) -> Option<image::Handle> {
        self.images.get(key).cloned()
    }
}
