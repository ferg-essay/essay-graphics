use essay_graphics_api::{renderer::{self, Canvas, Drawable, Event, Renderer}, Bounds};

use super::{ui::UiRoot, Ui};

pub struct UiView {
    builder: Box<dyn FnMut(&mut Ui)->() + Send>,

    ui: UiRoot,
}

impl UiView {
    pub fn new(mut builder: impl FnMut(&mut Ui)->() + 'static + Send) -> Self {
        let mut ui = Ui::new();
        builder(&mut ui);

        Self {
            builder: Box::new(builder),
            ui: ui.build(),
        }
    }
}

impl Drawable for UiView {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        if self.ui.is_dirty() {
            let mut ui = Ui::new();
            (self.builder)(&mut ui);
            self.ui = ui.build();
            let pos = renderer.pos().clone();
            self.ui.resize(renderer, &pos);
        }

        self.ui.draw(renderer)
    }

    fn resize(
        &mut self, 
        renderer: &mut dyn Renderer, 
        bounds: &Bounds<Canvas>
    ) -> Bounds<Canvas> {
        self.ui.resize(renderer, bounds)
    }

    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) {
        self.ui.event(renderer, event)
    }
}
