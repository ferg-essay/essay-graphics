use essay_graphics_api::{input::Input};

use crate::{ui::{Response, Ui}, widget2::{tooltip::Tooltip, Element, Frame, Shell}};

pub trait Widget<Message> {
    #[allow(unused_variables)]
    fn update(
        &mut self,
        input: &Input,
        shell: &mut Shell<'_, Message>,
    ) {
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        shell: &mut Shell<'_, Message>,
    ) -> Response;
}

pub trait WidgetFrame<'a, Message>: Widget<Message> + Sized + 'a {
    fn frame(self) -> Frame<'a, Message> {
        Frame::new(Element::new(self))
    }

    fn tooltip(self, tooltip: impl Into<Element<'a, Message>>) -> Tooltip<'a, Message> {
        Tooltip::new(Element::new(self), tooltip.into())
    }
}
