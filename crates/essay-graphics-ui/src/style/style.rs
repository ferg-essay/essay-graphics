use essay_graphics_api::Color;

use crate::{style::UiTheme, ui::Ui};

pub trait UiStyle {
    fn background(&self, ui: &Ui) -> Color;
    fn foreground(&self, ui: &Ui) -> Color;

    fn border(&self, ui: &Ui) -> Color;
    fn border_width(&self, ui: &Ui) -> f32;

    fn corner_radius(&self, ui: &Ui) -> f32;

    fn hover_background(&self, ui: &Ui) -> Color;
    fn hover_foreground(&self, ui: &Ui) -> Color;
    fn hover_border(&self, ui: &Ui) -> Color;
}