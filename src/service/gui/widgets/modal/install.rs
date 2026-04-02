use iced::{
    Length, Task,
    widget::{column, row, space},
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
    process::enums::UVVerifyResult,
};

#[derive(Debug, Clone)]
pub enum InstallModalMsg {
    ToVersionsPage,
    ToConfirmPage,
}

#[derive(Debug, Clone)]
pub struct InstallModal {
    page: Page,
    uv_status: UVStatus,
}

#[derive(Debug, Clone)]
enum Page {
    UVVerify,
    Versions,
    Confirm,
}

#[derive(Debug, Clone)]
enum UVStatus {
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
        match &self.page {
            Page::UVVerify => {
                let title = title_text("Step 1: External Dependencies", theme, true, true);
                let (content, next_button) = match self.uv_status {
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
            Page::Versions => {}
        }
    }

    fn update(
        &mut self,
        app: &mut Self::App,
        message: Self::ModalMsg,
    ) -> Task<super::AbstractModalMessage<Self::ModalMsg, Message>> {
        todo!()
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
        }
    }
}
