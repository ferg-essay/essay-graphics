use std::ops;

use crate::{renderer::{self, Drawable, Pos, Renderer}, Size, TextStyle};

pub struct Viewport(Box<dyn ViewportApi>);

impl ops::Deref for Viewport {
    type Target = Box<dyn ViewportApi>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Viewport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait ViewportApi: Send {
    fn screen_pos(&self) -> Pos;

    fn scale_factor(&self) -> f32;

    fn to_px(&self, size: f32) -> f32 {
        size * self.scale_factor()
    }

    fn text_size(
        &mut self,
        text: &str,
        text_style: &TextStyle
    ) -> Size;

    fn draw(&mut self, drawable: &dyn FnOnce(&mut dyn Renderer) -> renderer::Result<()>);
}