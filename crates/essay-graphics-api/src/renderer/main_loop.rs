use crate::{renderer, renderer::Drawable};

pub trait Backend {
    fn main_loop(&mut self, drawable: Box<dyn Drawable>) -> renderer::Result<()>;
}
