use iced::{
    Element, Length, Task,
    widget::{column, container, row, space},
};

use crate::service::{
    gui::{
        App,
        enums::PathPythonState,
        message::Message,
        util,
        widgets::{
            button::{default_text_button, secondary_text_button},
            modal::{
                AbstractModal,
                AbstractModalMessage::{Global, Local},
                Modal,
            },
            text::{default_text, title_text},
        },
    },
    process::{enums::UVVerifyResult, structs::CurrentReleaseData},
};

#[derive(Debug, Clone)]
pub enum InstallModalMsg {
    ToVersionsPage,
    ToConfirmPage,
    UVStatusReceived(UVStatus),
    ReleaseDataReceived(CurrentReleaseData),
    InstallPython,
    InstallUV,
    // bool: whether or not this installation was successful
    PythonInstalled(bool),
    UVInstalled,
}

#[derive(Debug, Clone)]
pub struct InstallModal {
    page: Page,
    uv_status: UVStatus,
    release_data: Option<CurrentReleaseData>,
    install_state: InstallState,
    uv_install_state: UVInstallState,
}

#[derive(Debug, Clone)]
enum Page {
    UVVerify,
    Versions,
    Confirm,
}
#[derive(Debug, Clone)]
enum InstallState {
    NotStarted,
    Working,
    Completed,
    Error,
}
#[derive(Debug, Clone)]
enum UVInstallState {
    Idle,
    Loading,
    Complete,
}

#[derive(Debug, Clone)]
pub enum UVStatus {
    NotChecked,
    Loading,
    Result(UVVerifyResult),
}
impl From<UVVerifyResult> for UVStatus {
    fn from(value: UVVerifyResult) -> Self {
        UVStatus::Result(value)
    }
}

// install steps:
//  1. ensure uv is installed -- skip if present
//  2. check installed versions via uv and show installing version
//  3. confirm uv command + install python via uv
//      - uv should handle pathing and all that
impl AbstractModal<Message> for InstallModal {
    type ModalMsg = InstallModalMsg;
    type App = App;

    fn view(
        &self,
        app: &Self::App,
        theme: &iced::Theme,
    ) -> iced::Element<'_, super::AbstractModalMessage<Self::ModalMsg, Message>> {
        let m: Element<_> = match &self.page {
            Page::UVVerify => {
                let title = title_text("Step 1: External Dependencies", theme, true, true);
                let (content, next_button) = match &self.uv_status {
                    UVStatus::NotChecked | UVStatus::Loading => {
                        let content =
                            default_text("Checking to see if uv is installed..", theme, true, true);
                        let next_button = default_text_button("Next", theme);
                        (content, next_button)
                    }
                    UVStatus::Result(r) => match (r, &self.uv_install_state) {
                        (
                            UVVerifyResult::Error | UVVerifyResult::NotFound,
                            UVInstallState::Complete,
                        ) => {
                            let content = default_text(
                                "Installation failed. This is most likely due to not having an internet connection. \
                                To try again, press the Install button below.",
                                theme,
                                true,
                                true,
                            );
                            let next_button = default_text_button("Install", theme)
                                .on_press(Local(InstallModalMsg::InstallUV));
                            (content, next_button)
                        }
                        (_, UVInstallState::Complete) => {
                            let content = default_text(
                                "Installation complete! Press Next to continue.",
                                theme,
                                true,
                                true,
                            );
                            let next_button = default_text_button("Next", theme)
                                .on_press(Local(InstallModalMsg::ToVersionsPage));
                            (content, next_button)
                        }
                        (UVVerifyResult::Ok, _) => {
                            let content = default_text(
                                "uv is installed! Press Next to continue.",
                                theme,
                                true,
                                true,
                            );
                            let next_button = default_text_button("Next", theme)
                                .on_press(Local(InstallModalMsg::ToVersionsPage));
                            (content, next_button)
                        }
                        (_, UVInstallState::Loading) => {
                            let content = default_text(
                                "Installing uv... (this should not take more than a minute)",
                                theme,
                                true,
                                true,
                            );
                            let next_button = default_text_button("Next", theme);
                            (content, next_button)
                        }
                        (UVVerifyResult::Error | UVVerifyResult::NotFound, _) => {
                            let content = default_text(
                                "uv not found. uv is a custom Python installation \
                                manager and is the core depdendency that pancakes uses. To continue installing Python, \
                                please select Next below to automatically install uv to your system. \
                                Note: uv is quite small, battle tested, and ~10-100x faster than standard Python tools \
                                like pip, but pancakes will install the standard pip too if you prefer. \
                                If you do not wish to install uv to your system, pancakes will be unable to install \
                                Python, but can still provide all the guides listed below.",
                                theme,
                                true,
                                true,
                            );
                            let next_button = default_text_button("Install", theme)
                                .on_press(Local(InstallModalMsg::InstallUV));
                            (content, next_button)
                        }
                    },
                };
                let cancel_button =
                    secondary_text_button("Cancel", theme).on_press(Global(Message::HideModal));
                let lower_buttons = row![space().width(Length::Fill), cancel_button, next_button];

                column![title, content, lower_buttons].into()
            }
            Page::Versions => {
                let title = title_text("Step 2: Versioning", theme, true, true);
                let (content, next_button) = match &self.release_data {
                    Some(d) => {
                        let same_version = matches!(
                            (&app.data.path_python_version, &d.latest_release),
                            (PathPythonState::Version(c), Some(v)) if c == v
                        );
                        let mut content = column![
                            default_text(
                                format!(
                                    "Current Python version: {}",
                                    match &app.data.path_python_version {
                                        PathPythonState::Error =>
                                            String::from("error (this is a bug)"),
                                        PathPythonState::NotFound => String::from("none found"),
                                        PathPythonState::Unknown => String::from("loading..."),
                                        PathPythonState::Version(v) => v.to_string(),
                                    }
                                ),
                                theme,
                                true,
                                true
                            ),
                            default_text(
                                format!(
                                    "Installing version: {}",
                                    match &d.latest_release {
                                        Some(v) => v.to_string(),
                                        None => String::from(
                                            "none..? (This is a bug, please report it)"
                                        ),
                                    }
                                ),
                                theme,
                                true,
                                true
                            )
                        ];

                        if same_version {
                            content = content.push(default_text(
                                "Note: the installing version is the same \
                            as the current version. You can still continue just fine, but \
                            it is likely nothing will change.",
                                theme,
                                true,
                                true,
                            ))
                        }

                        let next_button = default_text_button("Next", theme)
                            .on_press(Local(InstallModalMsg::ToConfirmPage));
                        let content: Element<_> = content.into();
                        (content, next_button)
                    }
                    None => {
                        let content =
                            default_text("Loading current Python versions..", theme, true, true)
                                .into();
                        let next_button = default_text_button("Next", theme);

                        (content, next_button)
                    }
                };
                let cancel_button =
                    secondary_text_button("Cancel", theme).on_press(Global(Message::HideModal));
                let lower_buttons = row![space().width(Length::Fill), cancel_button, next_button];

                column![title, content, lower_buttons].into()
            }
            Page::Confirm => {
                let title = title_text("Step 3: Confirm", theme, true, true);
                let lower_content: Element<_> = match &self.release_data {
                    Some(d) => {
                        let content: Element<_> = match &self.install_state {
                            InstallState::NotStarted => {
                                let (display_version, cmd_version) = match &d.latest_release {
                                    Some(r) => (r.to_string(), r.to_string()),
                                    None => (
                                        String::from("none..? (This is a bug, please report it)"),
                                        String::from("<version>"),
                                    ),
                                };

                                column![
                                    default_text(
                                        format!("Installing Python {}:", display_version),
                                        theme,
                                        true,
                                        true
                                    ),
                                    default_text(
                                        format!(
                                            "To do this, the following command will be done:\n\
                                    'uv python install {} -r --default --preview-features python-install-default'", cmd_version
                                        ),
                                        theme,
                                        true,
                                        true
                                    )
                                ]
                                .into()
                            }
                            InstallState::Working => {
                                default_text("Downloading python..", theme, true, true).into()
                            }
                            InstallState::Completed => {
                                column![
                                    default_text("Downloading complete!", theme, true, true),
                                default_text("Please keep in mind the following:\n\
                                1. Due to system limitations, every program (such as pancakes, VS Code, or terminal windows) \
                                needs to be restarted to see newly installed Python versions.\n\
                                2. When installing Python, pancakes does not add pip to the system PATH. This means you will \
                                need to run python -m pip instead of pip anytime you want to use the command. \
                                However, for many reasons, it is recommended to never use this command by itself, rather using \
                                uv and virtual environments. To learn more, please read the guides :D.", theme, true, true),
                                ].into()
                            }
                            InstallState::Error => default_text(
                                "An error occured while downloading",
                                theme,
                                true,
                                true,
                            )
                            .into(),
                        };
                        let next_button: Element<_> = match &self.install_state {
                            InstallState::NotStarted => default_text_button("Start", theme)
                                .on_press(Local(InstallModalMsg::InstallPython))
                                .into(),
                            InstallState::Working => default_text_button("Start", theme).into(),
                            InstallState::Completed => space().into(),
                            InstallState::Error => default_text_button("Retry", theme)
                                .on_press(Local(InstallModalMsg::InstallPython))
                                .into(),
                        };
                        let cancel_button = match &self.install_state {
                            InstallState::Completed => default_text_button("Close", theme)
                                .on_press(Global(Message::HideModal)),
                            _ => secondary_text_button("Cancel", theme)
                                .on_press(Global(Message::HideModal)),
                        };
                        let lower_buttons =
                            row![space().width(Length::Fill), cancel_button, next_button];
                        column![content, lower_buttons].into()
                    }
                    None => {
                        let content = default_text(
                            "Somehow, there is no version data avaliable. This is a bug, please report this!",
                            theme,
                            true,
                            true,
                        );

                        let lower_buttons = row![
                            space().width(Length::Fill),
                            secondary_text_button("Cancel", theme)
                        ];
                        column![content, lower_buttons].into()
                    }
                };

                column![title, lower_content].into()
            }
        };
        container(m).width(Length::Fixed(400.0)).into()
    }

    fn update(
        &mut self,
        app: &mut Self::App,
        message: Self::ModalMsg,
    ) -> Task<super::AbstractModalMessage<Self::ModalMsg, Message>> {
        match message {
            InstallModalMsg::ToConfirmPage => {
                self.page = Page::Confirm;
                Task::none()
            }
            InstallModalMsg::ToVersionsPage => {
                self.page = Page::Versions;
                // also request version data
                util::current_release_data_to_modal(app.communication.process_sender.clone())
                    .map(Global)
            }
            InstallModalMsg::ReleaseDataReceived(d) => {
                self.release_data = Some(d);
                Task::none()
            }
            InstallModalMsg::UVStatusReceived(s) => {
                self.uv_status = s;
                Task::none()
            }
            InstallModalMsg::InstallPython => {
                // install python via util
                self.install_state = InstallState::Working;
                util::install_python_to_modal(
                    app.communication.process_sender.clone(),
                    self.release_data
                        .as_ref()
                        .expect("Expected to have CurrentReleaseData provided")
                        .latest_release
                        .as_ref()
                        .expect("Expected to have latest version provided")
                        .clone(),
                )
                .map(Global)
            }
            InstallModalMsg::PythonInstalled(s) => {
                // do something here
                if s {
                    self.install_state = InstallState::Completed
                } else {
                    self.install_state = InstallState::Error
                }
                Task::done(Global(Message::RestartNeeded))
            }
            InstallModalMsg::InstallUV => {
                self.uv_install_state = UVInstallState::Loading;
                util::install_uv_to_modal(app.communication.process_sender.clone())
                    .map(Global)
                    .chain(Task::done(Local(InstallModalMsg::UVInstalled)))
            }
            InstallModalMsg::UVInstalled => {
                self.uv_install_state = UVInstallState::Complete;
                Task::none()
            }
        }
    }
    // fn fill_height(&self) -> super::ModalFillAmount {
    //     super::ModalFillAmount::Offset(40)
    // }
}
impl Into<Modal> for InstallModal {
    fn into(self) -> Modal {
        Modal::Install(self)
    }
}
impl InstallModal {
    pub fn new() -> Self {
        Self {
            page: Page::UVVerify,
            uv_status: UVStatus::NotChecked,
            release_data: None,
            install_state: InstallState::NotStarted,
            uv_install_state: UVInstallState::Idle,
        }
    }
}
