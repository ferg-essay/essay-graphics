use super::{Renderer, Result};

pub trait Drawable: Send + Sync {
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
    fn draw(&mut self, ui: &mut dyn Renderer) -> Result<()>;
}

impl Drawable for Box<dyn Drawable> {
    #[inline]
    fn draw(&mut self, ui: &mut dyn Renderer) -> Result<()> {
        self.as_mut().draw(ui)
    }
}

impl Drawable for Box<dyn Drawable + Send> {
    #[inline]
    fn draw(&mut self, ui: &mut dyn Renderer) -> Result<()> {
        self.as_mut().draw(ui)
    }
}

impl<F> Drawable for F
where
    F: FnMut(&mut dyn Renderer) -> Result<()> + Send + Sync + 'static
{
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        (self)(renderer)
    }
}
