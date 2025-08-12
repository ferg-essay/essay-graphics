use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, RwLock};

use essay_graphics_api::renderer::{Canvas, Renderer};
use essay_graphics_api::Bounds;

use crate::ui::cursor::ViewSizeCache;
use crate::ui::layers::GraphicsLayers;
use crate::ui::null_render::NullRenderer;
use crate::ui::style::UiStyle;
use crate::ui::ui2::Ui2;
use crate::ui::widget::{WidgetRect, WidgetRects};
use crate::ui::Ui;

#[derive(Clone)]
pub struct Context(Arc<RwLock<ContextImpl>>);

impl Context {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(ContextImpl::default())))
    }

    fn read<R>(&self, reader: impl FnOnce(&ContextImpl) -> R) -> R {
        let inner = self.0.read().unwrap();
        (reader)(inner.deref())
    }

    fn write<R>(&self, writer: impl FnOnce(&mut ContextImpl) -> R) -> R {
        let mut inner = self.0.write().unwrap();
        (writer)(inner.deref_mut())
    }

    #[inline]
    pub fn graphics<R>(&self, reader: impl FnOnce(&GraphicsLayers) -> R) -> R {
        self.read(|cxt| (reader)(&cxt.viewport.layers))
    }

    #[inline]
    pub fn graphics_mut<R>(&self, writer: impl FnOnce(&mut GraphicsLayers) -> R) -> R {
        self.write(|cxt| (writer)(&mut cxt.viewport.layers))
    }

    #[inline]
    pub fn pass<R>(&self, reader: impl FnOnce(&RenderPass) -> R) -> R {
        self.read(|cxt| (reader)(&cxt.viewport.pass))
    }

    #[inline]
    pub fn pass_mut<R>(&self, writer: impl FnOnce(&mut RenderPass) -> R) -> R {
        self.write(|cxt| (writer)(&mut cxt.viewport.pass))
    }

    #[inline]
    pub fn last_pass<R>(&self, reader: impl FnOnce(&RenderPass) -> R) -> R {
        self.read(|cxt| (reader)(&cxt.viewport.last_pass))
    }

    #[inline]
    pub fn last_pass_mut<R>(&self, writer: impl FnOnce(&mut RenderPass) -> R) -> R {
        self.write(|cxt| (writer)(&mut cxt.viewport.last_pass))
    }
}

impl Context {
    pub fn run_ui(
        &self, 
        renderer: &mut dyn Renderer, 
        mut draw: impl FnMut(&mut Ui2) + Send
    ) {
        self.run(renderer, move |cxt, renderer| {
            Ui2::top(cxt, renderer, |ui2| (draw)(ui2));
        })
    }

    pub fn run(
        &self, 
        renderer: &mut dyn Renderer, 
        mut draw: impl FnMut(&Self, &mut dyn Renderer) + Send
    ) {
        loop {
            let is_resize = self.write(|cxt| {
                if cxt.cache_pos != renderer.pos() {
                    cxt.cache_pos = renderer.pos();
                    true
                } else {
                    false
                }
            });

            self.start_pass(is_resize);

            if is_resize {
                let mut null_render = NullRenderer(renderer);

                (draw)(self, &mut null_render);
            } else {
                (draw)(self, renderer);
            }

            if ! is_resize {
                break;
            }
        }

        self.graphics_mut(|layers| {
            layers.render(renderer).unwrap();
        })
    }

    fn start_pass(&self, _is_resize: bool) {
        self.write(|cxt| {
            let mut pass = RenderPass::default();
            std::mem::swap(&mut cxt.viewport.pass, &mut pass);
            std::mem::swap(&mut cxt.viewport.last_pass, &mut pass);
        })
    }


    
    pub(crate) fn create_widget(&self, widget: WidgetRect) {
        self.write(|ctx| {
            ctx.viewport.pass.widgets.insert(widget);
        });

        self.get_response(widget)
    }

    pub(crate) fn get_response(&self, widget: WidgetRect) {

    }
}

#[derive(Default)]
pub(crate) struct ContextImpl {
    viewport: Viewport,

    style: Arc<UiStyle>,

    cache_pos: Bounds<Canvas>,
}

#[derive(Default)]
pub struct Viewport {
    last_pass: RenderPass,
    pass: RenderPass,
    
    layers: GraphicsLayers,
}

#[derive(Default)]
pub struct RenderPass {
    widgets: WidgetRects,

    view_size: ViewSizeCache,
}
