use iced::widget::{column, container, space};
use iced::{Alignment, Length, Padding, Theme};
use iced::{Element, widget::row};

use crate::service::gui::enums::PathPythonState;
use crate::service::gui::icons;
use crate::service::gui::styling::AppTheme;
use crate::service::gui::update::guide::GuideMessage;
use crate::service::gui::update::install::InstallMessage;
use crate::service::gui::widgets::button::{default_text_button, guide_button, invisible_button};
use crate::service::gui::widgets::container::{
    guide_tag_container, home_menu_widget_container, menu_content,
};
use crate::service::gui::widgets::notification::NotificationRenderData;
use crate::service::gui::widgets::page::build_page;
use crate::service::gui::widgets::rule::default_horizontal_rule;
use crate::service::gui::widgets::scrollable::default_scrollable;
use crate::service::gui::widgets::text::{default_text, icon_text, title_text};
use crate::service::gui::widgets::text_input::default_text_input;
use crate::service::gui::{App, message::Message};

pub fn view<'a>(app: &'a App, theme: &'a Theme) -> Element<'a, Message> {
    let header = row![
        title_text("Pancakes", theme, true, true),
        space().width(Length::Fill),
        invisible_button(
            icon_text(icons::WRENCH, theme.stylesheet().title_text(true, true)),
            theme
        )
        .on_press(Message::Dev)
    ];
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
        let title = title_text("Learning - Guides", theme, true, true).height(Length::Fill);
        let header_row = row![
            title,
            space().width(Length::Fill),
            default_text_input("Search Guides..", &app.data.learn_data.home_search, theme)
                .width(200)
                .on_input(|t| GuideMessage::SearchText(t).into())
                .on_paste(|t| GuideMessage::SearchText(t).into())
        ]
        .align_y(Alignment::Center)
        .height(Length::Shrink);
        let guide_col = column(itertools::Itertools::intersperse_with(
            app.data
                .guide_registry
                .guides
                .iter()
                .filter(|(id, _)| app.data.learn_data.search_match_guide_ids.contains(id))
                .map(|(id, guide)| {
                    // create each guide entry
                    const ITEM_SPACING: u32 = 10;
                    let guide_text = default_text(&guide.name, theme, true, true);
                    let tags_data = row(guide.tags.iter().map(|tag| {
                        guide_tag_container(
                            default_text(tag.to_string(), theme, true, true).height(Length::Fill),
                            theme,
                        )
                        .padding(6)
                        .width(Length::Shrink)
                        .into()
                    }))
                    .align_y(Alignment::Center)
                    .spacing(ITEM_SPACING);
                    let mut content = row![guide_text, space().width(Length::Fill), tags_data]
                        .spacing(ITEM_SPACING);
                    // add on pin symbol if needed
                    if guide.pinned {
                        content = content.push(icon_text(
                            icons::PIN,
                            theme.stylesheet().default_text(true, true),
                        ));
                    }

                    guide_button(content, theme)
                        .on_press(GuideMessage::OpenGuide(*id).into())
                        .width(Length::Fill)
                        .into()
                }),
            || {
                // add rule in between each element
                default_horizontal_rule(2.0, theme).into()
            },
        ))
        .spacing(6);

        let guides = default_scrollable(
            container(guide_col),
            theme.stylesheet().main_content(),
            theme,
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(12);
        container(column![header_row, guides].spacing(4))
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
