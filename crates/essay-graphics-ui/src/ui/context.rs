use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, RwLock};

use essay_graphics_api::input::Input;
use essay_graphics_api::renderer::{Canvas, Renderer};
use essay_graphics_api::Bounds;

use crate::ui::cursor::ViewSizeCache;
use crate::ui::layers::GraphicsLayers;
use crate::ui::null_render::NullRenderer;
use crate::ui::style::UiStyle;
use crate::ui::ui2::{ResponseValue, Ui2};
use crate::ui::widget::{WidgetRect, WidgetRects};
use crate::ui::{Id, IdSet, Ui};

#[derive(Clone)]
pub struct Context(Arc<RwLock<ContextInner>>);

impl Context {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(ContextInner::default())))
    }

    fn read<R>(&self, reader: impl FnOnce(&ContextInner) -> R) -> R {
        let inner = self.0.read().unwrap();
        (reader)(inner.deref())
    }

    fn write<R>(&self, writer: impl FnOnce(&mut ContextInner) -> R) -> R {
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

    #[inline]
    pub fn style(&self) -> Arc<UiStyle> {
        self.read(|cxt| cxt.style.clone())
    }
}

impl Context {
    pub fn run_ui<R>(
        &self, 
        renderer: &mut dyn Renderer, 
        mut draw: impl FnMut(&mut Ui2) -> R + Send
    ) -> ResponseValue<R> {
        self.run(renderer, move |cxt, renderer| {
            let id = Id::new("top");

            Ui2::top(cxt, id, renderer, |ui2| {
                (draw)(ui2)
            })
        })
    }

    pub fn run<R>(
        &self, 
        renderer: &mut dyn Renderer, 
        mut draw: impl FnMut(&Self, &mut dyn Renderer) -> R + Send
    ) -> R {
        loop {
            let is_resize = self.write(|cxt| {
                if cxt.cache_pos != renderer.pos() {
                    cxt.cache_pos = renderer.pos();
                    true
                } else {
                    false
                }
            });

            self.start_pass(is_resize, renderer.input());

            let result = if is_resize {
                let mut null_render = NullRenderer(renderer);

                (draw)(self, &mut null_render)
            } else {
                (draw)(self, renderer)
            };

            if ! is_resize {
                self.graphics_mut(|layers| {
                    layers.render(renderer).unwrap();
                });

                return result;
            }
        }
    }

    fn start_pass(&self, _is_resize: bool, input: &Input) {
        self.write(|cxt| {
            let mut pass = RenderPass::default();
            std::mem::swap(&mut cxt.viewport.pass, &mut pass);
            std::mem::swap(&mut cxt.viewport.last_pass, &mut pass);

            cxt.viewport.hover.clear();

            if let Some(point) = input.cursor {

                for widget in cxt.viewport.last_pass.widgets.iter() {
                    if widget.rect.contains(point) {
                        cxt.viewport.hover.insert(widget.id);
                    }
                }
            }
        });
    }


    pub(crate) fn create_widget(&self, widget: WidgetRect) -> Response {
        self.write(|ctx| {
            ctx.viewport.pass.widgets.insert(widget);
        });

        self.get_response(widget)
    }

    pub(crate) fn get_response(&self, widget: WidgetRect) -> Response {
        let mut response = Response {
            id: widget.id,
            is_hover: false,
        };

        self.read(|cxt| {
            if cxt.viewport.hover.contains(widget.id) {
                response.is_hover = true;
            }
        });

        response
    }
}

#[derive(Default)]
pub(crate) struct ContextInner {
    viewport: Viewport,

    style: Arc<UiStyle>,

    cache_pos: Bounds<Canvas>,
}

#[derive(Default)]
pub struct Viewport {
    last_pass: RenderPass,
    pass: RenderPass,
    
    layers: GraphicsLayers,
    hover: WidgetHover,
}

#[derive(Default)]
pub struct RenderPass {
    widgets: WidgetRects,

    view_size: ViewSizeCache,
}

pub struct Response {
    id: Id,
    is_hover: bool,
}

impl Response {
    #[inline]
    pub fn id(&self) -> Id {
        self.id
    }

    #[inline]
    pub fn is_hover(&self) -> bool {
        self.is_hover
    }

    pub fn tooltip(&self, text: &str) {
        if self.is_hover {
            println!("Tooltip {:?}", text);
        }
    }
}

impl Default for Response {
    fn default() -> Self {
        Self { 
            id: Id::NULL,
            is_hover: Default::default() 
        }
    }
}

#[derive(Default)]
struct WidgetHover {
    hover: IdSet,
}

impl WidgetHover {
    fn clear(&mut self) {
        self.hover.clear();
    }

    fn contains(&self, id: Id) -> bool {
        self.hover.contains(&id)
    }

    fn insert(&mut self, id: Id) {
        self.hover.insert(id);
    }
}