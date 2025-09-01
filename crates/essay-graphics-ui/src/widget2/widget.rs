use essay_graphics_api::{input::Input};

use crate::{ui::{Response, Ui, Widget}, widget2::{tooltip::Tooltip, Element, Frame}};

pub trait WidgetFrame<'a, Message>: Widget<Message> + Sized + 'a {
    fn frame(self) -> Frame<'a, Message> {
        Frame::new(Element::new(self))
    }

    fn tooltip(self, tooltip: impl Into<Element<'a, Message>>) -> Tooltip<'a, Message> {
        Tooltip::new(Element::new(self), tooltip.into())
    }
}
