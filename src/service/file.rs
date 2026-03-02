use std::path::PathBuf;

use tokio::{fs, sync::mpsc};

use crate::service::{
    file::enums::{Directory, FileMessage},
    logic::ServiceLogic,
};

pub mod enums;
pub mod structs;
pub mod util;

pub type FileSender = mpsc::Sender<FileMessage>;

pub struct FileService {
    cache_dir: PathBuf,
    data_dir: PathBuf,
}

impl FileService {
    pub fn new() -> Self {
        let (cache_dir, data_dir) = util::setup_directories().expect("Failed to setup directories");
        Self {
            cache_dir,
            data_dir,
        }
    }
}

#[async_trait::async_trait]
impl ServiceLogic<FileMessage> for FileService {
    fn name(&self) -> &'static str {
        "FileService"
    }
    async fn handle_message(&mut self, msg: FileMessage) {
        match msg {
            FileMessage::GetCacheDir { response } => {
                let _ = response.send(self.cache_dir.clone());
            }
            FileMessage::GetDataDir { response } => {
                let _ = response.send(self.data_dir.clone());
            }
            FileMessage::SaveString {
                response,
                directory,
                data,
            } => {
                // create full path
                let p = match directory {
                    Directory::Cache(s) => self.cache_dir.join(s),
                    Directory::Data(s) => self.data_dir.join(s),
                };
                // save the file
                let r = fs::write(p, data).await;
                let _ = response.send(r.map_err(|e| e.into()));
            }
            FileMessage::LoadString {
                response,
                directory,
            } => {
                // create full path
                let p = match directory {
                    Directory::Cache(s) => self.cache_dir.join(s),
                    Directory::Data(s) => self.data_dir.join(s),
                };
                // read the file, if possible
                let r = fs::read_to_string(p).await;
                let _ = response.send(r.map_err(|e| e.into()));
            }
        }
    }
}
