use std::ops::Index;

use essay_graphics_api::{color::Grey, Color, HorizAlign, PathStyle, TextStyle};

#[derive(Clone)]
pub struct UiStyle {
    pub label: PathStyle,
    pub label_text: TextStyle,

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
        let mut text = TextStyle::new();
        text.halign(HorizAlign::Left);

        Self {
            label: PathStyle::new(),
            label_text: text.clone(),

            button: PathStyle::new(),
            button_press: PathStyle::new(),
            button_text: text.clone(),

            inactive: Colorset::new(
                Grey(0.95),
                Grey(0.3),
                Grey(0.2),
                Grey(0.7)
            ),

            active: Colorset::new(
                Grey(1.),
                Grey(0.),
                "azure",
                Grey(0.7)
            ),

            hover: Colorset::new(
                Grey(0.98),
                Grey(0.1),
                "red",
                Grey(0.7)
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
