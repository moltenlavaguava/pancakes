use iced::{
    Alignment, Element, Length, Padding, Theme,
    widget::{column, container, markdown, space},
};

use crate::service::gui::{
    structs::ImageRegistry,
    styling::{AppTheme, MarkdownCodeContainer},
    widgets::text::default_text,
};

pub fn default_markdown<'a, Message: 'a>(
    items: &'a [markdown::Item],
    on_interaction: impl Fn(String) -> Message + 'a,
    theme: &Theme,
) -> Element<'a, Message> {
    let settings = theme.stylesheet().default_markdown();
    markdown::view(items, settings).map(on_interaction)
}

// pub fn markdown_with_images<'a, Message: 'a>(
//     items: &'a [markdown::Item],
//     on_interaction: impl Fn(String) -> Message + 'a,
//     image_registry: &ImageRegistry,
//     theme: &Theme,
// ) -> Element<'a, Message> {
//     let settings = theme.stylesheet().default_markdown();
//     let mut col = column![].spacing(settings.spacing).width(Length::Fill);

//     for item in items {
//         match item {
//             markdown::Item::Image {
//                 url,
//                 title: _,
//                 alt: _,
//             } => {
//                 // find image in registry
//                 if let Some(handle) = image_registry.get(url) {
//                     col = col.push(
//                         container(
//                             iced::widget::image(handle)
//                                 .height(300)
//                                 .content_fit(iced::ContentFit::Contain),
//                         )
//                         .width(Length::Fill)
//                         .align_x(Alignment::Center),
//                     )
//                 } else {
//                     col = col.push(default_text(
//                         format!("[image not found: {}]", url),
//                         theme,
//                         true,
//                         true,
//                     ));
//                 }
//             }
//             _ => {
//                 let md = markdown::view(std::slice::from_ref(item), settings);
//                 col = col.push(container(md).width(Length::Fill));
//             }
//         }
//     }
//     Element::from(col).map(on_interaction)
// }
pub fn markdown_with_images<'a, Message: 'a>(
    items: &'a [markdown::Item],
    on_interaction: impl Fn(String) -> Message + 'a,
    image_registry: &ImageRegistry,
    theme: &Theme,
) -> Element<'a, Message> {
    let settings = theme.stylesheet().default_markdown();
    let mut col = column![].spacing(settings.spacing).width(Length::Fill);

    for item in items {
        match item {
            markdown::Item::Image { url, .. } => {
                if let Some(handle) = image_registry.get(url) {
                    col = col.push(
                        container(
                            iced::widget::image(handle)
                                .height(300)
                                .content_fit(iced::ContentFit::Contain),
                        )
                        .width(Length::Fill)
                        .padding(10)
                        .align_x(Alignment::Center),
                    );
                } else {
                    col = col.push(default_text(
                        format!("[image not found: {}]", url),
                        theme,
                        true,
                        true,
                    ));
                }
            }
            markdown::Item::CodeBlock { code, .. } => {
                col = col.push(
                    container(
                        default_text(code.trim(), theme, true, true)
                            .font(settings.style.code_block_font)
                            .color(settings.style.inline_code_color)
                            .size(settings.code_size)
                            .line_height(1.5),
                    )
                    .padding(8)
                    .style(settings.to_container_style().style())
                    .width(Length::Fill),
                );
            }
            _ => {
                let md = markdown::view(std::slice::from_ref(item), settings);

                col = col.push(md);
            }
        }
    }

    col = col.push(iced::widget::space().width(60));
    Element::from(col).map(on_interaction)
}
