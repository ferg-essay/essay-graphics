use essay_graphics_api::{Color, Padding};

use crate::{style::{theme::{Palette, ThemeSize}}, ui::Ui};

pub trait UiStyle {
    fn background(&self, ui: &Ui) -> Color;
    fn foreground(&self, ui: &Ui) -> Color;

    fn border(&self, ui: &Ui) -> Color;

    fn hover_background(&self, ui: &Ui) -> Color;
    fn hover_foreground(&self, ui: &Ui) -> Color;
    fn hover_border(&self, ui: &Ui) -> Color;

    fn background_on_hover(&self, ui: &Ui, is_hover: bool) -> Color {
        if is_hover {
            self.hover_background(ui)
        } else {
            self.background(ui)
        }
    }

    fn foreground_on_hover(&self, ui: &Ui, is_hover: bool) -> Color {
        if is_hover {
            self.hover_foreground(ui)
        } else {
            self.foreground(ui)
        }
    }

    fn border_on_hover(&self, ui: &Ui, is_hover: bool) -> Color {
        if is_hover {
            self.hover_border(ui)
        } else {
            self.border(ui)
        }
    }

    fn padding(&self, ui: &Ui) -> Padding;
    fn margin(&self, ui: &Ui) -> Padding;
    fn border_width(&self, ui: &Ui) -> f32;
    fn corner_radius(&self, ui: &Ui) -> f32;
}

#[derive(Copy, Clone, Debug)]
pub enum Style {
    None,
    Base,
    ButtonOn,
    ButtonOff,
    Group,
}

impl Default for Style {
    fn default() -> Self {
        Self::Base
    }
}

impl Style {
    fn palette<'a>(&self, ui: &'a Ui) -> &'a Palette {
        match self {
            Style::None => panic!("none does not have a palette"),
            Style::Base => &ui.theme().base,
            Style::ButtonOn => &ui.theme().button_on,
            Style::ButtonOff => &ui.theme().button_off,
            Style::Group => &ui.theme().group,
        }
    }

    fn size<'a>(&self, ui: &'a Ui) -> &'a ThemeSize {
        match self {
            Style::None => panic!("none does not have a size"),
            Style::Base => &ui.theme().base_size,
            Style::ButtonOn => &ui.theme().button_size,
            Style::ButtonOff => &ui.theme().button_size,
            Style::Group => &ui.theme().group_size,
        }
    }
}

impl UiStyle for Style {
    #[inline]
    fn background(&self, ui: &Ui) -> Color {
        match self {
            Style::None => Color(0),
            _ => self.palette(ui).background,
        }
    }

    #[inline]
    fn foreground(&self, ui: &Ui) -> Color {
        match self {
            Style::None => Color(0),
            _ => self.palette(ui).foreground,
        }
    }

    #[inline]
    fn border(&self, ui: &Ui) -> Color {
        match self {
            Style::None => Color(0),
            // TODO:
            _ => self.palette(ui).foreground,
        }
    }

    #[inline]
    fn hover_background(&self, ui: &Ui) -> Color {
        match self {
            Style::None => Color(0),
            _ => self.palette(ui).hover_background,
        }
    }

    #[inline]
    fn hover_foreground(&self, ui: &Ui) -> Color {
        match self {
            Style::None => Color(0),
            _ => self.palette(ui).hover_foreground,
        }
    }

    #[inline]
    fn hover_border(&self, ui: &Ui) -> Color {
        match self {
            Style::None => Color(0),
            // TODO:
            _ => self.palette(ui).hover_foreground,
        }
    }

    #[inline]
    fn padding(&self, ui: &Ui) -> Padding {
        match self {
            Style::None => Padding::default(),
            _ => self.size(ui).padding,
        }
    }

    #[inline]
    fn margin(&self, ui: &Ui) -> Padding {
        match self {
            Style::None => Padding::default(),
            _ => self.size(ui).margin,
        }
    }

    #[inline]
    fn border_width(&self, ui: &Ui) -> f32 {
        match self {
            Style::None => 0.,
            _ => self.size(ui).border_width,
        }
    }

    #[inline]
    fn corner_radius(&self, ui: &Ui) -> f32 {
        match self {
            Style::None => 0.,
            _ => self.size(ui).corner_radius,
        }
    }
}
