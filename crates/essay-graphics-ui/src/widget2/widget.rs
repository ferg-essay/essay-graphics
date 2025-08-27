use essay_graphics_api::{input::Input, Rectangle};

use crate::{painter::Painter, widget2::Shell};

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

    #[allow(unused_variables)]
    fn draw(
        &mut self,
        painter: &mut Painter,
        bounds: &Rectangle
    ) {
    }

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