use super::Renderer;
use crate::{CanvasEvent, Canvas};

pub trait FigureApi {
    fn update(&mut self, canvas: &Canvas);

    fn draw(&mut self, renderer: &mut dyn Renderer);

    fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent);
}