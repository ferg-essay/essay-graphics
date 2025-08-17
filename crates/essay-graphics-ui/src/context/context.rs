use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};
use std::time::Instant;

use essay_graphics_api::input::Input;
use essay_graphics_api::output::Output;
use essay_graphics_api::renderer::{self, Canvas, FontSetMetrics, GraphicsContext, Renderer};
use essay_graphics_api::{Bounds, Point};

use crate::context::widget::{WidgetRect, WidgetRects};
use crate::painter::GraphicsLayers;
use crate::style::UiStyle;
use crate::ui::ui::{ResponseValue, Ui, UiBuilder};
use crate::ui::{Response};
use crate::util::{Id, IdSet};

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
    pub fn input<R>(&self, reader: impl FnOnce(&Input) -> R) -> R {
        self.read(|cxt| (reader)(&cxt.viewport.input))
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
    
    pub fn request_redraw_when(&self, _duration: f32) {
        
    }
}

impl Context {
    /*
    pub fn run_ui<R>(
        &self, 
        renderer: &mut dyn Renderer, 
        mut draw: impl FnMut(&mut Ui) -> R + Send
    ) -> ResponseValue<R> {
        self.run(renderer, move |cxt| {
            let id = Id::new("top");

            let builder = UiBuilder::default();
            
            Ui::top(cxt, id, builder, |ui2| {
                (draw)(ui2)
            })
        })
    }
    */

    pub fn run(
        &self, 
        renderer: &mut dyn Renderer, 
        mut draw: impl FnMut(&Self)
    ) -> renderer::Result<Output> {
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

            (draw)(self);

            if ! is_resize {
                self.graphics_mut(|layers| {
                    layers.render(renderer).unwrap();
                });

                return Ok(self.output());
            }
        }
    }

    fn output(&self) -> Output {
        Output::default()
    }

    fn start_pass(&self, _is_resize: bool, input: &Input) {
        self.write(|ctx| {
            let mut pass = RenderPass::default();
            std::mem::swap(&mut ctx.viewport.pass, &mut pass);
            std::mem::swap(&mut ctx.viewport.last_pass, &mut pass);

            ctx.viewport.input = input.clone(); // TODO: transfer input
            ctx.viewport.interact.clicked = None;
            ctx.viewport.hover.clear();

            if let Some(point) = input.cursor {
                if ctx.viewport.interact.cursor != input.cursor {
                    ctx.viewport.interact.last_cursor_move = Instant::now();
                }

                for widget in ctx.viewport.last_pass.widgets.iter() {
                    if widget.rect.contains(point) {
                        ctx.viewport.hover.insert(widget.id);

                        if input.left.click {
                            ctx.viewport.interact.clicked = Some(widget.id);
                        }
                    }
                }
            }

            ctx.viewport.interact.cursor = input.cursor;
        });
    }


    pub(crate) fn create_widget(&self, widget: WidgetRect) -> Response {
        self.write(|ctx| {
            ctx.viewport.pass.widgets.insert(widget);
        });

        Response::new(&self, widget)
    }
}

pub(crate) struct ContextInner {
    _graphics_context: Box<dyn GraphicsContext>,

    fonts: Fonts,

    viewport: Viewport,

    style: Arc<UiStyle>,
}

impl ContextInner {
    pub fn new(graphics_context: Box<dyn GraphicsContext>) -> Self {
        let default_font_set = graphics_context.default_font_set();

        Self {
            _graphics_context: graphics_context,
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

    input: Input,
    pub interact: Interact,
    pub(crate) hover: WidgetHover,
}

pub struct Interact {
    pub cursor: Option<Point>,
    
    pub clicked: Option<Id>,

    last_cursor_move: Instant,
}

impl Interact {
    pub fn since_cursor_move(&self) -> f32 {
        Instant::now()
            .duration_since(self.last_cursor_move)
            .as_secs_f32()
    }
}

impl Default for Interact {
    fn default() -> Self {
        Self { 
            cursor: Default::default(), 
            clicked: Default::default(), 
            last_cursor_move: Instant::now(),
        }
    }
}

#[derive(Default)]
pub struct RenderPass {
    widgets: WidgetRects,

    // view_size: ViewSizeCache,
}

impl RenderPass {
    pub fn widgets(&self) -> &WidgetRects {
        &self.widgets
    }
}

pub struct Fonts {
    pub default_font_set: Box<dyn FontSetMetrics>,
}


#[derive(Default)]
pub(crate) struct WidgetHover {
    hover: IdSet,
}

impl WidgetHover {
    fn clear(&mut self) {
        self.hover.clear();
    }

    pub fn contains(&self, id: Id) -> bool {
        self.hover.contains(&id)
    }

    fn insert(&mut self, id: Id) {
        self.hover.insert(id);
    }
}