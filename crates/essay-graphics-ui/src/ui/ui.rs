use core::hash;
use std::{ops, sync::Arc};

use essay_graphics_api::{
    input::Input, 
    renderer::{self, Canvas, Drawable, Renderer}, 
    Bounds, Point, Size, TextStyle
};

use crate::{context::{Context, WidgetRect}, painter::Painter, style::UiStyle, ui::Response, util::Id, widgets::{Button, Label}};

use super::alloc::{Alloc, AllocUpdate};

pub struct Ui {
    id: Id,
    unique_id: Id,
    next_auto_id_salt: u64,
    
    alloc: Alloc,
    update: AllocUpdate,

    painter: Painter,
    style: Arc<UiStyle>,
    // prev_cache: Option<&'a ViewSizeCache>,
    // next_cache: &'a mut ViewSizeCache,
    cache_index: usize,
}

impl Ui {
    #[inline]
    pub fn style(&self) -> &UiStyle {
        &self.style
    }

    pub(crate) fn top<R>(
        ctx: &Context,
        id: Id,
        builder: UiBuilder,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let UiBuilder {
            max_bounds,
            ..
        } = builder;

        let canvas = max_bounds.unwrap_or_else(|| {
            ctx.screen_pos()
        });

        let alloc_cache = ctx.last_pass(|pass| pass.alloc_map.get(&id).cloned());

        // println!("TopCache {:?}", alloc_cache);

        let cursor = Alloc::new(canvas, alloc_cache.clone());
        
        let mut ui = Ui {
            id,
            unique_id: id,
            next_auto_id_salt: id.with("auto").value(),
            alloc: cursor,
            update: AllocUpdate::Vertical,
            painter: Painter::new(&ctx),
            style: ctx.style(),
    
            cache_index: 0,
        };

        let start_rect = Bounds::none();
        ui.context().create_widget(WidgetRect {
            id: ui.unique_id,
            rect: start_rect,
        });
    
        let result = (add_content)(&mut ui);
    
        let response = ui.end();

        let new_alloc = ui.alloc.to_cache();
        if new_alloc.is_changed(&alloc_cache) {
            println!("AllocChangeTop")
        }

        ctx.pass_mut(|pass| {
            pass.alloc_map.insert(id, new_alloc);
        });

        ResponseValue::new(result, response)
    }
    
    pub(crate) fn child<R>(
        &mut self,
        builder: UiBuilder,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let UiBuilder {
            id_salt,
            max_bounds,
            update: alloc_update,
            is_view,
        } = builder;
        
        let id_salt = id_salt.unwrap_or_else(|| Id::from("child"));
        let stable_id = self.id.with(id_salt);
        let unique_id = stable_id.with(self.next_auto_id_salt);
        self.next_auto_id_salt = self.next_auto_id_salt.wrapping_add(1);
        let next_auto_id_salt = unique_id.value().wrapping_add(1);

        let max_bounds = max_bounds.unwrap_or_else(|| {
            let pos = self.alloc.pos;
            let extent = self.alloc.canvas_extent;

            Bounds::from([
                [pos.x(), extent.ymin()],
                [extent.xmax(), pos.y()]
            ])
        });

        let update = alloc_update.unwrap_or_else(|| self.update);

        let bounds = Bounds::none();
        self.context().create_widget(WidgetRect {
            id: unique_id,
            rect: bounds,
        });

        let alloc_cache = self.context().last_pass(|pass| {
            pass.alloc_map.get(&unique_id).cloned()
        });

        let alloc = self.alloc.child(
            Point(max_bounds.xmin(), max_bounds.ymin()),
            self.update,
            alloc_cache.clone()
        );

        let mut child = Ui {
            id: stable_id,
            unique_id,
            next_auto_id_salt,
            alloc,
            update,
            painter: Painter::new(self.painter.context()),
            style: self.style.clone(),
            cache_index: self.cache_index,
        };

        let result = (add_content)(&mut child);

        self.alloc.merge_child(&child.alloc, self.update, is_view);

        let response = child.end();

        let new_alloc = child.alloc.to_cache();
        if new_alloc.is_changed(&alloc_cache) {
            println!("AllocChange")
        }

        self.context().pass_mut(|pass| {
            pass.alloc_map.insert(unique_id, new_alloc);
        });

        ResponseValue::new(result, response)
    }

    fn end(&mut self) -> Response {
        let bounds = self.alloc.canvas_allocated;
        let response = self.context().create_widget(WidgetRect {
            id: self.unique_id,
            rect: bounds,
        });

        response
    }

    #[inline]
    pub fn context(&self) -> &Context {
        self.painter.context()
    }

    #[inline]
    pub fn painter_mut(&mut self) -> &mut Painter {
        &mut self.painter
    }

    pub fn available_bounds(&self) -> Bounds<Canvas> {
        self.alloc.available_bounds()
    }

    #[inline]
    pub fn allocate_rect(&mut self, size: Size) -> ResponseValue<Bounds<Canvas>> {
        let pos = self.update.alloc_canvas(size, &mut self.alloc);

        self.alloc_response(pos)
    }
    
    #[inline]
    pub fn allocate_view(&mut self, size: Size) -> ResponseValue<Bounds<Canvas>> {
        let pos = self.update.alloc_view(size, &mut self.alloc);

        self.alloc_response(pos)
    }

    pub fn alloc_response(&mut self, pos: Bounds<Canvas>) -> ResponseValue<Bounds<Canvas>> {
        let id = self.id.with(self.next_auto_id_salt);
        self.next_auto_id_salt = self.next_auto_id_salt.wrapping_add(1);

        let widget = WidgetRect {
            id,
            rect: pos,
        };

        let response = self.context().create_widget(widget);

        ResponseValue::new(pos, response)
    }

    #[inline]
    pub fn remaining_size(&mut self) -> Size {
        self.alloc.canvas_free()
    }

    #[inline]
    pub fn add(&mut self, mut widget: impl Widget) -> Response {
        widget.ui(self)
    }

    #[inline]
    pub fn label(&mut self, label: &str) -> Response {
        let label = Label::new(label);

        self.add(label)
    }

    #[inline]
    pub fn button(&mut self, label: &str, press: bool) -> Response {
        let button = Button::new(label, press);

        self.add(button)
    }

    pub fn draw(&mut self, draw: impl Drawable + 'static) -> Response {
        self.draw_size(Size(1., 1.), draw)
    }

    pub fn draw_size(&mut self, size: Size, draw: impl Drawable + 'static) -> Response {
        let ResponseValue { response, .. } = self.allocate_view(size);
        
        self.painter_mut().add(draw);

        response
    }

    pub fn horizontal<R>(&mut self, add_content: impl FnOnce(&mut Ui) -> R) -> ResponseValue<R> {
        let pos = self.alloc.pos;
        let extent = self.alloc.canvas_extent;

        let bounds = Bounds::from([
            [pos.x(), pos.y()],
            [extent.xmax(), extent.ymax()]
        ]);

        let result = self.child(UiBuilder::default()
            .max_bounds(bounds)
            .update(AllocUpdate::Horizontal),
            add_content
        );

        self.update_pos();

        result
    }

    pub fn vertical<R>(&mut self, add_content: impl FnOnce(&mut Ui) -> R) -> ResponseValue<R> {
        let pos = self.alloc.pos;
        let extent = self.alloc.canvas_extent;
        let bounds = Bounds::<Canvas>::from((
            [pos.x(), pos.y()],
            [extent.xmax(), extent.ymax()]
        ));

        let result = self.child(UiBuilder::default()
            .max_bounds(bounds)
            .update(AllocUpdate::Vertical),
            add_content
        );

        self.update_pos();

        result
    }

    pub fn view<R>(
        &mut self, 
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let pos = self.update.alloc_view(Size(1., 1.), &mut self.alloc);

        let result = self.child(
            UiBuilder::default()
                .max_bounds(pos)
                .view(true)
                .update(AllocUpdate::Vertical),
            add_content
        );

        self.update_pos();

        result
    }

    pub fn horizontal_size<R>(
        &mut self, 
        size: UiSize, 
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let mut is_view = false;

        let bounds = match size {
            UiSize::Canvas(width, height) => {
                self.update.alloc_canvas(
                    Size(width, height), 
                    &mut self.alloc
                )
            }
            UiSize::View(width, height) => {
                is_view = true;
                self.update.alloc_view(
                    Size(width, height), 
                    &mut self.alloc
                )
            }
        };

        let result = self.child(UiBuilder::default()
            .max_bounds(bounds)
            .view(is_view)
            .update(AllocUpdate::Horizontal),
            add_content
        );

        self.update_pos();

        result
    }

    pub fn vertical_size<R>(
        &mut self, 
        size: UiSize, 
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let mut is_view = false;

        let bounds = match size {
            UiSize::Canvas(width, height) => {
                self.update.alloc_canvas(
                    Size(width, height), 
                    &mut self.alloc
                )
            }
            UiSize::View(width, height) => {
                is_view = true;
                self.update.alloc_view(
                    Size(width, height), 
                    &mut self.alloc
                )
            }
        };

        let result = self.child(UiBuilder::default()
            .max_bounds(bounds)
            .view(is_view)
            .update(AllocUpdate::Vertical),
            add_content
        );

        self.update_pos();

        result
    }

    fn update_pos(&mut self) {
        match self.update {
            AllocUpdate::Vertical => {
                self.alloc.pos = Point(self.alloc.pos.x(), self.alloc.canvas_allocated.ymax());
                self.alloc.view_pos = Point(self.alloc.view_pos.x(), self.alloc.view_allocated.ymax());
            },
            AllocUpdate::Horizontal => {
                self.alloc.pos = Point(self.alloc.canvas_allocated.xmax(), self.alloc.pos.y());
                self.alloc.view_pos = Point(self.alloc.view_allocated.xmax(), self.alloc.view_pos.y());
            },
        }
    }
    
    #[inline]
    pub fn text_size(&mut self, label: &str, style_text: &TextStyle) -> Size {
        self.context().fonts_mut(|fonts| {
            let fonts = &mut fonts.default_font_set;

            let mut width = 0.;
            let mut height = 0.;

            let size = style_text.get_size().unwrap_or(10.);
            let size = 4. / 3. * 2. * size; // ppt

            for ch in label.chars() {
                let rect = fonts.glyph_size(size, ch);

                width += rect.width;
                height = (rect.ascent + rect.descent).max(height);
            }

            Size(width, height)
        })
        // let len = label.len();
        // let pt = style_text.get_size().unwrap_or(10.);

        // TODO:
    }

    #[inline]
    pub fn input<R>(&self, reader: impl FnOnce(&Input) -> R) -> R {
        self.context().input(reader)
    }
}

#[derive(Default)]
pub(crate) struct UiBuilder {
    id_salt: Option<Id>,
    max_bounds: Option<Bounds<Canvas>>,
    update: Option<AllocUpdate>,
    is_view: bool,
}

impl UiBuilder {
    #[inline]
    pub fn id_salt(mut self, hash: impl hash::Hash) -> Self {
        self.id_salt = Some(Id::new(hash));

        self
    }

    #[inline]
    pub fn max_bounds(mut self, bounds: impl Into<Bounds<Canvas>>) -> Self {
        self.max_bounds = Some(bounds.into());

        self
    }

    #[inline]
    pub(crate) fn update(mut self, update: AllocUpdate) -> Self {
        self.update = Some(update);

        self
    }

    #[inline]
    pub fn view(mut self, is_view: bool) -> Self {
        self.is_view = is_view;

        self
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UiSize {
    Canvas(f32, f32),
    View(f32, f32),
}

pub struct OnceView<T: Drawable> {
    view: Option<T>,
}

impl<T: Drawable> OnceView<T> {
    #[inline]
    pub fn get(&self) -> Option<&T> {
        self.view.as_ref()
    } 

    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.view.as_mut()
    } 

    #[inline]
    pub fn get_or_init(&mut self, draw: T) -> &T {
        if self.view.is_none() {
            self.view = Some(draw);
        }

        self.view.as_ref().unwrap()
    } 

    #[inline]
    pub fn get_or_init_mut(&mut self, draw: T) -> &mut T {
        if self.view.is_none() {
            self.view = Some(draw);
        }

        self.view.as_mut().unwrap()
    } 
}

impl<T: Drawable> Drawable for OnceView<T> {
    fn draw(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        if let Some(view) = &mut self.view {
            view.draw(ui)
        } else {
            Ok(())
        }
    }
}


pub trait Widget {
    fn ui(
        &mut self, 
        ui: &mut Ui,
    ) -> Response;
}

pub struct ResponseValue<T> {
    pub value: T,
    pub response: Response,
}

impl<T> ResponseValue<T> {
    pub(crate) fn new(value: T, response: Response) -> Self {
        Self {
            value,
            response
        }
    }

    pub fn response(&self) -> &Response {
        &self.response
    }

    pub fn response_mut(&mut self) -> &mut Response {
        &mut self.response
    }
}

impl<T> ops::Deref for ResponseValue<T> {
    type Target = Response;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

impl<T> ops::DerefMut for ResponseValue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.response
    }
}
