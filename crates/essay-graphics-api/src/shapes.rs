use crate::{renderer::{self, Drawable, Renderer}, Color, Point, Size};

pub enum Shapes {
    None,
    Rectangle(Point, Size, f32, Color),
}

impl Drawable for Shapes {
    fn draw(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        ui.draw_shape(self)
    }
}