use crate::backend::Renderer;

pub trait Artist {
    fn draw(&mut self, renderer: &mut dyn Renderer);
}