use essay_graphics_api::{renderer::{self, Renderer}, Point};

use super::{style::UiStyle, ui::UiItem};

pub(crate) struct UiLabel {
    pos: Point,
    label: String,
}

impl UiLabel {
    pub(crate) fn new(pos: Point, label: &str) -> Self {
        Self {
            pos,
            label: String::from(label),
        }
    }
}

impl UiItem for UiLabel {
    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer, 
        style: &UiStyle
    ) -> renderer::Result<()> {
        renderer.draw_text(self.pos, &self.label, 0., &style.label, &style.label_text)
    }
}
