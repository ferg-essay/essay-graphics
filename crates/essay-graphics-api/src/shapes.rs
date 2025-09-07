use crate::{renderer::{self, Drawable, Renderer}, Color, Point, Size};

pub enum Shapes {
    None,
    Quad(Quad),
}

impl Shapes {
    pub fn rect(
        pos: Point, 
        size: Size, 
        r: f32, 
        color: Color,
    ) -> Self {
        Self::Quad(Quad::rect(pos, size, r, color))
    }

    pub fn quad(
        pos: Point, 
        size: Size, 
        r_outer: f32, 
        color_outer: Color,
        r_inner: f32, 
        color_inner: Color,
    ) -> Self {
        Self::Quad(Quad::new(pos, size, r_outer, color_outer, r_inner, color_inner))
    }
}

impl Drawable for Shapes {
    fn draw(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        ui.draw_shape(self)
    }
}

pub struct Quad {
    pub pos: Point,
    pub size: Size,
    pub r_outer: f32,
    pub color_outer: Color,
    pub r_inner: f32,
    pub color_inner: Color,
}

impl Quad {
    pub fn new(
        pos: Point, 
        size: Size, 
        r_outer: f32, 
        color_outer: Color,
        r_inner: f32, 
        color_inner: Color,
    ) -> Self {
        Self {
            pos, size, r_outer, color_outer, r_inner, color_inner
        }
    }

    pub fn rect(
        pos: Point, 
        size: Size, 
        r: f32, 
        color: Color,
    ) -> Self {
        Self {
            pos, 
            size, 
            r_outer: r, 
            color_outer: color, 
            r_inner: r, 
            color_inner: color
        }
    }
}