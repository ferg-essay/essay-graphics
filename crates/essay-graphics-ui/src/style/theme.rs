use std::ops::Index;

use essay_graphics_api::{color::Grey, Color, HorizAlign, Padding, PathStyle, TextStyle};

use crate::{style::UiStyle, ui::Ui};

#[derive(Clone)]
pub struct UiTheme {
    pub base: Palette,
    pub base_size: ThemeSize,

    pub button_on: Palette,
    pub button_off: Palette,
    pub button_size: ThemeSize,

    pub group: Palette,
    pub group_size: ThemeSize,

    pub background: Color,
    pub foreground: Color,
    pub border: Color,
    pub shadow: Color,
    pub focus_border: Color,

    pub padding: Padding,
    pub margin: Padding,
    pub border_width: f32,
    pub corner_radius: f32,

    pub label: PathStyle,
    pub label_text: TextStyle,

    pub button: PathStyle,
    pub button_press: PathStyle,
    pub button_text: TextStyle,

    pub active: Colorset,
    pub inactive: Colorset,
    pub hover: Colorset,
}

impl UiTheme {
}

impl Default for UiTheme {
    fn default() -> Self {
        LightTheme.style()
    }
}

#[derive(Clone, Debug)]
pub struct Palette {
    pub background: Color,
    pub foreground: Color,
    pub hover_background: Color,
    pub hover_foreground: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self { 
            background: Color::white(),
            foreground: Color::black(),
            hover_background: Color::from_grey(0.95),
            hover_foreground: Color::black(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ThemeSize {
    pub padding: Padding,
    pub margin: Padding,
    pub corner_radius: f32,
    pub border_width: f32,
}

impl Default for ThemeSize {
    fn default() -> Self {
        Self { 
            padding: Padding::from_all(6.),
            margin: Padding::from_all(0.),
            corner_radius: 10.,
            border_width: 0.,
        }
    }
}

pub trait Theme {
    fn style(&self) -> UiTheme;
}

pub struct LightTheme;

impl Theme for LightTheme {
    fn style(&self) -> UiTheme {
        let mut text = TextStyle::new();
        text.halign(HorizAlign::Left);

        let azure = Color::from("azure");
        let azure_light = Color::from(0x3cadf3);

        let base = Palette {
            background: Color::white(),
            foreground: Color::black(),
            hover_background: Color::white(),
            hover_foreground: Color::black(),
        };

        let base_size = ThemeSize::default();

        UiTheme {
            background: Color::white(),
            foreground: Color::black(),
            border: Grey(0.7).into(),
            shadow: Color(0x00000020),

            padding: Padding::from(6.),
            margin: Padding::from(0.),
            border_width: 0.,
            focus_border: azure,
            corner_radius: 10.,

            base: base.clone(),

            base_size: base_size.clone(),

            button_on: Palette {
                background: azure,
                foreground: Color::white(),
                hover_background: azure_light, // Grey(0.97).into(),
                hover_foreground: Color::white(), // Grey(0.97).into(),
            },

            button_off: Palette {
                background: Color::white(),
                foreground: Color::black(),
                hover_background: azure, // Grey(0.97).into(),
                hover_foreground: Color::white(), // Grey(0.97).into(),
            },

            button_size: base_size.clone(),

            group: base.clone(),
            group_size: ThemeSize {
                border_width: 1.,
                margin: Padding::from_all(5.),
                .. base_size.clone()
            },

            label: PathStyle::new(),
            label_text: text.clone(),

            button: PathStyle::new(),
            button_press: PathStyle::new(),
            button_text: text,

            inactive: Colorset::new(
                Grey(0.95),
                Grey(0.3),
                Grey(0.2),
                Grey(0.7)
            ),

            hover: Colorset::new(
                Grey(0.98),
                Grey(0.1),
                "red",
                Grey(0.7)
            ),

            active: Colorset::new(
                "azure",
                Grey(1.),
                "azure",
                "azure",
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Active,
    Inactive,
    Hover,
}

impl Index<State> for UiTheme {
    type Output = Colorset;

    #[inline]
    fn index(&self, index: State) -> &Self::Output {
        match index {
            State::Active => &self.active,
            State::Inactive => &self.inactive,
            State::Hover => &self.hover,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Colorset {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub edge: Color,
}

impl Colorset {
    pub fn new(
        background: impl Into<Color>,
        foreground: impl Into<Color>, 
        accent: impl Into<Color>,
        edge: impl Into<Color>,
    ) -> Self {
        Self {
            background: background.into(),
            foreground: foreground.into(),
            accent: accent.into(),
            edge: edge.into(),
        }
    }
}
