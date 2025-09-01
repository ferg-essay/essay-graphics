use essay_graphics_api::renderer::{self, Drawable, Renderer};

use crate::ui::Context;

pub struct Painter {
    ctx: Context,
}

impl Painter {
    pub fn new(ctx: &Context) -> Self {
        Self {
            ctx: ctx.clone(),
        }
    }
    
    pub(crate) fn context(&self) -> &Context {
        &self.ctx
    }

    #[inline]
    fn paint<R>(&self, paint: impl FnOnce(&mut PaintList) -> R) -> R {
        self.ctx.graphics_mut(|layers| {
            (paint)(&mut layers.paint_list)
        })
    }

    pub fn add(&self, draw: impl Drawable + 'static) -> PaintIndex {
        self.paint(|paint_list| {
            paint_list.add(draw);
            PaintIndex(paint_list.len() - 1)
        })
    }

    pub fn set(&self, index: PaintIndex, draw: impl Drawable + 'static) {
        self.paint(|paint_list| {
            paint_list.set(index, draw);
        })
    }

    pub fn extend<I>(&self, iter: I) 
    where
        I : IntoIterator<Item = Box<dyn Drawable>>
    {
        self.paint(|paint_list| {
            for draw in iter {
                paint_list.add(draw);
            }
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PaintIndex(usize);

#[derive(Default)]
pub struct GraphicsLayers {
    paint_list: PaintList,
}

impl GraphicsLayers {
    pub fn draw(&mut self, draw: impl Drawable + Send + 'static) {
        self.paint_list.0.push(Box::new(draw));
    }

    pub(crate) fn render(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        for mut draw in self.paint_list.0.drain(..) {
            draw.draw(ui)?;
        }

        Ok(())
    }

    pub(crate) fn clear(&mut self) {
        self.paint_list.clear();
    }
}

#[derive(Default)]
pub struct PaintList(Vec<Box<dyn Drawable + Send + 'static>>);

impl PaintList {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn add(&mut self, draw: impl Drawable + Send + 'static) {
        self.0.push(Box::new(draw));
    }

    pub fn set(&mut self, index: PaintIndex, draw: impl Drawable + Send + 'static) {
        assert!(index.0 < self.0.len());

        self.0[index.0] = Box::new(draw);
    }

    pub fn clear(&mut self) {
        self.0.drain(..);
    }
}
