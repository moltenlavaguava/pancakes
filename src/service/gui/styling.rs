use iced::{
    Background, Border, Color, Font, Padding, Pixels, Theme,
    border::Radius,
    color,
    font::Weight,
    padding,
    widget::{
        self,
        markdown::{self, Highlight},
    },
};

const NO_BORDER: Border = Border {
    color: Color::TRANSPARENT,
    radius: Radius {
        bottom_left: 0.0,
        bottom_right: 0.0,
        top_left: 0.0,
        top_right: 0.0,
    },
    width: 0.0,
};

#[derive(Debug, Clone, Copy)]
pub struct TextStyle {
    pub color: Color,
    pub text_size: f32,
    pub wrap: bool,
    pub center_y: bool,
    pub font: Font,
}
impl TextStyle {
    pub fn style(self) -> impl Fn(&Theme) -> widget::text::Style {
        move |_| widget::text::Style {
            color: Some(self.color),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonStatusBackgrounds {
    pub active: Option<Background>,
    pub hovered: Option<Background>,
    pub pressed: Option<Background>,
    pub disabled: Option<Background>,
}
#[derive(Debug, Clone, Copy)]
pub struct ButtonStyle {
    pub status_bgs: ButtonStatusBackgrounds,
    pub padding: Padding,
}
impl ButtonStyle {
    pub fn style(self) -> impl Fn(&Theme, widget::button::Status) -> widget::button::Style {
        move |_, s| {
            use widget::button::Status;
            widget::button::Style {
                background: match s {
                    Status::Active => self.status_bgs.active,
                    Status::Disabled => self.status_bgs.disabled,
                    Status::Hovered => self.status_bgs.hovered,
                    Status::Pressed => self.status_bgs.pressed,
                },
                ..widget::button::Style::default()
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ContainerStyle {
    pub bg_color: Option<Background>,
    pub border: Border,
}
impl ContainerStyle {
    pub fn style(self) -> impl Fn(&Theme) -> widget::container::Style {
        move |_| widget::container::Style {
            background: self.bg_color,
            border: self.border,
            ..Default::default()
        }
    }
}
pub trait MarkdownCodeContainer {
    fn to_container_style(&self) -> ContainerStyle;
}
impl MarkdownCodeContainer for markdown::Settings {
    fn to_container_style(&self) -> ContainerStyle {
        ContainerStyle {
            bg_color: Some(self.style.inline_code_highlight.background),
            border: self.style.inline_code_highlight.border,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScrollableStyle {
    pub v_scroll_rail_bg: Option<Background>,
    pub v_scroll_scroller_bg: Background,
}
impl ScrollableStyle {
    pub fn style(
        self,
        container_style: ContainerStyle,
    ) -> impl Fn(&Theme, widget::scrollable::Status) -> widget::scrollable::Style {
        move |t, s| widget::scrollable::Style {
            container: container_style.style()(t),
            vertical_rail: widget::scrollable::Rail {
                background: self.v_scroll_rail_bg,
                scroller: widget::scrollable::Scroller {
                    background: self.v_scroll_scroller_bg,
                    border: Border::default(),
                },
                border: Border::default(),
            },
            ..widget::scrollable::default(t, s)
        }
    }
}

pub struct SliderStyle {
    uncovered_bg: Background,
    covered_bg: Background,
    handle_bg: Background,
    width: f32,
}
impl SliderStyle {
    pub fn style(self) -> impl Fn(&Theme, widget::slider::Status) -> widget::slider::Style {
        move |_t, _s| widget::slider::Style {
            rail: widget::slider::Rail {
                backgrounds: (self.covered_bg, self.uncovered_bg),
                border: Border::default(),
                width: self.width,
            },
            handle: widget::slider::Handle {
                background: self.handle_bg,
                shape: widget::slider::HandleShape::Circle {
                    radius: (self.width * 1.2).round(),
                },
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }
}

pub struct TextInputStyle {
    bg: Background,
    border_color: Color,
    border_width: f32,
    corner_radius: f32,
    placeholder_text_color: Color,
    text_color: Color,
    selection_color: Color,
}
impl TextInputStyle {
    pub fn style(self) -> impl Fn(&Theme, widget::text_input::Status) -> widget::text_input::Style {
        move |_t, _s| widget::text_input::Style {
            background: self.bg,
            border: Border {
                color: self.border_color,
                width: self.border_width,
                radius: Radius::new(self.corner_radius),
            },
            icon: Color::TRANSPARENT,
            placeholder: self.placeholder_text_color,
            value: self.text_color,
            selection: self.selection_color,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RuleStyle {
    color: Color,
    radius: Radius,
}
impl RuleStyle {
    pub fn style(self) -> impl Fn(&Theme) -> widget::rule::Style {
        move |_t| widget::rule::Style {
            color: self.color,
            radius: self.radius,
            fill_mode: widget::rule::FillMode::Full,
            snap: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProgressBarStyle {
    fill_bg: Background,
    bg: Background,
    border: Border,
}
impl ProgressBarStyle {
    pub fn style(self) -> impl Fn(&Theme) -> widget::progress_bar::Style {
        move |_t| widget::progress_bar::Style {
            background: self.bg,
            bar: self.fill_bg,
            border: self.border,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Stylesheet {
    pub base_text_size: f32,
    pub main_text_color: Color,
    pub error_text_color: Color,
    pub default_font: Font,
    pub secondary_text_color: Color,
    pub secondary_rule_color: Color,
    pub main_content_bg: Background,
    pub secondary_content_bg: Background,
    pub sub_content_bg: Background,
    pub accent_color_bg: Background,
    pub text_input_border_color: Color,
    pub home_menu_widget_border_color: Color,
    pub interactable_bg: Background,
    pub interactable_color: Color,
    pub default_button_status_bgs: ButtonStatusBackgrounds,
    pub secondary_button_status_bgs: ButtonStatusBackgrounds,
    pub guide_button_status_bgs: ButtonStatusBackgrounds,
    pub default_modal_background_alpha: f32,
    pub link_color: Color,
    pub markdown_code_border_color: Color,
}
impl Stylesheet {
    pub fn default_text(&self, wrap_text: bool, center_y: bool) -> TextStyle {
        TextStyle {
            color: self.main_text_color,
            text_size: self.base_text_size,
            wrap: wrap_text,
            center_y,
            font: self.default_font,
        }
    }
    pub fn same_size_secondary_text(&self, wrap_text: bool, center_y: bool) -> TextStyle {
        let mut s = self.secondary_text(wrap_text, center_y);
        s.text_size = self.default_text(wrap_text, center_y).text_size;
        s
    }
    pub fn secondary_text(&self, wrap_text: bool, center_y: bool) -> TextStyle {
        TextStyle {
            color: self.secondary_text_color,
            text_size: self.base_text_size * 0.875,
            wrap: wrap_text,
            center_y,
            font: self.default_font,
        }
    }
    pub fn title_text(&self, wrap_text: bool, center_y: bool) -> TextStyle {
        TextStyle {
            color: self.main_text_color,
            text_size: self.base_text_size * 1.25,
            wrap: wrap_text,
            center_y,
            font: Font {
                weight: Weight::Bold,
                ..self.default_font
            },
        }
    }
    pub fn error_text(&self, wrap_text: bool, center_y: bool) -> TextStyle {
        let mut s = self.default_text(wrap_text, center_y);
        s.color = self.error_text_color;
        s
    }
    pub fn left_menu_bold_text(&self, wrap_text: bool, center_y: bool) -> TextStyle {
        TextStyle {
            color: self.secondary_text_color,
            font: Font {
                weight: Weight::Bold,
                ..self.default_font
            },
            ..self.default_text(wrap_text, center_y)
        }
    }
    pub fn left_menu_sub_text(&self, wrap_text: bool, center_y: bool) -> TextStyle {
        TextStyle {
            color: self.secondary_text_color,
            ..self.secondary_text(wrap_text, center_y)
        }
    }
    pub fn default_button(&self) -> ButtonStyle {
        ButtonStyle {
            status_bgs: self.default_button_status_bgs,
            padding: padding::all(8),
        }
    }
    pub fn secondary_button(&self) -> ButtonStyle {
        ButtonStyle {
            status_bgs: self.secondary_button_status_bgs,
            padding: padding::all(8),
        }
    }
    pub fn invisible_button(&self) -> ButtonStyle {
        ButtonStyle {
            status_bgs: ButtonStatusBackgrounds {
                active: None,
                hovered: None,
                pressed: None,
                disabled: None,
            },
            padding: Padding::ZERO,
        }
    }
    pub fn main_content(&self) -> ContainerStyle {
        ContainerStyle {
            bg_color: Some(self.main_content_bg),
            border: NO_BORDER,
        }
    }
    pub fn menu_content(&self) -> ContainerStyle {
        ContainerStyle {
            bg_color: Some(self.secondary_content_bg),
            border: NO_BORDER,
        }
    }
    pub fn sub_content(&self) -> ContainerStyle {
        ContainerStyle {
            bg_color: Some(self.sub_content_bg),
            border: NO_BORDER,
        }
    }
    pub fn home_widget_container(&self) -> ContainerStyle {
        ContainerStyle {
            bg_color: Some(self.main_content_bg),
            border: Border {
                color: self.home_menu_widget_border_color,
                width: 2.0,
                radius: Radius::new(4),
            },
        }
    }
    pub fn default_scrollable(&self) -> ScrollableStyle {
        ScrollableStyle {
            v_scroll_rail_bg: Some(self.accent_color_bg),
            v_scroll_scroller_bg: self.interactable_bg,
        }
    }
    pub fn default_slider(&self) -> SliderStyle {
        SliderStyle {
            uncovered_bg: self.accent_color_bg,
            covered_bg: self.interactable_bg,
            handle_bg: self.interactable_bg,
            width: 6.0,
        }
    }
    pub fn default_text_input(&self) -> TextInputStyle {
        TextInputStyle {
            bg: self.accent_color_bg,
            border_color: self.text_input_border_color,
            border_width: 1.0,
            corner_radius: 0.0,
            placeholder_text_color: self.secondary_text_color,
            text_color: self.main_text_color,
            selection_color: self.interactable_color.scale_alpha(0.5),
        }
    }
    pub fn default_rule(&self, radius: impl Into<Pixels>) -> RuleStyle {
        RuleStyle {
            color: self.main_text_color,
            radius: Radius::new(radius),
        }
    }
    pub fn secondary_rule(&self, radius: impl Into<Pixels>) -> RuleStyle {
        RuleStyle {
            color: self.secondary_rule_color,
            radius: Radius::new(radius),
        }
    }
    pub fn default_progress_bar(&self) -> ProgressBarStyle {
        ProgressBarStyle {
            fill_bg: Background::Color(self.interactable_color),
            bg: self.accent_color_bg,
            border: NO_BORDER,
        }
    }
    pub fn default_modal(&self) -> ContainerStyle {
        self.home_widget_container()
    }
    pub fn default_modal_background(&self) -> ContainerStyle {
        ContainerStyle {
            bg_color: Some(Background::Color(Color::from_rgba(
                0.0,
                0.0,
                0.0,
                self.default_modal_background_alpha,
            ))),
            border: NO_BORDER,
        }
    }
    pub fn default_markdown(&self) -> markdown::Settings {
        markdown::Settings {
            text_size: Pixels(self.base_text_size),
            h1_size: Pixels(self.base_text_size * 2.0),
            h2_size: Pixels(self.base_text_size * 1.5),
            h3_size: Pixels(self.base_text_size * 1.25),
            h4_size: Pixels(self.base_text_size * 1.0),
            h5_size: Pixels(self.base_text_size * 0.875),
            h6_size: Pixels(self.base_text_size * 0.85),
            code_size: Pixels(self.base_text_size * 0.9),
            spacing: Pixels(10.0),
            style: markdown::Style {
                font: self.default_font,
                inline_code_highlight: Highlight {
                    background: self.secondary_content_bg,
                    border: Border {
                        color: self.markdown_code_border_color,
                        width: 2.0,
                        radius: Radius::new(2),
                    },
                },
                inline_code_padding: Padding {
                    top: 0.0,
                    right: 2.0,
                    bottom: 0.0,
                    left: 2.0,
                },
                inline_code_color: self.main_text_color,
                inline_code_font: Font::MONOSPACE,
                code_block_font: Font::MONOSPACE,
                link_color: self.link_color,
            },
        }
    }
    pub fn guide_button(&self) -> ButtonStyle {
        let mut default = self.default_button();
        default.status_bgs = self.guide_button_status_bgs;
        default
    }
}

// Color palettes

const DARK_STYLESHEET: Stylesheet = Stylesheet {
    base_text_size: 16.0,
    main_text_color: color!(255, 255, 255),
    error_text_color: color!(0xDC143C),
    default_font: Font::DEFAULT,
    secondary_text_color: color!(200, 200, 200),

    main_content_bg: Background::Color(color!(40, 40, 40)),
    secondary_content_bg: Background::Color(color!(20, 20, 20)),
    sub_content_bg: Background::Color(color![30, 30, 30]),

    accent_color_bg: Background::Color(color!(45, 45, 45)),

    secondary_rule_color: color!(0, 94, 245, 0.5),

    text_input_border_color: color!(55, 55, 55),
    home_menu_widget_border_color: color!(30, 30, 30),

    interactable_bg: Background::Color(color!(14, 96, 230)),
    interactable_color: color!(14, 96, 230),

    default_button_status_bgs: ButtonStatusBackgrounds {
        active: Some(Background::Color(color!(14, 96, 230))),
        hovered: Some(Background::Color(color!(32, 113, 245))),
        pressed: Some(Background::Color(color!(10, 86, 209))),
        disabled: Some(Background::Color(color!(1, 39, 99))),
    },
    secondary_button_status_bgs: ButtonStatusBackgrounds {
        active: Some(Background::Color(color!(143, 143, 143))),
        hovered: Some(Background::Color(color!(153, 153, 153))),
        pressed: Some(Background::Color(color!(133, 133, 133))),
        disabled: Some(Background::Color(color!(123, 123, 123))),
    },
    // default_button_status_bgs: ButtonStatusBackgrounds {
    //     active: None,
    //     hovered: None,
    //     pressed: None,
    //     disabled: None,
    // }
    guide_button_status_bgs: ButtonStatusBackgrounds {
        active: None,
        hovered: Some(Background::Color(color!(122, 122, 122))),
        pressed: Some(Background::Color(color!(100, 100, 100))),
        disabled: None,
    },
    default_modal_background_alpha: 0.6,
    link_color: color!(68, 147, 248),
    markdown_code_border_color: color!(26, 26, 26),
};

// Tack on app stylesheet data to any theme
pub trait AppTheme {
    fn stylesheet(&self) -> &'static Stylesheet;
}

impl AppTheme for Theme {
    fn stylesheet(&self) -> &'static Stylesheet {
        match &self {
            Theme::Dark => &DARK_STYLESHEET,
            _ => panic!("Unsupported theme used"),
        }
    }
}
