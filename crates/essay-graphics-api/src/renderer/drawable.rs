use super::{Event, Renderer, Result};

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
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()>;

    ///
    /// Called to inform the drawable when an event occurs in the drawable.
    /// 
    #[allow(unused_variables)]
    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) {
    }
}

impl<F> Drawable for F
where
    F: FnMut(&mut dyn Renderer) -> Result<()> 
{
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        (self)(renderer)
    }
}

