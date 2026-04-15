use iced::widget::{column, container, space};
use iced::{Element, widget::row};
use iced::{Length, Padding};

use crate::service::gui::enums::PathPythonState;
use crate::service::gui::styling::AppTheme;
use crate::service::gui::update::guide::GuideMessage;
use crate::service::gui::update::install::InstallMessage;
use crate::service::gui::widgets::button::{default_text_button, guide_button};
use crate::service::gui::widgets::container::{home_menu_widget_container, menu_content};
use crate::service::gui::widgets::notification::NotificationRenderData;
use crate::service::gui::widgets::page::build_page;
use crate::service::gui::widgets::rule::default_horizontal_rule;
use crate::service::gui::widgets::scrollable::default_scrollable;
use crate::service::gui::widgets::text::{default_text, title_text};
use crate::service::gui::widgets::text_input::default_text_input;
use crate::service::gui::{App, message::Message};

pub fn view<'a>(app: &'a App) -> Element<'a, Message> {
    let theme = &app.theme();
    let header = row![title_text("Pancakes", theme, true, true),];
    let top_rule = default_horizontal_rule(4, theme);

    let install_content = {
        let title = title_text("Python", theme, true, true);
        let path_python_text = match &app.data.path_python_version {
            PathPythonState::Error => "error (this is a bug)",
            PathPythonState::NotFound => "none found",
            PathPythonState::Unknown => "loading...",
            PathPythonState::Version(v) => &v.to_string(),
        };
        let version_row = row![
            default_text_button("Install Python", theme)
                .on_press(InstallMessage::InstallPython.into()),
            space().width(Length::Fill),
            default_text(
                format!(
                    "Python version: {}{}",
                    path_python_text,
                    if app.data.restart_needed {
                        " (Restart needed)"
                    } else {
                        ""
                    }
                ),
                theme,
                true,
                true
            ),
        ];
        let env_row = row![
            default_text_button("Setup Virtual Enviornment", theme)
                .on_press(InstallMessage::Environment.into()),
            space().width(Length::Fill),
            default_text("", theme, true, true),
        ];
        container(column![title, version_row, env_row].spacing(4))
    };
    let install_widget = home_menu_widget_container(install_content, theme).width(Length::Fill);

    let help_content = {
        let title = title_text("Learning", theme, true, true);
        let header_row = row![
            default_text("All Guides", theme, true, true),
            space().width(Length::Fill),
            default_text_input("Search Guides..", &app.data.learn_data.home_search, theme)
                .on_input(|t| GuideMessage::SearchText(t).into())
                .on_paste(|t| GuideMessage::SearchText(t).into())
        ];
        let guides = default_scrollable(
            column(app.data.guide_registry.guides.iter().map(|(id, guide)| {
                guide_button(default_text(&guide.name, theme, true, true), theme)
                    .on_press(GuideMessage::OpenGuide(*id).into())
                    .width(Length::Fill)
                    .into()
            })),
            theme.stylesheet().main_content(),
            theme,
        )
        .width(Length::Fill)
        .height(Length::Fill);
        container(column![title, header_row, guides].spacing(4))
    };
    let help_widget = home_menu_widget_container(help_content, theme)
        .width(Length::Fill)
        .height(Length::Fill);

    let content = menu_content(
        column![
            container(column![header, top_rule,]).padding(Padding::new(12.0).bottom(0).top(6)),
            container(column![install_widget, help_widget].spacing(12)).padding(12),
        ],
        theme,
    )
    .width(Length::Fill)
    .height(Length::Fill);

    build_page(
        content,
        None,
        NotificationRenderData {
            side_padding: Padding::ZERO,
            spacing: 0.0,
        },
        app.data.modal.as_ref(),
        app,
        theme,
    )
}
