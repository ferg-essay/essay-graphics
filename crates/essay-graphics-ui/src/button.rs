use essay_graphics_api::{renderer, Point};

use crate::Ui;

use super::ui::Widget;

pub(crate) struct UiButton {
    label: String,
    press: bool,
}

impl UiButton {
    pub(crate) fn new(label: &str) -> Self {
        Self {
            label: String::from(label),
            press: false,
        }
    }
}

impl Widget for UiButton {
    fn ui(&mut self, ui: &mut Ui) -> renderer::Result<()> {
        let bounds = ui.allocate_rect(Point(100., 30.));
        let pos = bounds.p0();

        if self.press { 
            let button_press = ui.style().button_press.clone();
            let button_text = ui.style().button_text.clone();
            ui.renderer().draw_text(pos, &self.label, 0., &button_press, &button_text)
        } else {
            let button = ui.style().button.clone();
            let button_text = ui.style().button_text.clone();

            ui.renderer().draw_text(pos, &self.label, 0., &button, &button_text)
        }
    }
}
