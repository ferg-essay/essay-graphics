use essay_graphics_api::renderer::{self, Drawable, Renderer};

use crate::ui::Ui;

use super::{cursor::ViewSizeCache, ui::draw_top};

pub struct UiView {
    add_content: Box<dyn FnMut(&mut Ui)->() + Send>,
    state: Option<ViewSizeCache>,
}

impl UiView {
    pub fn new(add_content: impl FnMut(&mut Ui)->() + Send + 'static) -> Self {
        Self {
            add_content: Box::new(add_content),
            state: None,
        }
    }
}

impl Drawable for UiView {
    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer
    ) -> renderer::Result<()> {
        let prev_cache = self.state.take().unwrap_or_else(|| {
            ViewSizeCache::new()
        });

        let (_, next_cache) = draw_top(
            prev_cache, 
            renderer, 
            &mut self.add_content
        );

        self.state = Some(next_cache);

        Ok(())
    }
}
