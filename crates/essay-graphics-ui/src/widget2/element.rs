use essay_graphics_api::Rectangle;

use crate::{ui::{Response, Ui}, widget2::{widget::Widget, Frame, Shell}};


pub struct Element<'a, Message> {
    widget: Box<dyn Widget<Message> + 'a>,
}

impl<'a, Message> Element<'a, Message> {
    pub fn new(widget: impl Widget<Message> + 'a) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }

    pub fn as_widget(&self) -> &dyn Widget<Message> {
        self.widget.as_ref()
    }

    pub fn as_widget_mut(&mut self) -> &dyn Widget<Message> {
        self.widget.as_mut()
    }
}

impl<'a, Message> Widget<Message> for Element<'a, Message> {
    fn update(
        &mut self,
        input: &essay_graphics_api::input::Input,
        shell: &mut super::Shell<'_, Message>,
    ) {
        self.widget.update(input, shell)
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        bounds: &Rectangle,
        shell: &mut Shell<Message>,
    ) -> Response {
        self.widget.draw(ui, bounds, shell)
    }
}
