use essay_graphics_api::{renderer::{self, Canvas, Drawable, Event, Renderer}, Bounds};

use super::Ui;

pub struct UiView {
    builder: Box<dyn FnMut(&mut Ui)->() + Send>,

    // ui: UiRoot,
}

impl UiView {
    pub fn new(builder: impl FnMut(&mut Ui)->() + 'static + Send) -> Self {
        Self {
            builder: Box::new(builder),
        }
    }
}

impl Drawable for UiView {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        let mut ui = Ui::new(renderer);
        (self.builder)(&mut ui);

        Ok(())
    }

    fn resize(
        &mut self, 
        _renderer: &mut dyn Renderer, 
        bounds: &Bounds<Canvas>
    ) -> Bounds<Canvas> {
        // self.ui.resize(renderer, bounds)
        bounds.clone()
    }

    fn event(&mut self, _renderer: &mut dyn Renderer, _event: &Event) {
        // self.ui.event(renderer, event)
    }
}
