// stores all icon data

use iced::Font;

pub const ICON_FONT: Font = Font::with_name("pancakeicons");
pub struct IconChar(pub &'static str);

pub const COPY: IconChar = IconChar("\u{e925}");
pub const LEFT_ARROW: IconChar = IconChar("\u{ea40}");
pub const WRENCH: IconChar = IconChar("\u{e991}");
pub const PIN: IconChar = IconChar("\u{e946}");
