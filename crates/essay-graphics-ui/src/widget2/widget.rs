use essay_graphics_api::{input::Input, Rectangle};

use crate::{painter::Painter, ui::{Response, Ui}, widget2::{tooltip::Tooltip, Element, Frame, Shell}};

pub trait Widget<Message> {
    #[allow(unused_variables)]
    fn update(
        &mut self,
        input: &Input,
        shell: &mut Shell<'_, Message>,
    ) {
    }

    /*
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
     */

    fn draw(
        &mut self,
        ui: &mut Ui,
        bounds: &Rectangle,
        shell: &mut Shell<'_, Message>,
    ) -> Response;

    /*
  fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style, // text_color
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    */
}

pub trait WidgetFrame<'a, Message>: Widget<Message> + Sized + 'a {
    fn frame(self) -> Frame<'a, Message> {
        Frame::new(Element::new(self))
    }

    fn tooltip(self, tooltip: impl Into<Element<'a, Message>>) -> Tooltip<'a, Message> {
        Tooltip::new(Element::new(self), tooltip.into())
    }
}
