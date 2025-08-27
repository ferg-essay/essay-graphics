use essay_graphics_api::Color;

pub trait Catalog: Sized {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}
pub struct Style {
    pub background: Color,
    pub icon_color: Color,
    pub text_color: Option<Color>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active {
        is_checked: bool,
    },
    Hovered {
        is_checked: bool,
    },
    Disabled {
        is_checked: bool,
    }
}