use essay_graphics_api::{renderer::Renderer};

use crate::{ui::{Response, ResponseValue}, widget2::{Element, Shell, Widget}};

pub struct Text {
    pub content: String,
    // pub font: Font,

    // pub bounds: Size,
}

impl Text {
    pub fn new(value: &str) -> Self {
        Self {
            content: String::from(value),
        }
    }

    pub fn value(&self) -> &str {
        &self.content
    }
}

impl<'a, Message> Widget<Message> for Text {
    fn draw(
        &mut self,
        ui: &mut crate::ui::Ui,
        _bounds: &essay_graphics_api::Rectangle,
        _shell: &mut Shell<Message>,
    ) -> Response {
        let style = ui.style().label.clone();
        let style_text = ui.style().label_text.clone();
        let size = ui.text_size(&self.content, &style_text);
        
        let ResponseValue { 
            value, 
            response
        } = ui.allocate_rect(size);

        let label = String::from(&self.content);

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

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self {
            content: String::from(value),
        }
    }
}

impl<'a, M> From<&str> for Element<'a, M> {
    fn from(value: &str) -> Self {
        Text::from(value).into()
    }
}

impl<'a, M> From<Text> for Element<'a, M> {
    fn from(value: Text) -> Self {
        Element::new(value)
    }
}
