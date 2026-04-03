use iced::{
    Element, Length, Task,
    widget::{column, container, row, space},
};

use crate::service::{
    gui::{
        App,
        message::Message,
        widgets::{
            button::{default_text_button, secondary_text_button},
            modal::{AbstractModal, Modal},
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
}

#[derive(Debug, Clone)]
pub struct InstallModal {
    page: Page,
    uv_status: UVStatus,
    release_data: Option<CurrentReleaseData>,
}

#[derive(Debug, Clone)]
enum Page {
    UVVerify,
    Versions,
    Confirm,
}

#[derive(Debug, Clone)]
pub enum UVStatus {
    NotChecked,
    Loading,
    Result(UVVerifyResult),
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
                    UVStatus::Result(r) => match r {
                        UVVerifyResult::Ok => {
                            let content = default_text(
                                "uv is installed! Press Next to continue.",
                                theme,
                                true,
                                true,
                            );
                            let next_button = default_text_button("Next", theme);
                            (content, next_button)
                        }
                        UVVerifyResult::Error | UVVerifyResult::NotFound => {
                            let content = default_text(
                                r#"uv not found. uv is a custom Python installation
                                manager and is the core depdendency that pancakes uses. To continue installing Python,
                                please select Next below to automatically install uv to your system.
                                Note: uv is quite small, battle tested, and ~10-100x faster than standard Python tools
                                like pip, but pancakes will install the standard pip too if you prefer.
                                If you do not wish to install uv to your system, pancakes will be unable to install
                                Python, but can still provide all the guides listed below."#,
                                theme,
                                true,
                                true,
                            );
                            let next_button = default_text_button("Install", theme);
                            (content, next_button)
                        }
                    },
                };
                let cancel_button = secondary_text_button("Cancel", theme);
                let lower_buttons = row![space().width(Length::Fill), cancel_button, next_button];

                column![title, content, lower_buttons].into()
            }
            Page::Versions => {
                let title = title_text("Step 2: Versioning", theme, true, true);
                let (content, next_button) = match &self.release_data {
                    Some(d) => {
                        let content: Element<_> = column![
                            default_text(
                                format!(
                                    "Current Python version: {}",
                                    match &d.current_version {
                                        Some(v) => v.to_string(),
                                        None => String::from("none found"),
                                    }
                                ),
                                theme,
                                true,
                                true
                            ),
                            default_text(
                                format!(
                                    "Installing version: {}",
                                    match &d.current_version {
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
                        ]
                        .into();
                        let next_button = default_text_button("Next", theme);

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
                let cancel_button = secondary_text_button("Cancel", theme);
                let lower_buttons = row![space().width(Length::Fill), cancel_button, next_button];

                column![title, content, lower_buttons].into()
            }
            Page::Confirm => {
                let title = title_text("Step 3: Confirm", theme, true, true);
                let lower_content: Element<_> = match &self.release_data {
                    Some(d) => {
                        let content = column![default_text(
                            format!(
                                "To install: Python {}",
                                match &d.latest_release {
                                    Some(r) => r.to_string(),
                                    None =>
                                        String::from("none..? (This is a bug, please report it)"),
                                }
                            ),
                            theme,
                            true,
                            true
                        )];

                        let lower_buttons = row![
                            space().width(Length::Fill),
                            secondary_text_button("Cancel", theme),
                            default_text_button("Start", theme)
                        ];
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
                Task::none()
            }
            InstallModalMsg::ReleaseDataReceived(d) => {
                self.release_data = Some(d);
                Task::none()
            }
            InstallModalMsg::UVStatusReceived(s) => {
                self.uv_status = s;
                Task::none()
            }
        }
    }
    fn fill_height(&self) -> super::ModalFillAmount {
        super::ModalFillAmount::Offset(40)
    }
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
        }
    }
}
