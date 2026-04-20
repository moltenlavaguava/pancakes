use std::sync::OnceLock;

use crate::service::gui::styling::AppTheme;
use crate::service::gui::update::dev::DevMessage;
use crate::service::gui::widgets::button::default_button;
use crate::service::gui::widgets::container::menu_content;
use crate::service::gui::widgets::markdown::default_markdown;
use crate::service::gui::widgets::rule::default_horizontal_rule;
use crate::service::gui::widgets::scrollable::{
    default_scrollable, virtualized_vertical_scrollable,
};
use crate::service::gui::{
    App, icons,
    message::Message,
    widgets::{
        button::invisible_button,
        text::{default_text, icon_text, title_text},
    },
};
use iced::widget::{column, container, markdown, row};
use iced::{Alignment, Element, Font, Length, Padding, Theme};

pub fn view<'a>(app: &'a App, theme: &'a Theme) -> Element<'a, Message> {
    // build the dev page
    let back_button = {
        let ts = theme.stylesheet().title_text(true, true);
        invisible_button(icon_text(icons::LEFT_ARROW, ts), theme).on_press(Message::Home)
    };
    let title_text = title_text("Debug - Program Log", theme, true, true).height(Length::Shrink);
    let header = container(column![
        row![
            back_button,
            title_text,
            icon_text(icons::WRENCH, theme.stylesheet().title_text(true, true))
        ]
        .spacing(10)
        .height(Length::Shrink)
        .align_y(Alignment::Center),
        default_horizontal_rule(4, theme),
    ])
    .height(Length::Shrink);

    let info_mkdn = default_markdown(&app.data.dev_data.info_markdown, Message::Link, theme);
    let log_header = row![
        container(info_mkdn).width(Length::Fill),
        default_button(
            icon_text(icons::COPY, theme.stylesheet().default_text(true, true)),
            theme
        )
        .width(Length::Shrink)
        .on_press(DevMessage::CopyLog.into())
    ]
    .spacing(20);

    // TODO: make this scrollable virtual
    // let item_height = theme.stylesheet().base_text_size;
    // let log_scroll = virtualized_vertical_scrollable(
    //     &app.logs,
    //     30.0,
    //     app.data.dev_data.log_scroll_offset,
    //     theme,
    //     |_, lt, theme| {
    //         default_text(lt, theme, true, true)
    //             .font(Font::MONOSPACE)
    //             .into()
    //     },
    //     theme.stylesheet().default_scrollable(),
    //     theme.stylesheet().main_content(),
    //     |v| DevMessage::LogScroll(v.absolute_offset().y).into(),
    //     0.0,
    //     |s| s,
    // );
    let log_scroll = default_scrollable(
        column(app.logs.iter().rev().map(|ls| {
            default_text(ls, theme, true, true)
                .font(Font::MONOSPACE)
                .into()
        })),
        theme.stylesheet().home_widget_container(),
        theme,
    )
    .width(Length::Fill);

    menu_content(
        column![header, log_header, log_scroll]
            .spacing(10)
            .height(Length::Fill),
        theme,
    )
    .padding(Padding::new(12.0).top(6))
    .into()
}
