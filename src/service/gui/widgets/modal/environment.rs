use std::path::PathBuf;

use directories::UserDirs;
use futures::FutureExt;
use iced::widget::{column, container, markdown, row, space};
use iced::{Alignment, Element, Length, Task};
use rfd::AsyncFileDialog;

use crate::service::gui::enums::PathPythonState;
use crate::service::gui::widgets::markdown::default_markdown;
use crate::service::gui::widgets::modal::ModalMessage;
use crate::service::gui::{
    App,
    message::Message,
    widgets::{
        button::{default_text_button, secondary_text_button},
        modal::{
            AbstractModal,
            AbstractModalMessage::{Global, Local},
            Modal,
        },
        text::{default_text, title_text},
        text_input::default_text_input,
    },
};
use crate::service::gui::{styling, util};

#[derive(Debug, Clone)]
pub enum EnvMessage {
    Confirm,
    PathTextEdit(String),
    NameTextEdit(String),
    // bool: success or not
    PathValidated(bool),
    NameValidated(bool),
    SetupDone(bool),
    PathPicked(Option<PathBuf>),
    PickPath,
    LinkClicked(markdown::Uri),
}
impl Into<ModalMessage> for EnvMessage {
    fn into(self) -> ModalMessage {
        ModalMessage::Environment(self)
    }
}

#[derive(Debug, Clone)]
enum LoadingState {
    NotLoading,
    Loading,
    Done,
    Error,
}
#[derive(Debug, Clone)]
enum PathExistState {
    Exists,
    Invalid,
    Loading,
}
impl PathExistState {
    pub fn from_bool(b: bool) -> Self {
        if b { Self::Exists } else { Self::Invalid }
    }
}

#[derive(Debug, Clone)]
pub struct EnvModal {
    file_path: PathBuf,
    name: String,
    loading_state: LoadingState,
    path_exist_state: PathExistState,
    name_exist_state: PathExistState,
    status_markdown: Option<Vec<markdown::Item>>,
    has_python: bool,
    continue_ready: bool,
}
impl EnvModal {
    pub fn new() -> Self {
        Self {
            file_path: PathBuf::new(),
            name: String::new(),
            loading_state: LoadingState::NotLoading,
            path_exist_state: PathExistState::Invalid,
            name_exist_state: PathExistState::Invalid,
            status_markdown: None,
            has_python: true,
            continue_ready: false,
        }
    }
}
impl Into<Modal> for EnvModal {
    fn into(self) -> Modal {
        Modal::Environment(self)
    }
}

impl AbstractModal<Message> for EnvModal {
    type ModalMsg = EnvMessage;
    type App = App;

    fn view(
        &self,
        app: &Self::App,
        theme: &iced::Theme,
    ) -> iced::Element<'_, super::AbstractModalMessage<Self::ModalMsg, Message>> {
        let title = title_text("Create Virtual Environment", theme, true, true);
        let file_path = self.file_path.to_string_lossy();
        let info_text = default_text(
            "Virtual environments allow you to manage Python projects \
        (like assignments) while ensuring your packages actually work! To start, please \
        click the 'Select' button below to pick the folder where you want to have your project. \
        If you're unsure, a good place would be on your Desktop or Documents.",
            theme,
            true,
            true,
        );
        let select_button =
            default_text_button("Select", theme).on_press(Local(EnvMessage::PickPath));
        let cancel_button: Element<_> = match &self.loading_state {
            LoadingState::NotLoading => secondary_text_button("Cancel", theme)
                .on_press(Global(Message::HideModal))
                .into(),
            _ => space().into(),
        };
        let file_box = default_text_input("File path...", &file_path, theme)
            .on_input(|s| Local(EnvMessage::PathTextEdit(s)))
            .on_paste(|s| Local(EnvMessage::PathTextEdit(s)));
        let name_txt = default_text("Project name:", theme, true, true).height(Length::Fill);
        let name_box = default_text_input("Project name...", &self.name, theme)
            .on_input(|s| Local(EnvMessage::NameTextEdit(s)))
            .on_paste(|s| Local(EnvMessage::NameTextEdit(s)))
            .width(200);

        // mgerjigerijgioergoe
        let confirm_text: Element<super::AbstractModalMessage<_, _>> = self
            .status_markdown
            .as_ref()
            .map(|m| default_markdown(m, |s| Local(EnvMessage::LinkClicked(s)), theme))
            .unwrap_or_else(|| space().into());

        let confirm_button = match &self.loading_state {
            LoadingState::NotLoading if self.continue_ready && self.has_python => {
                default_text_button("Confirm", theme).on_press(Local(EnvMessage::Confirm))
            }
            LoadingState::NotLoading => default_text_button("Confirm", theme),
            LoadingState::Loading => default_text_button("Loading...", theme),
            LoadingState::Done | LoadingState::Error => {
                default_text_button("Close", theme).on_press(Global(Message::HideModal))
            }
        };
        container(
            column![
                title,
                info_text,
                confirm_text,
                row![select_button, file_box]
                    .align_y(Alignment::Center)
                    .spacing(10),
                container(
                    row![
                        name_txt,
                        name_box,
                        space().width(Length::Fill),
                        cancel_button,
                        confirm_button
                    ]
                    .align_y(Alignment::Center)
                    .spacing(10)
                )
                .align_y(Alignment::Center),
            ]
            .spacing(20),
        )
        .padding(20)
        .width(Length::Fixed(600.0))
        .into()
    }

    fn update(
        &mut self,
        app: &mut Self::App,
        message: Self::ModalMsg,
    ) -> iced::Task<super::AbstractModalMessage<Self::ModalMsg, Message>> {
        match message {
            EnvMessage::Confirm => {
                self.loading_state = LoadingState::Loading;
                let pv = match &app.data.path_python_version {
                    PathPythonState::Version(v) => v.clone(),
                    _ => return Task::none(),
                };
                let full_path = PathBuf::from(self.file_path.clone()).join(self.name.clone());
                util::setup_project_to_modal(
                    app.communication.process_sender.clone(),
                    full_path,
                    pv,
                )
                .map(Global)
            }
            EnvMessage::NameTextEdit(t) => {
                self.name = t.clone();
                let path_str = self.file_path.clone();
                let path = PathBuf::from(path_str).join(t);
                self.name_exist_state = PathExistState::Loading;

                // update markdown
                status_markdown(self, &app);

                Task::perform(async move { path.exists() }, |exists| {
                    EnvMessage::NameValidated(exists)
                })
                .map(Local)
            }
            EnvMessage::PathTextEdit(t) => {
                self.file_path = PathBuf::from(&t);
                self.path_exist_state = PathExistState::Loading;

                // update markdown
                status_markdown(self, &app);

                Task::perform(async move { std::path::Path::new(&t).exists() }, |exists| {
                    EnvMessage::PathValidated(exists)
                })
                .map(Local)
            }
            EnvMessage::NameValidated(b) => {
                self.name_exist_state = PathExistState::from_bool(b);

                // update markdown
                status_markdown(self, &app);

                Task::none()
            }
            EnvMessage::PathValidated(b) => {
                self.path_exist_state = PathExistState::from_bool(b);

                // update markdown
                status_markdown(self, &app);

                Task::none()
            }
            EnvMessage::SetupDone(b) => {
                if b {
                    self.loading_state = LoadingState::Done
                } else {
                    self.loading_state = LoadingState::Error
                };

                // update markdown
                status_markdown(self, &app);

                Task::none()
            }
            EnvMessage::PickPath => Task::perform(
                {
                    let dir = UserDirs::new()
                        .and_then(|u| u.document_dir().map(|p| p.to_path_buf()))
                        .unwrap_or(PathBuf::from("/"));
                    AsyncFileDialog::new()
                        .set_directory(dir)
                        .pick_folder()
                        .map(|o| o.map(|h| h.path().to_path_buf()))
                },
                |p| Local(EnvMessage::PathPicked(p)),
            ),
            EnvMessage::PathPicked(p) => {
                if let Some(path) = p {
                    self.file_path = path.clone();

                    // update markdown
                    status_markdown(self, &app);

                    Task::perform(async move { path.exists() }, |exists| {
                        EnvMessage::PathValidated(exists)
                    })
                    .map(Local)
                } else {
                    Task::none()
                }
            }
            EnvMessage::LinkClicked(l) => {
                log::info!("clicked: {l:?}");
                Task::none()
            }
        }
    }
}

fn status_markdown(modal: &mut EnvModal, app: &App) {
    let mut has_python = true;
    let mut continue_ready = false;
    let file_path = modal.file_path.to_string_lossy();
    let txt: Option<String> = match (&modal.loading_state, &app.data.path_python_version) {
        (_, PathPythonState::Error | PathPythonState::NotFound | PathPythonState::Unknown) => {
            has_python = false;
            Some(String::from(
                "Error: Python must be installed to create \
                a virtual environment! Please press the **Install Python** button above first.\n\n \
                If you just installed Python using pancakes, please close and reopen pancakes then \
                try again.",
            ))
        }
        (LoadingState::NotLoading, _) => {
            if !file_path.trim().is_empty() && !modal.name.trim().is_empty() {
                match (&modal.path_exist_state, &modal.name_exist_state) {
                    (PathExistState::Exists, PathExistState::Invalid) => {
                        continue_ready = true;
                        Some(format!(
                            "Creating project `{}` within directory `{}`",
                            modal.name.trim(),
                            &file_path,
                        ))
                    }
                    (PathExistState::Invalid, _) => Some(String::from(
                        "The folder path entered does not exist. Please double \
                            check you have the correct path",
                    )),
                    (_, PathExistState::Exists) => Some(String::from(
                        "The project name provided already exists as a folder. \
                            Please provide a name that isn't taken yet!",
                    )),
                    _ => None,
                }
            } else {
                None
            }
        }
        (LoadingState::Loading, _) => Some(String::from("Loading environment...")),
        (LoadingState::Done, _) => Some(String::from("Creation complete! Press Close to finish.")),
        (LoadingState::Error, _) => Some(String::from(
            "An interal error occured. This is most likely due to the folder location being malformed \
                or not existing. \
                To try again, please click 'Close' and ensure the path you entered is valid.\n\n \
                **Important:** if you have not yet installed uv with the **Install Python** button above, make \
                to do that first.",
        )),
    };

    // assign values
    modal.has_python = has_python;
    modal.continue_ready = continue_ready;

    let mkdn: Option<Vec<_>> = txt.map(|t| markdown::parse(&t).collect());
    modal.status_markdown = mkdn;
}
