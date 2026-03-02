use std::path::PathBuf;

use anyhow::Result;
use tokio::sync::oneshot;

pub enum FileMessage {
    GetCacheDir {
        response: oneshot::Sender<PathBuf>,
    },
    GetDataDir {
        response: oneshot::Sender<PathBuf>,
    },
    SaveString {
        response: oneshot::Sender<Result<()>>,
        directory: Directory,
        data: String,
    },
    LoadString {
        response: oneshot::Sender<Result<String>>,
        directory: Directory,
    },
}

pub enum Directory {
    Cache(String),
    Data(String),
}
