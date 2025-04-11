use essay_graphics_api::{
    input::Input, renderer::{self, Canvas, Drawable, Renderer}, Bounds, Point, Size, TextStyle
};

use crate::{page::Page, ui::{
    button::Button, 
    label::Label, 
    style::UiStyle,
}};

use super::{cursor::{Cursor, CursorTop, CursorUpdate, ViewSizeId, ViewSizeCache}, UiView};

pub struct Ui<'a> {
    renderer: &'a mut dyn Renderer,
    style: &'a UiStyle,
    cursor: Cursor,
    update: CursorUpdate,

    state: &'a ViewSizeCache,
}

pub struct UiTop<'a> {
    renderer: &'a mut dyn Renderer,
    style: &'a UiStyle,
    top: CursorTop,
}

impl<'a> Ui<'a> {
    pub(super) fn new(
        renderer: &'a mut dyn Renderer,
        style: &'a UiStyle,
        cursor: Cursor,

        state: &'a ViewSizeCache,
    ) -> Self {
        Self {
            cursor,
            renderer,
            style,
            update: CursorUpdate::Vertical,
            state,
        }
    }
    
    fn child(
        &mut self,
        bounds: Bounds<Canvas>, 
        update: CursorUpdate,
        add_content: impl FnOnce(&mut Ui)
    ) {
        let mut child = Ui {
            cursor: self.cursor.child(Point(bounds.xmin(), bounds.ymax())),
            renderer: self.renderer,
            style: self.style,
            update,
            state: self.state,
        };

        (add_content)(&mut child);

        self.cursor.merge_child(&child.cursor);
    }
    
    fn child_view(
        &mut self,
        bounds: Bounds<Canvas>, 
        update: CursorUpdate,
        add_content: impl FnOnce(&mut Ui)
    ) {
        let mut child = Ui {
            cursor: self.cursor.child(Point(bounds.xmin(), bounds.ymax())),
            renderer: self.renderer,
            style: self.style,
            update,
            state: self.state,
        };

        (add_content)(&mut child);

        // self.cursor.merge_child(&child.cursor);
    }

    #[inline]
    pub fn draw<R>(
        renderer: &'a mut dyn Renderer,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> R {
        todo!();
        /*
        let cursor = Cursor::new(renderer.pos().clone());

        let mut ui = Ui::new(renderer, cursor);

        (add_content)(&mut ui)
        */
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

    pub fn view<T: Drawable>(&mut self, view: &mut OnceView<T>, draw: T) -> Response {
        view.get_or_init_mut(draw).draw(self.renderer()).unwrap();

        Response::default()
    }

    pub fn horizontal(&mut self, add_content: impl FnOnce(&mut Ui)) -> &mut Self {
        let pos = self.cursor.canvas_pos;
        let extent = self.cursor.canvas_extent;

        let bounds = Bounds::from([
            [pos.x(), extent.ymin()],
            [extent.xmax(), pos.y()]
        ]);

        self.child(bounds, CursorUpdate::Horizontal, add_content);

        self.cursor.canvas_pos = Point(self.cursor.canvas_pos.x(), self.cursor.canvas_allocated.ymin());

        self
    }

    pub fn vertical(&mut self, add_content: impl FnOnce(&mut Ui)) -> &mut Self {
        let pos = self.cursor.canvas_pos;
        let extent = self.cursor.canvas_extent;
        let bounds = Bounds::<Canvas>::from((
            [pos.x(), extent.ymin()],
            [extent.xmax() - pos.x(), pos.y() - extent.ymin()]
        ));

        self.child(bounds, CursorUpdate::Vertical, add_content);

        self.cursor.canvas_pos = Point(self.cursor.canvas_allocated.xmax(), self.cursor.canvas_pos.y());
        self.cursor.page_pos = Point(self.cursor.page_allocated.xmax(), self.cursor.page_pos.y());

        self
    }

    pub fn horizontal_view(
        &mut self, 
        size: UiSize, 
        add_content: impl FnOnce(&mut Ui)
    ) -> &mut Self {
        let bounds = match size {
            UiSize::Canvas(width, height) => todo!(),
            UiSize::Page(width, height) => {
                self.update.alloc_page(
                    Size(width, height), 
                    &mut self.cursor
                )
            }
        };
            
        self.child_view(bounds, CursorUpdate::Horizontal, add_content);

        self.cursor.canvas_pos = Point(self.cursor.canvas_pos.x(), self.cursor.canvas_allocated.ymin());

        println!("View {:?}", bounds);
        println!("Alloc {:?}", self.cursor.canvas_allocated);
        println!("Extent {:?}", self.cursor.canvas_extent);
        println!("PageAlloc {:?}", self.cursor.page_allocated);

        self
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

pub(crate) fn draw_top<'a>(
    id: ViewSizeId, 
    state: ViewSizeCache, 
    renderer: &'a mut dyn Renderer, 
    add_content: &'a mut dyn FnMut(&mut Ui)
) -> (ViewSizeId, ViewSizeCache) {
    let mut style = UiStyle::new();
    style.button_press.color("red");

    let mut top = CursorTop::new(id, state, renderer.pos());

    let cursor = Cursor::new(renderer.pos(), top.prev_state.page);

    let style = UiStyle::new();
    
    let mut ui = Ui {
        state: &top.prev_state,
        cursor,
        renderer,
        style: &style,
        update: CursorUpdate::Vertical,
    };

    (add_content)(&mut ui);

    // todo()
    top.next_state.page = ui.cursor.page_allocated;
    println!("NextPage: {:?}", top.next_state.page);

    let last_id = top.last_id;
    let next_state = top.merge_state();

    (last_id, next_state)
}

pub enum UiSize {
    Canvas(f32, f32),
    Page(f32, f32),
}

#[derive(Default)]
pub struct UiGroup {
    canvas_size: Size,
    page_size: Size,

    items: Vec<UiGroup>,
}

pub struct UiState {
}

impl UiState {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn draw<R>(
        &mut self, 
        renderer: &mut dyn Renderer,
        f: impl FnOnce(&mut Ui) -> R
    ) -> R {
        todo!();
        /*
        let cursor = Cursor::new(renderer.pos().clone());
        
        let mut ui = Ui::new(renderer, cursor);
        (f)(&mut ui)
        */
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
        }
    }
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
