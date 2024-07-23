use super::Renderer;
use crate::{Bounds, Canvas, CanvasEvent};

pub trait Drawable {
    ///
    /// Called to inform the drawable when the view bounds or scale factor
    /// has changed.
    /// 
    // fn update(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>);

    ///
    /// Callback to draw into the renderer.
    /// 
    /// The view pos is identical to the most recent update to avoid the
    /// need to store the position.
    /// 
    fn draw(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>);

    ///
    /// Called to inform the drawable when an event occurs in the drawable.
    /// 
    fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent);
}