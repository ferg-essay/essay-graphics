use essay_graphics_api::renderer::Renderer;

use crate::ui::{DrawWidget, Response, ResponseValue, Ui};

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

impl DrawWidget for Label {
    fn draw(
        &mut self, 
        ui: &mut Ui, 
    ) -> Response {
        let style = ui.style().label.clone();
        let style_text = ui.style().label_text.clone();
        let size = ui.text_size(&self.label, &style_text);
        
        let ResponseValue { 
            value, 
            response
        } = ui.allocate(size);

        let label = String::from(&self.label);

        ui.painter().add(move |renderer: &mut dyn Renderer| {
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

    use crate::ui::Context;

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
