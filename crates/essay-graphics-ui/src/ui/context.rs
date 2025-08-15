use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

use essay_graphics_api::input::Input;
use essay_graphics_api::renderer::{Canvas, FontSetMetrics, GraphicsContext, Renderer};
use essay_graphics_api::Bounds;

use crate::ui::cursor::ViewSizeCache;
use crate::ui::layers::GraphicsLayers;
use crate::ui::null_render::NullRenderer;
use crate::ui::style::UiStyle;
use crate::ui::tooltip::Tooltip;
use crate::ui::ui2::{ResponseValue, Ui2, UiBuilder};
use crate::ui::widget::{WidgetRect, WidgetRects};
use crate::ui::{Id, IdSet};

#[derive(Clone)]
pub struct Context(Arc<RwLock<ContextInner>>);

impl Context {
    pub fn new(graphics_context: Box<dyn GraphicsContext>) -> Self {
        Self(Arc::new(RwLock::new(ContextInner::new(graphics_context))))
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
    pub fn viewport<R>(&self, reader: impl FnOnce(&Viewport) -> R) -> R {
        self.read(|cxt| (reader)(&cxt.viewport))
    }

    #[inline]
    pub fn viewport_mut<R>(&self, writer: impl FnOnce(&mut Viewport) -> R) -> R {
        self.write(|cxt| (writer)(&mut cxt.viewport))
    }

    #[inline]
    pub fn fonts<R>(&self, reader: impl FnOnce(&Fonts) -> R) -> R {
        self.read(|cxt| (reader)(&cxt.fonts))
    }

    #[inline]
    pub fn fonts_mut<R>(&self, writer: impl FnOnce(&mut Fonts) -> R) -> R {
        self.write(|cxt| (writer)(&mut cxt.fonts))
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

    #[inline]
    pub fn screen_pos(&self) -> Bounds<Canvas> {
        self.viewport(|viewport| viewport.screen_pos)
    }
}

impl Context {
    pub fn run_ui<R>(
        &self, 
        renderer: &mut dyn Renderer, 
        mut draw: impl FnMut(&mut Ui2) -> R + Send
    ) -> ResponseValue<R> {
        self.run(renderer, move |cxt| {
            let id = Id::new("top");

            let builder = UiBuilder::default();
            
            Ui2::top(cxt, id, builder, |ui2| {
                (draw)(ui2)
            })
        })
    }

    pub fn run<R>(
        &self, 
        renderer: &mut dyn Renderer, 
        mut draw: impl FnMut(&Self) -> R + Send
    ) -> R {
        loop {
            let is_resize = self.write(|cxt| {
                if cxt.viewport.screen_pos != renderer.pos() {
                    cxt.viewport.screen_pos = renderer.pos();
                    true
                } else {
                    false
                }
            });

            self.start_pass(is_resize, renderer.input());

            let result = (draw)(self);

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
            ctx: self.clone(),
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

pub(crate) struct ContextInner {
    graphics_context: Box<dyn GraphicsContext>,

    fonts: Fonts,

    viewport: Viewport,

    style: Arc<UiStyle>,
}

impl ContextInner {
    pub fn new(graphics_context: Box<dyn GraphicsContext>) -> Self {
        let default_font_set = graphics_context.default_font_set();

        Self {
            graphics_context,
            fonts: Fonts {
                default_font_set,
            },

            viewport: Viewport::default(),
            style: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct Viewport {
    last_pass: RenderPass,
    pass: RenderPass,

    screen_pos: Bounds<Canvas>,
    
    layers: GraphicsLayers,
    hover: WidgetHover,
}

#[derive(Default)]
pub struct RenderPass {
    widgets: WidgetRects,

    view_size: ViewSizeCache,
}

impl RenderPass {
    pub fn widgets(&self) -> &WidgetRects {
        &self.widgets
    }
}

pub struct Fonts {
    pub default_font_set: Box<dyn FontSetMetrics>,
}

pub struct Response {
    pub id: Id,
    pub ctx: Context,
    is_hover: bool,
}

impl Response {
    #[inline]
    pub fn id(&self) -> Id {
        self.id
    }

    #[inline]
    pub fn context(&self) -> &Context {
        &self.ctx
    }

    #[inline]
    pub fn is_hover(&self) -> bool {
        self.is_hover
    }

    pub fn on_hover_ui(&self, add_contents: impl FnOnce(&mut Ui2)) -> &Self {
        if self.is_hover() {
            Tooltip::for_enabled(&self).show(add_contents);
        }

        &self
    }
    
    pub fn onclick(&self, _there: impl FnMut()) {
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