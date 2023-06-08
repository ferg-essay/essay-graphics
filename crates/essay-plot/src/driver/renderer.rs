use essay_tensor::Tensor;

use crate::{artist::{Path, StyleOpt}, frame::{Affine2d, Bounds, Point, Data}};

use super::{GraphicsContext, Canvas};

pub trait Renderer {
    ///
    /// Returns the boundary of the canvas, usually in pixels or points.
    ///
    fn get_canvas_bounds(&self) -> Bounds<Canvas> {
        Bounds::new(Point(0., 0.), Point(1., 1.))
    }

    fn new_gc(&mut self) -> GraphicsContext {
        GraphicsContext::default()
    }

    fn draw_path(
        &mut self, 
        style: &dyn StyleOpt, 
        path: &Path<Data>, 
        to_canvas: &Affine2d,
        clip: &Bounds<Canvas>,
    ) -> Result<(), RenderErr>;
}

#[derive(Debug)]
pub enum RenderErr {
    NotImplemented,
}
