use crate::ui::{context, ui::Response, ui2::{Ui2, Widget2}, Ui};

use super::ui::Widget;

pub(crate) struct Label {
    label: String,
}

impl Label {
    pub(crate) fn new(label: &str) -> Self {
        Self {
            label: String::from(label),
        }
    }
}

impl Widget for Label {
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

pub(crate) struct Label2 {
    label: String,
}

impl Label2 {
    pub(crate) fn new(label: &str) -> Self {
        Self {
            label: String::from(label),
        }
    }
}

impl Widget2 for Label2 {
    fn ui(
        &mut self, 
        ui: &mut Ui2, 
    ) -> context::Response {
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

        context::Response::default()
    }
}
