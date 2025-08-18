use crate::{output::Output, renderer::{self, Renderer}};

pub trait Backend {
    fn context(&self) -> Box<dyn GraphicsContext>;

    fn main_loop(&mut self, app: Box<dyn App>) -> renderer::Result<()>;
}

pub trait App: Send + Sync {
    fn render(&mut self, ui: &mut dyn Renderer) -> renderer::Result<Output>;
}

impl App for dyn FnMut(&mut dyn Renderer) -> renderer::Result<Output> + Send + Sync {
    fn render(&mut self, ui: &mut dyn Renderer) -> renderer::Result<Output> {
        (self)(ui)
    }
}

pub trait GraphicsContext: Send + Sync {
    fn default_font_set(&self) -> Box<dyn FontSetMetrics>;
}

pub trait FontSetMetrics: Send + Sync {
    fn glyph_size(&mut self, size: f32, glyph: char) -> GlyphSize;
}

#[derive(Clone, Copy, Debug)]
pub struct GlyphSize {
    pub width: f32,
    pub height: f32,
    pub lsb: f32,
    // TODO: descend
}
