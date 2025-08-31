use essay_graphics_api::renderer::Renderer;

use crate::ui::{ui::Widget, Response, ResponseValue, Ui};

pub struct Label {
    label: String,
}

impl Label {
    pub fn new(label: &str) -> Self {
        Self {
            label: String::from(label),
        }
    }
}

impl Widget for Label {
    fn ui(
        self, 
        ui: &mut Ui, 
    ) -> Response {
        let style = ui.style().label.clone();
        let style_text = ui.style().label_text.clone();
        let size = ui.text_size(&self.label, &style_text);
        
        let ResponseValue { 
            value, 
            response
        } = ui.allocate_rect(size);

        let label = String::from(&self.label);

        ui.painter_mut().add(move |renderer: &mut dyn Renderer| {
            renderer.draw_text(
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

#[cfg(test)]
mod test {
    use essay_graphics_test::{TestGraphicsContext, TestRenderer};

    use crate::{context::Context};

    #[test]
    fn label() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.label("Test");
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'Test'");
    }

    #[test]
    fn vert_label() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.label("A");
            ui.label("B");
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
text (0.0,26.7) 'B'");

        ctx.run(&mut test, |ui| {
            ui.label("A");
            ui.label("B");
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
text (0.0,26.7) 'B'");
    }
}
