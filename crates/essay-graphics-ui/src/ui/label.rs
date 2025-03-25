use crate::ui::{ui::Response, Ui};

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
    ) -> Response {
        let style = ui.style().label.clone();
        let style_text = ui.style().label_text.clone();
        let size = ui.text_size(&self.label, &style_text);
        let pos = ui.allocate_rect(size);

        ui.renderer().draw_text(
            pos.p0(), 
            &self.label, 
            0., 
            &style,
            &style_text
        ).unwrap();

        Response::default()
    }
}
