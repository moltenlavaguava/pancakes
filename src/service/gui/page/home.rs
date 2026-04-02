use iced::widget::{column, container, space};
use iced::{Element, widget::row};
use iced::{Length, Padding};

use crate::service::gui::update::install::InstallMessage;
use crate::service::gui::update::learn::LearnMessage;
use crate::service::gui::widgets::button::default_text_button;
use crate::service::gui::widgets::container::{home_menu_widget_container, menu_content};
use crate::service::gui::widgets::rule::default_horizontal_rule;
use crate::service::gui::widgets::text::{default_text, title_text};
use crate::service::gui::widgets::text_input::default_text_input;
use crate::service::gui::{App, message::Message};

pub fn view<'a>(app: &'a App) -> Element<'a, Message> {
    let theme = &app.theme();
    let header = row![title_text("Pancakes", theme, true, true),];
    let top_rule = default_horizontal_rule(4, theme);

    let install_content = {
        let title = title_text("Python", theme, true, true);
        let version_row = row![
            default_text_button("Install Python", theme)
                .on_press(InstallMessage::InstallPython.into()),
            space().width(Length::Fill),
            default_text("Python Version: ??", theme, true, true),
        ];
        let test_row = row![
            default_text_button("Test", theme).on_press(InstallMessage::TestPython.into()),
            space().width(Length::Fill),
            default_text("", theme, true, true),
        ];
        container(column![title, version_row, test_row].spacing(4))
    };
    let install_widget = home_menu_widget_container(install_content, theme).width(Length::Fill);

    let help_content = {
        let title = title_text("Learning", theme, true, true);
        let header_row = row![
            default_text("All Guides", theme, true, true),
            space().width(Length::Fill),
            default_text_input("Search Guides..", &app.data.learn_data.home_search, theme)
                .on_input(|t| LearnMessage::SearchText(t).into())
                .on_paste(|t| LearnMessage::SearchText(t).into())
        ];
        container(column![title, header_row].spacing(4))
    };
    let help_widget = home_menu_widget_container(help_content, theme)
        .width(Length::Fill)
        .height(Length::Fill);

    menu_content(
        column![
            container(column![header, top_rule,]).padding(Padding::new(12.0).bottom(0).top(6)),
            container(column![install_widget, help_widget].spacing(12)).padding(12),
        ],
        theme,
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
