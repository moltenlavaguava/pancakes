use std::fmt::Debug;

use iced::{
    Element, Length, Padding, Task, Theme,
    widget::{column, container, mouse_area, opaque, space},
};

use crate::service::gui::{
    App,
    message::Message,
    widgets::{
        container::{default_modal_background_container, default_modal_container},
        modal::{
            environment::{EnvMessage, EnvModal},
            install::{InstallModal, InstallModalMsg},
        },
    },
};

pub mod environment;
pub mod install;

#[derive(Debug, Clone)]
enum AbstractModalMessage<Local, Global> {
    Local(Local),
    Global(Global),
}

trait AbstractModal<Global: 'static>: Into<Modal> {
    type ModalMsg;
    type App;
    // Only creates interior content. Does not make or handle modal mangement / container.
    fn view(
        &self,
        app: &Self::App,
        theme: &Theme,
    ) -> Element<'_, AbstractModalMessage<Self::ModalMsg, Global>>;
    fn update(
        &mut self,
        app: &mut Self::App,
        message: Self::ModalMsg,
    ) -> Task<AbstractModalMessage<Self::ModalMsg, Message>>;
    // Wrapper to create the modal container body (centered)
    fn build(
        &self,
        app: &Self::App,
        theme: &Theme,
    ) -> Element<'_, AbstractModalMessage<Self::ModalMsg, Global>> {
        // make main container opaque to make mouse clicks on modal itself not kill it
        let modal = container(opaque(
            default_modal_container(self.view(app, theme), theme).height(Length::Fill),
        ));
        // customize modal sizing
        let sized_modal: Element<AbstractModalMessage<Self::ModalMsg, Global>> =
            match self.fill_height() {
                ModalFillAmount::Shrink => modal.height(Length::Shrink).into(),
                ModalFillAmount::FillPercentage(p) => {
                    // create spacers on top and bottom
                    let sh = (200 - 2 * p) / 2;
                    column![
                        space().height(Length::FillPortion(sh)),
                        modal.height(Length::FillPortion(2 * p)),
                        space().height(Length::FillPortion(sh))
                    ]
                    .into()
                }
                ModalFillAmount::Offset(o) => {
                    // add padding to top and bottom
                    container(modal).padding(Padding::ZERO.vertical(o)).into()
                }
                ModalFillAmount::Fill => modal.height(Length::Fill).into(),
            };
        default_modal_background_container(sized_modal, theme)
            .center(Length::Fill)
            .into()
    }
    fn fill_height(&self) -> ModalFillAmount {
        ModalFillAmount::Shrink
    }
}

#[derive(Debug, Clone)]
pub enum ModalMessage {
    Install(InstallModalMsg),
    Environment(EnvMessage),
    HideModal,
}
impl From<InstallModalMsg> for ModalMessage {
    fn from(value: InstallModalMsg) -> Self {
        ModalMessage::Install(value)
    }
}
impl Into<Message> for ModalMessage {
    fn into(self) -> Message {
        Message::ModalMessage(self)
    }
}

#[derive(Debug, Clone)]
enum ModalFillAmount {
    Shrink,
    Fill,
    /// Percentage of total width, out of 100
    FillPercentage(u16),
    Offset(u32),
}

#[derive(Debug, Clone)]
pub enum Modal {
    Install(InstallModal),
    Environment(EnvModal),
}
impl Modal {
    pub fn view(&self, app: &App, theme: &Theme) -> Element<'_, Message> {
        let main_modal_content = match self {
            Self::Install(m) => m.build(app, theme).map(|abstract_msg| match abstract_msg {
                AbstractModalMessage::Local(l) => ModalMessage::Install(l).into(),
                AbstractModalMessage::Global(g) => g,
            }),
            Self::Environment(m) => m.build(app, theme).map(|abstract_msg| match abstract_msg {
                AbstractModalMessage::Local(l) => ModalMessage::Environment(l).into(),
                AbstractModalMessage::Global(g) => g,
            }),
        };
        opaque(mouse_area(main_modal_content).on_press(Message::HideModal))
    }
    pub fn update(&mut self, app: &mut App, msg: ModalMessage) -> Task<Message> {
        match (self, msg) {
            (Modal::Install(w), ModalMessage::Install(m)) => w.update(app, m).map(|bm| match bm {
                AbstractModalMessage::Local(l) => ModalMessage::Install(l).into(),
                AbstractModalMessage::Global(g) => g,
            }),
            (Modal::Environment(w), ModalMessage::Environment(m)) => {
                w.update(app, m).map(|bm| match bm {
                    AbstractModalMessage::Local(l) => ModalMessage::Environment(l).into(),
                    AbstractModalMessage::Global(g) => g,
                })
            }
            _ => Task::none(),
        }
    }
}
