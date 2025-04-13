use essay_graphics_api::{
    input::Input, 
    renderer::{self, Canvas, Drawable, Renderer}, 
    Bounds, Point, Size, TextStyle
};

use crate::ui::{
    button::Button, 
    label::Label, 
    style::UiStyle,
};

use super::cursor::{Cursor, CursorUpdate, ViewSizeCache};

pub struct Ui<'a> {
    renderer: &'a mut dyn Renderer,
    style: &'a UiStyle,
    cursor: Cursor,
    update: CursorUpdate,

    prev_cache: Option<&'a ViewSizeCache>,
    next_cache: &'a mut ViewSizeCache,
    cache_index: usize,
}

impl<'a> Ui<'a> {
    fn top<R>(
        renderer: &mut dyn Renderer,
        style: &UiStyle,
        prev_cache: Option<&ViewSizeCache>,
        next_cache: &mut ViewSizeCache,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> R {
        let page = prev_cache
            .map(|cache| cache.page)
            .unwrap_or(Bounds::unit());

        let cursor = Cursor::new(renderer.pos(), page);
        
        let mut ui = Ui {
            cursor,
            renderer,
            style,
            update: CursorUpdate::Vertical,
    
            prev_cache,
            next_cache,
            cache_index: 0,
        };
    
        let result = (add_content)(&mut ui);
    
        next_cache.page = ui.cursor.page_allocated;

        result
    }
    
    fn child<R>(
        &mut self,
        bounds: Bounds<Canvas>, 
        update: CursorUpdate,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> R {
        let mut child = Ui {
            cursor: self.cursor.child(Point(bounds.xmin(), bounds.ymax())),
            renderer: self.renderer,
            style: self.style,
            update,
            prev_cache: self.prev_cache,
            next_cache: self.next_cache,
            cache_index: self.cache_index,
        };

        let result = (add_content)(&mut child);

        self.cursor.merge_child(&child.cursor);

        result
    }
    
    fn child_view<R>(
        &mut self,
        bounds: Bounds<Canvas>, 
        update: CursorUpdate,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> R {
        let child_cache = self.prev_cache
            .map(|cache| cache.get(self.cache_index))
            .unwrap_or(None);

        let mut child = Ui {
            cursor: self.cursor.child_view(
                bounds,
                child_cache
                    .map(|cache| cache.page)
                    .unwrap_or(Bounds::unit()),
                child_cache
                    .map(|cache| cache.canvas)
                    .unwrap_or(Bounds::zero()),
            ),
            renderer: self.renderer,
            style: self.style,
            update,
            prev_cache: child_cache,
            next_cache: self.next_cache.push(self.cache_index),
            cache_index: self.cache_index + 1,
        };

        let result = (add_content)(&mut child);

        self.cache_index = child.cache_index;
        child.next_cache.canvas = child.cursor.fixed_allocated;
        child.next_cache.page = child.cursor.page_allocated;

        result
    }

    #[inline]
    pub fn renderer(&mut self) -> &mut dyn Renderer {
        self.renderer
    }

    #[inline]
    pub fn style(&mut self) -> &UiStyle {
        &self.style
    }

    #[inline]
    pub fn allocate_rect(&mut self, size: Size) -> Bounds<Canvas> {
        self.update.alloc_canvas(size, &mut self.cursor)
    }
    
    #[inline]
    pub fn allocate_page(&mut self, size: Size) -> Bounds<Canvas> {
        self.update.alloc_page(size, &mut self.cursor)
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

    pub fn view<'b, T: Drawable>(&mut self, draw: &'b mut T) -> Response {
        self.draw_size(Size(1., 1.), draw)
    }

    pub fn draw_size(&mut self, size: Size, draw: &mut dyn Drawable) -> Response {
        let rect = self.allocate_page(size);
        
        self.renderer().draw_with(rect, Box::new(|ui| draw.draw(ui))).unwrap();

        Response::default()
    }

    pub fn horizontal<R>(&mut self, add_content: impl FnOnce(&mut Ui) -> R) -> R {
        let pos = self.cursor.canvas_pos;
        let extent = self.cursor.canvas_extent;

        let bounds = Bounds::from([
            [pos.x(), extent.ymin()],
            [extent.xmax(), pos.y()]
        ]);

        let result = self.child(bounds, CursorUpdate::Horizontal, add_content);

        self.cursor.canvas_pos = Point(self.cursor.canvas_pos.x(), self.cursor.canvas_allocated.ymin());

        result
    }

    pub fn vertical<R>(&mut self, add_content: impl FnOnce(&mut Ui) -> R) -> R {
        let pos = self.cursor.canvas_pos;
        let extent = self.cursor.canvas_extent;
        let bounds = Bounds::<Canvas>::from((
            [pos.x(), extent.ymin()],
            [extent.xmax() - pos.x(), pos.y() - extent.ymin()]
        ));

        let result = self.child(bounds, CursorUpdate::Vertical, add_content);

        self.cursor.canvas_pos = Point(self.cursor.canvas_allocated.xmax(), self.cursor.canvas_pos.y());
        self.cursor.page_pos = Point(self.cursor.page_allocated.xmax(), self.cursor.page_pos.y());

        result
    }

    pub fn horizontal_view<R>(
        &mut self, 
        size: UiSize, 
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> R {
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

        self.child_view(bounds, CursorUpdate::Horizontal, add_content)
    }

    pub fn vertical_view<R>(
        &mut self, 
        size: UiSize, 
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> R {
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

        self.child_view(bounds, CursorUpdate::Vertical, add_content)
    }
    
    #[inline]
    pub fn text_size(&mut self, label: &str, style_text: &TextStyle) -> Size {
        self.renderer.text_size(label, style_text)
    }
    
    #[inline]
    pub fn input(&self) -> &Input {
        self.renderer.input()
    }
}

pub(super) fn draw_top<'a, R>(
    prev_cache: &ViewSizeCache, 
    renderer: &'a mut dyn Renderer, 
    style: &UiStyle,
    add_content: &'a mut dyn FnMut(&mut Ui) -> R
) -> (R, ViewSizeCache) {
    let mut next_cache = ViewSizeCache::new();
    
    let result = Ui::top(renderer, style, Some(prev_cache), &mut next_cache, add_content);

    (result, next_cache)
}

#[derive(Copy, Clone, Debug)]
pub enum UiSize {
    Canvas(f32, f32),
    Page(f32, f32),
}

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
