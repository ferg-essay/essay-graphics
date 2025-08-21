use std::ops::Index;

use essay_graphics_api::{color::Grey, Color, HorizAlign, PathStyle, TextStyle};

#[derive(Clone)]
pub struct UiStyle {
    pub background: Color,
    pub foreground: Color,
    pub border: Color,
    pub focus_border: Color,
    pub corner_radius: f32,
    pub shadow: Color,

    pub label: PathStyle,
    pub label_text: TextStyle,

    pub button2_on: UiStyleButton,
    pub button2_off: UiStyleButton,

    pub button: PathStyle,
    pub button_press: PathStyle,
    pub button_text: TextStyle,

    pub active: Colorset,
    pub inactive: Colorset,
    pub hover: Colorset,
}

impl UiStyle {
}

impl Default for UiStyle {
    fn default() -> Self {
        LightTheme.style()
    }
}

#[derive(Clone, Debug)]
pub struct UiStyleButton {
    pub background: Color,
    pub foreground: Color,
    pub hover_background: Color,
    pub hover_foreground: Color,
}

pub trait Theme {
    fn style(&self) -> UiStyle;
}

pub struct LightTheme;

impl Theme for LightTheme {
    fn style(&self) -> UiStyle {
        let mut text = TextStyle::new();
        text.halign(HorizAlign::Left);

        let azure = Color::from("azure");
        let azure_light = Color::from(0x3cadf3);

        UiStyle {
            background: Color::white(),
            foreground: Color::black(),
            border: Grey(0.7).into(),
            focus_border: azure,
            corner_radius: 10.,
            shadow: Color(0x00000020),

            label: PathStyle::new(),
            label_text: text.clone(),

            button2_on: UiStyleButton {
                background: azure,
                foreground: Color::white(),
                hover_background: azure_light, // Grey(0.97).into(),
                hover_foreground: Color::white(), // Grey(0.97).into(),
            },

            button2_off: UiStyleButton {
                background: Color::white(),
                foreground: Color::black(),
                hover_background: azure, // Grey(0.97).into(),
                hover_foreground: Color::white(), // Grey(0.97).into(),
            },

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

impl Index<State> for UiStyle {
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
