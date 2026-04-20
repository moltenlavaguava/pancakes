use iced::{
    Alignment, Element, Length, Padding, Theme,
    widget::{column, container, markdown, row},
    window,
};
use indexmap::IndexMap;
use serde::Deserialize;
use strum::{Display, EnumString};

use crate::service::gui::{
    App, icons,
    message::Message,
    styling::AppTheme,
    update::guide::GuideMessage,
    widgets::{
        button::invisible_button,
        container::{home_menu_widget_container, menu_content},
        markdown::markdown_with_images,
        rule::default_horizontal_rule,
        scrollable::default_scrollable,
        text::{default_text, icon_text, title_text},
    },
};

struct IdCounter {
    current: u32,
}
impl IdCounter {
    pub fn new() -> Self {
        Self { current: 0 }
    }
    pub fn next(&mut self) -> u32 {
        self.current += 1;
        self.current
    }
}

#[derive(Deserialize)]
struct ManifestEntry {
    slug: String,
    name: String,
    pinned: bool,
    tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Guide {
    pub pinned: bool,
    pub name: String,
    pub markdown: Vec<markdown::Item>,
    pub tags: Vec<String>,
}

#[derive(rust_embed::RustEmbed)]
#[folder = "guides/"]
struct GuideAssets;

#[derive(Debug, Clone)]
pub struct GuideRegistry {
    pub guides: IndexMap<u32, Guide>,
}
impl GuideRegistry {
    pub fn new() -> GuideRegistry {
        let mut pinned_guides = IndexMap::new();
        let mut guides = IndexMap::new();
        let mut id_counter = IdCounter::new();
        let manifest = GuideAssets::get("manifest.json").expect("manifest.json is missing");
        let entries: Vec<ManifestEntry> =
            serde_json::from_slice(&manifest.data).expect("Failed to parse manifest.json");

        for entry in entries {
            let md_filename = format!("{}.md", entry.slug);
            let f = GuideAssets::get(&md_filename)
                .unwrap_or_else(|| panic!("Markdown file {} not found", md_filename));
            let content = std::str::from_utf8(&f.data).expect("Markdown file is not valid UTF-8");
            let guide = Guide {
                name: entry.name,
                pinned: entry.pinned,
                tags: entry.tags,
                markdown: markdown::parse(content).collect(),
            };

            let id = id_counter.next();
            if guide.pinned {
                pinned_guides.insert(id, guide);
            } else {
                guides.insert(id, guide);
            }
        }

        // sort both kinds of guides
        let sort_guide = |_k1: &u32, v1: &Guide, _k2: &u32, v2: &Guide| {
            v1.name.to_lowercase().cmp(&v2.name.to_lowercase())
        };
        pinned_guides.sort_unstable_by(sort_guide);
        guides.sort_unstable_by(sort_guide);

        pinned_guides.append(&mut guides);

        GuideRegistry {
            guides: pinned_guides,
        }
    }
}

pub fn view<'a>(
    guide_id: u32,
    app: &'a App,
    theme: &'a Theme,
    window_id: window::Id,
) -> Element<'a, Message> {
    // get the guide from the id
    let Some(guide) = app.data.guide_registry.guides.get(&guide_id) else {
        return default_text(
            "Error: failed to get guide from guide id. \
        Please report this :D",
            theme,
            true,
            true,
        )
        .into();
    };

    // build the guide page
    let back_button = {
        let ts = theme.stylesheet().title_text(true, true);
        invisible_button(icon_text(icons::LEFT_ARROW, ts), theme)
            .on_press(Message::CloseWindow(window_id))
    };
    let title_text = title_text(&guide.name, theme, true, true).height(Length::Shrink);
    let header = container(column![
        row![back_button, title_text]
            .spacing(10)
            .height(Length::Shrink)
            .align_y(Alignment::Center),
        default_horizontal_rule(4, theme),
    ])
    .height(Length::Shrink);
    // .padding(Padding::ZERO.top(6));
    let content = home_menu_widget_container(
        default_scrollable(
            column![markdown_with_images(
                &guide.markdown,
                |s| Message::Link(s),
                &app.data.image_registry,
                theme,
            )]
            .width(Length::Fill)
            .padding(Padding::ZERO.horizontal(16)),
            theme.stylesheet().main_content(),
            theme,
        ),
        theme,
    )
    .width(Length::Fill)
    .height(Length::Fill);

    menu_content(column![header, content].spacing(10), theme)
        .padding(Padding::new(12.0).top(6))
        .into()
}
