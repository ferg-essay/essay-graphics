use core::hash;
use std::{ops::{self, Deref}, sync::Arc};

use essay_graphics_api::{
    input::Input, 
    renderer::{self, Canvas, Drawable, Renderer}, 
    Bounds, Point, Size, TextStyle
};

use crate::ui::{
    button::{Button}, Response, label::{Label}, style::UiStyle, widget::WidgetRect, Context, Id, Painter
};

use super::cursor::{Cursor, CursorUpdate, ViewSizeCache};

pub struct Ui {
    id: Id,
    unique_id: Id,
    next_auto_id_salt: u64,
    
    cursor: Cursor,
    update: CursorUpdate,

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
        cxt: &Context,
        id: Id,
        builder: UiBuilder,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let UiBuilder {
            max_bounds,
            ..
        } = builder;

        let pos = max_bounds.unwrap_or_else(|| {
            cxt.screen_pos()
        });

        let page = Bounds::unit();

        let cursor = Cursor::new(pos, page);
        
        let mut ui = Ui {
            id,
            unique_id: id,
            next_auto_id_salt: id.with("auto").value(),
            cursor,
            update: CursorUpdate::Vertical,
            painter: Painter::new(&cxt),
            style: cxt.style(),
    
            // prev_cache,
            // next_cache,
            cache_index: 0,
        };

        let start_rect = Bounds::none();
        ui.context().create_widget(WidgetRect {
            id: ui.unique_id,
            rect: start_rect,
        });
    
        let result = (add_content)(&mut ui);
    
        // next_cache.page = ui.cursor.page_allocated;

        let response = ui.end();

        ResponseValue::new(result, response)
    }
    
    pub fn child<R>(
        &mut self,
        builder: UiBuilder,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let UiBuilder {
            id_salt,
            max_bounds,
            update,
        } = builder;
        
        let id_salt = id_salt.unwrap_or_else(|| Id::from("child"));
        let stable_id = self.id.with(id_salt);
        let unique_id = stable_id.with(self.next_auto_id_salt);
        self.next_auto_id_salt = self.next_auto_id_salt.wrapping_add(1);
        let next_auto_id_salt = unique_id.value().wrapping_add(1);

        let max_bounds = max_bounds.unwrap_or_else(|| {
            let pos = self.cursor.canvas_pos;
            let extent = self.cursor.canvas_extent;

            Bounds::from([
                [pos.x(), extent.ymin()],
                [extent.xmax(), pos.y()]
            ])
        });

        let update = update.unwrap_or_else(|| self.update);

        let bounds = Bounds::none();
        let response = self.context().create_widget(WidgetRect {
            id: unique_id,
            rect: bounds,
        });

        let mut child = Ui {
            id: stable_id,
            unique_id,
            next_auto_id_salt,
            cursor: self.cursor.child(Point(max_bounds.xmin(), max_bounds.ymax())),
            update,
            painter: Painter::new(self.painter.context()),
            style: self.style.clone(),
            // prev_cache: self.prev_cache,
            // next_cache: self.next_cache,
            cache_index: self.cache_index,
        };

        let result = (add_content)(&mut child);

        self.cursor.merge_child(&child.cursor);

        let response = child.end();

        ResponseValue::new(result, response)
    }

    fn end(&mut self) -> Response {
        let bounds = self.cursor.canvas_allocated;
        let response = self.context().create_widget(WidgetRect {
            id: self.unique_id,
            rect: bounds,
        });

        response
    }
    
    fn child_view<R>(
        &mut self,
        bounds: Bounds<Canvas>, 
        update: CursorUpdate,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> R {
        todo!();
        /*
        let child_cache = self.prev_cache
            .map(|cache| cache.get(self.cache_index))
            .unwrap_or(None);

        let child_cache = None;

        let mut child = Ui2 {
            unique_id: self.unique_id,
            cursor: self.cursor.child_view(
                bounds,
                child_cache
                    .map(|cache: &ViewSizeCache| cache.page)
                    .unwrap_or(Bounds::unit()),
                child_cache
                    .map(|cache| cache.canvas)
                    .unwrap_or(Bounds::zero()),
            ),
            renderer: self.renderer,
            update,
            painter: Painter::new(self.painter.context()),
            // prev_cache: child_cache,
            // next_cache: self.next_cache.push(self.cache_index),
            cache_index: self.cache_index + 1,
        };

        let result = (add_content)(&mut child);

        self.cache_index = child.cache_index;
        // child.next_cache.canvas = child.cursor.fixed_allocated;
        // child.next_cache.page = child.cursor.page_allocated;

        result
        */
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
        self.cursor.available_bounds()
    }

    #[inline]
    pub fn allocate_rect(&mut self, size: Size) -> ResponseValue<Bounds<Canvas>> {
        let pos = self.update.alloc_canvas(size, &mut self.cursor);

        self.alloc_response(pos)
    }
    
    #[inline]
    pub fn allocate_page(&mut self, size: Size) -> ResponseValue<Bounds<Canvas>> {
        let pos = self.update.alloc_page(size, &mut self.cursor);

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
        self.cursor.canvas_free()
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

    pub fn view(&mut self, draw: impl Drawable + 'static) -> Response {
        self.draw_size(Size(1., 1.), draw)
    }

    pub fn draw_size(&mut self, size: Size, draw: impl Drawable + 'static) -> Response {
        let ResponseValue { response, .. } = self.allocate_page(size);
        
        self.painter_mut().add(draw);
        // self.renderer().draw_with(value, Box::new(|ui| draw.draw(ui))).unwrap();
        //todo!();

        response
    }

    pub fn horizontal<R>(&mut self, add_content: impl FnOnce(&mut Ui) -> R) -> ResponseValue<R> {
        let pos = self.cursor.canvas_pos;
        let extent = self.cursor.canvas_extent;

        let bounds = Bounds::from([
            [pos.x(), extent.ymin()],
            [extent.xmax(), pos.y()]
        ]);

        let result = self.child(UiBuilder::default()
            .max_bounds(bounds)
            .update(CursorUpdate::Horizontal),
            add_content
        );

        self.cursor.canvas_pos = Point(self.cursor.canvas_pos.x(), self.cursor.canvas_allocated.ymin());

        result
    }

    pub fn vertical<R>(&mut self, add_content: impl FnOnce(&mut Ui) -> R) -> ResponseValue<R> {
        let pos = self.cursor.canvas_pos;
        let extent = self.cursor.canvas_extent;
        let bounds = Bounds::<Canvas>::from((
            [pos.x(), extent.ymin()],
            [extent.xmax() - pos.x(), pos.y() - extent.ymin()]
        ));

        let result = self.child(UiBuilder::default()
            .max_bounds(bounds)
            .update(CursorUpdate::Vertical),
            add_content
        );

        self.cursor.canvas_pos = Point(self.cursor.canvas_allocated.xmax(), self.cursor.canvas_pos.y());
        self.cursor.page_pos = Point(self.cursor.page_allocated.xmax(), self.cursor.page_pos.y());

        result
    }

    pub fn horizontal_view<R>(
        &mut self, 
        size: UiSize, 
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let bounds = match size {
            UiSize::Canvas(width, height) => {
                self.update.alloc_canvas(
                    Size(width, height), 
                    &mut self.cursor
                )
            }
            UiSize::Page(width, height) => {
                self.update.alloc_page(
                    Size(width, height), 
                    &mut self.cursor
                )
            }
        };

        let result = self.child(UiBuilder::default()
            .max_bounds(bounds)
            .update(CursorUpdate::Horizontal),
            add_content
        );

        self.cursor.canvas_pos = Point(self.cursor.canvas_allocated.xmax(), self.cursor.canvas_pos.y());
        self.cursor.page_pos = Point(self.cursor.page_allocated.xmax(), self.cursor.page_pos.y());

        result
    }

    pub fn vertical_view<R>(
        &mut self, 
        size: UiSize, 
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let bounds = match size {
            UiSize::Canvas(width, height) => {
                self.update.alloc_canvas(
                    Size(width, height), 
                    &mut self.cursor
                )
            }
            UiSize::Page(width, height) => {
                self.update.alloc_page(
                    Size(width, height), 
                    &mut self.cursor
                )
            }
        };

        let result = self.child(UiBuilder::default()
            .max_bounds(bounds)
            .update(CursorUpdate::Vertical),
            add_content
        );

        self.cursor.canvas_pos = Point(self.cursor.canvas_allocated.xmax(), self.cursor.canvas_pos.y());
        self.cursor.page_pos = Point(self.cursor.page_allocated.xmax(), self.cursor.page_pos.y());

        result
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
                height = rect.height.max(height);
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
pub struct UiBuilder {
    id_salt: Option<Id>,
    max_bounds: Option<Bounds<Canvas>>,
    update: Option<CursorUpdate>
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
    pub fn update(mut self, update: CursorUpdate) -> Self {
        self.update = Some(update);

        self
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UiSize {
    Canvas(f32, f32),
    Page(f32, f32),
}

/*
pub struct Response {
    onclick: bool,
}

impl Response {
    pub fn with_onclick(mut self, onclick: bool) -> Self {
        self.onclick = onclick;

        self
    }

    pub fn onclick(&self, fun: impl FnOnce()) -> &Self {
        if self.onclick {
            (fun)();
        }

        self
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            onclick: false,
        }
    }
}
    */


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
