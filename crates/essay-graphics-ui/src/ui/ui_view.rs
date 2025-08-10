use essay_graphics_api::{renderer::{self, Drawable, Renderer}, Path, PathStyle};

use crate::ui::{null_render::NullRenderer, Ui};

use super::{cursor::ViewSizeCache, style::{State, UiStyle}, ui::draw_top};

pub struct UiView {
    add_content: Box<dyn FnMut(&mut Ui)->() + Send>,
    state: Option<ViewSizeCache>,
    style: UiStyle,
}

impl UiView {
    pub fn new(add_content: impl FnMut(&mut Ui)->() + Send + 'static) -> Self {
        let mut style = UiStyle::new();
        style.button_press.color("red");

        Self {
            add_content: Box::new(add_content),
            state: None,
            style,
        }
    }
}

impl Drawable for UiView {
    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer
    ) -> renderer::Result<()> {
        let is_new = self.state.is_none();
        let prev_cache = self.state.take().unwrap_or_else(|| {
            ViewSizeCache::new()
        });

        let (_, mut next_cache) = if is_new {
            let mut null_renderer = NullRenderer(renderer);

            draw_top(
                &prev_cache, 
                &mut null_renderer, 
                &self.style,
                &mut self.add_content,
            )
        } else {
            draw_top(
                &prev_cache, 
                renderer, 
                &self.style,
                &mut self.add_content,
            )
        };

        if ! next_cache.merge(&prev_cache) {
            let path = Path::from(renderer.pos());
            let mut style = PathStyle::new();
            style.color(self.style[State::Active].background);

            renderer.draw_path(&path, &style)?;

            // redraw if page cache changes
            draw_top(
                &next_cache, 
                renderer, 
                &self.style,
                &mut self.add_content
            );
        }

        self.state = Some(next_cache);

        Ok(())
    }
}

pub struct UiTop {
    state: Option<ViewSizeCache>,
    style: UiStyle,
}

impl UiTop {
    pub fn draw<R>(
        &mut self, 
        renderer: &mut dyn Renderer, 
        mut add_content: impl FnMut(&mut Ui) -> R
    ) -> R {
        let is_new = self.state.is_none();
        let prev_cache = self.state.take().unwrap_or_else(|| {
            ViewSizeCache::new()
        });
        // let is_new = false;

        let (result, mut next_cache) = if is_new {
            let mut null_renderer = NullRenderer(renderer);

            draw_top(
                &prev_cache, 
                &mut null_renderer, 
                &self.style,
                &mut add_content,
            )
        } else {
            draw_top(
                &prev_cache, 
                renderer, 
                &self.style,
                &mut add_content,
            )
        };

        if ! next_cache.merge(&prev_cache) {
            let path = Path::from(renderer.pos());
            let mut style = PathStyle::new();
            style.color(self.style[State::Active].background);

            renderer.flush();
            renderer.draw_path(&path, &style).unwrap();

            // redraw if page cache changes
            draw_top(
                &next_cache, 
                renderer, 
                &self.style,
                &mut add_content
            );
        }

        self.state = Some(next_cache);

        result
    }
}

impl Default for UiTop {
    fn default() -> Self {
        let style = UiStyle::new();

        Self {
            state: None,
            style,
        }
    }
}
