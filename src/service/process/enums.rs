use std::path::PathBuf;
use std::{ffi::OsString, process::ExitStatus};

use anyhow::anyhow;

use anyhow::Result;
use pep440_rs::Version;
use tokio::sync::{mpsc, oneshot};

use crate::service::process::structs::CurrentReleaseData;

pub enum ProcessMessage {
    SpawnProcess {
        cmd: OsString,
        args: Vec<OsString>,
        output_stream: mpsc::Sender<ChildMessage>,
        spawn_result: oneshot::Sender<Result<()>>,
    },
    // Check to see if uv even exists
    VerifyUV {
        result_sender: oneshot::Sender<UVVerifyResult>,
    },
    InstallUV {
        result_sender: oneshot::Sender<Result<()>>,
    },
    UVCurrentReleaseData {
        result_sender: oneshot::Sender<Result<CurrentReleaseData>>,
    },
    PathPythonVersion {
        result_sender: oneshot::Sender<Result<Option<Version>>>,
    },
    InstallPython {
        version: Version,
        result_sender: oneshot::Sender<Result<()>>,
    },
    SetupProject {
        path: PathBuf,
        version: Version,
        result_sender: oneshot::Sender<Result<()>>,
    },
}

#[derive(Debug, Clone)]
pub enum ChildMessage {
    StdOut(String),
    StdErr(String),
    Exit(ExitStatus),
}

// Version of child message that doesn't have the exit variant
#[derive(Debug, Clone)]
pub enum ChildOutMessage {
    StdOut(String),
    StdErr(String),
}
impl ChildOutMessage {
    pub fn text(&self) -> &str {
        match self {
            ChildOutMessage::StdOut(t) => &t,
            ChildOutMessage::StdErr(t) => &t,
        }
    }
}

// Contains output data of program and its exit status
#[derive(Debug, Clone)]
pub struct CompletedProgram {
    pub messages: Vec<ChildOutMessage>,
    pub exit_status: ExitStatus,
}
impl TryFrom<Vec<ChildMessage>> for CompletedProgram {
    type Error = anyhow::Error;

    fn try_from(mut value: Vec<ChildMessage>) -> std::result::Result<Self, Self::Error> {
        // Last message should be an exit status
        let Ok(ChildMessage::Exit(exit_status)) = value.pop().ok_or("Vec must not be empty") else {
            return Err(anyhow!("Last ChildMessage must be an ExitStatus"));
        };
        let messages: Vec<ChildOutMessage> = value
            .into_iter()
            .map(|m| match m {
                ChildMessage::StdOut(s) => Ok(ChildOutMessage::StdOut(s)),
                ChildMessage::StdErr(s) => Ok(ChildOutMessage::StdErr(s)),
                ChildMessage::Exit(_) => {
                    Err(anyhow!("Only the last ChildMessage in the vec can be Exit",))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            messages: messages,
            exit_status,
        })
    }
}
impl CompletedProgram {
    pub fn was_successful(&self) -> bool {
        self.exit_status.success()
    }
}

#[derive(Debug, Clone)]
pub enum UVVerifyResult {
    Ok,
    NotFound,
    Error,
}
