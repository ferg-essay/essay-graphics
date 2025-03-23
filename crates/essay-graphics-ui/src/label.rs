use essay_graphics_api::{renderer, Point};

use crate::Ui;

use super::ui::Widget;

pub(crate) struct UiLabel {
    label: String,
}

impl UiLabel {
    pub(crate) fn new(label: &str) -> Self {
        Self {
            label: String::from(label),
        }
    }
}

impl Widget for UiLabel {
    fn ui(
        &mut self, 
        ui: &mut Ui, 
    ) -> renderer::Result<()> {
        let pos = ui.allocate_rect(Point(100., 20.));
        let style = ui.style().label.clone();
        let style_text = ui.style().label_text.clone();

        ui.renderer().draw_text(
            pos.p0(), 
            &self.label, 
            0., 
            &style,
            &style_text
        )
    }
}
