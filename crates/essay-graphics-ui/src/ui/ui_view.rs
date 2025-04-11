use essay_graphics_api::{renderer::{self, Canvas, Drawable, Renderer}, Bounds, Point, Size};

use crate::{page::Page, ui::Ui};

use super::{cursor::{ViewSizeId, ViewSizeCache}, ui::draw_top};

pub struct UiView {
    add_content: Box<dyn FnMut(&mut Ui)->() + Send>,
    last_id: ViewSizeId,
    state: Option<ViewSizeCache>,
}

impl UiView {
    pub fn new(add_content: impl FnMut(&mut Ui)->() + 'static + Send) -> Self {
        Self {
            add_content: Box::new(add_content),
            last_id: ViewSizeId::default(),
            state: None,
        }
    }
}

impl Drawable for UiView {
    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer
    ) -> renderer::Result<()> {
        let id = self.last_id;

        let prev_cache = self.state.take().unwrap_or_else(|| {
            ViewSizeCache::new(id)
        });

        let next_cache = draw_top(
            prev_cache, 
            renderer, 
            &mut self.add_content
        );

        self.last_id = id;
        self.state = Some(next_cache);

        Ok(())
    }
}
