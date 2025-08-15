use essay_graphics_api::renderer::Renderer;

use crate::ui::{context, ResponseValue, Ui, Widget};

/*
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
    */

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
    ) -> context::Response {
        let style = ui.style().label.clone();
        let style_text = ui.style().label_text.clone();
        let size = ui.text_size(&self.label, &style_text);
        let ResponseValue { 
            value, 
            response
        } = ui.allocate_rect(size);

        let label = String::from(&self.label);

        ui.painter_mut().add(move |ui: &mut dyn Renderer| {
            ui.draw_text(
                value.p0(), 
                &label, 
                0., 
                &style,
                &style_text
            )
        });

        response
    }
}
