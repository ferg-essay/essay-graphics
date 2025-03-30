use essay_graphics_api::renderer::{self, Drawable, Renderer};

use crate::ui::{ui::Cursor, Ui};

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
        let cursor = Cursor::new(renderer.pos().clone());
        
        let mut ui = Ui::new(renderer, cursor);
        (self.builder)(&mut ui);

        Ok(())
    }
}
