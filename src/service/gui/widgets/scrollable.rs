use std::marker::PhantomData;

use iced::{
    Element, Length, Theme,
    widget::{Column, Scrollable, column, responsive, scrollable, space},
};

use crate::service::gui::styling::{AppTheme, ContainerStyle, ScrollableStyle};

// 1. Hold the slice directly instead of the generic L
pub struct VirtualScrollable<'a, Message, Item, F>
where
    F: Fn(usize, &'a Item, &'a Theme) -> Element<'a, Message>,
{
    items: &'a [Item], // Changed from L to &'a [Item]
    item_height: f32,
    viewport_height: f32,
    scroll_offset: f32,
    theme: &'a Theme,
    render_item: F,
    spacing: f32,
    _phantom: PhantomData<Message>,
}

impl<'a, Message: 'a, Item: 'a, F> VirtualScrollable<'a, Message, Item, F>
where
    F: Fn(usize, &'a Item, &'a Theme) -> Element<'a, Message>,
{
    pub fn new(
        items: &'a [Item], // Accepts the slice
        item_height: f32,
        viewport_height: f32,
        scroll_offset: f32,
        theme: &'a Theme,
        render_item: F,
        spacing: f32,
    ) -> Self {
        Self {
            items,
            item_height,
            viewport_height,
            scroll_offset,
            theme,
            render_item,
            _phantom: PhantomData,
            spacing,
        }
    }

    pub fn build(self) -> Column<'a, Message> {
        let total_items = self.items.len();

        let items_per_screen = (self.viewport_height / self.item_height).ceil() as usize;
        let visible_count = items_per_screen + 2; // Buffer for smooth scrolling
        let raw_start_index = (self.scroll_offset / self.item_height).floor() as usize;
        let start_index = raw_start_index.min(total_items.saturating_sub(1));
        let end_index = (start_index + visible_count).min(total_items);

        let top_spacer = start_index as f32 * self.item_height;
        let bottom_spacer = (total_items.saturating_sub(end_index)) as f32 * self.item_height;

        // Now self.items[range] returns items with lifetime 'a
        // because self.items IS &'a [Item]
        let visible_items = column(self.items[start_index..end_index].iter().enumerate().map(
            |(ri, item)| {
                let i = start_index + ri;
                (self.render_item)(i, item, self.theme)
            },
        ))
        .width(Length::Fill)
        .spacing(self.spacing);

        column![
            space().height(top_spacer),
            visible_items,
            space().height(bottom_spacer),
        ]
        .width(Length::Fill)
    }
}

pub fn virtualized_vertical_scrollable<'a, Message, Item, F, S>(
    items: &'a [Item], // Take the slice directly
    item_height: f32,
    scroll_offset: f32,
    theme: &'a Theme,
    render_item: F,
    style: ScrollableStyle,
    container_style: ContainerStyle,
    on_scroll: S,
    spacing: f32,
    modify_scrollable: impl Fn(
        iced::widget::Scrollable<'a, Message>,
    ) -> iced::widget::Scrollable<'a, Message>
    + 'a,
) -> Element<'a, Message>
where
    Message: 'a,
    Item: 'a,
    F: Fn(usize, &'a Item, &'a Theme) -> Element<'a, Message> + 'a + Clone,
    S: Fn(iced::widget::scrollable::Viewport) -> Message + 'a + Clone,
{
    responsive(move |size| {
        let content = VirtualScrollable::new(
            items, // items is a slice reference, which is Copy
            item_height,
            size.height,
            scroll_offset,
            theme,
            render_item.clone(),
            spacing,
        )
        .build();

        modify_scrollable(
            scrollable(content)
                .height(Length::Fill)
                .width(Length::Fill)
                .on_scroll(on_scroll.clone())
                .style(style.style(container_style))
                .spacing(0),
        )
        .into()
    })
    .into()
}

pub fn default_scrollable<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    container_style: ContainerStyle,
    theme: &Theme,
) -> Scrollable<'a, Message> {
    let style = theme.stylesheet().default_scrollable();
    scrollable(content).style(style.style(container_style))
}
