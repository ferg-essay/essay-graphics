use essay_graphics_api::{
    input::Input, renderer::{Canvas, Renderer}, Bounds, Point, Size, TextStyle
};

use crate::ui::{
    button::Button, 
    label::Label, 
    style::UiStyle,
};

pub struct Ui<'a> {
    renderer: &'a mut dyn Renderer,
    style: UiStyle,
    cursor: Cursor,
    update: CursorUpdate,
}

impl<'a> Ui<'a> {
    pub(super) fn new(
        renderer: &'a mut dyn Renderer,
        cursor: Cursor,
    ) -> Self {
        let mut style = UiStyle::new();
        style.button_press.color("red");

        Self {
            cursor,
            renderer,
            style,
            update: CursorUpdate::Vertical,
        }
    }

    #[inline]
    pub fn draw<R>(
        renderer: &'a mut dyn Renderer,
        f: impl FnOnce(&mut Ui) -> R
    ) -> R {
        let cursor = Cursor::new(renderer.pos().clone());

        let mut ui = Ui::new(renderer, cursor);

        (f)(&mut ui)
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
        self.update.alloc(size, &mut self.cursor)
    }

    #[inline]
    pub fn remaining_size(&mut self) -> Size {
        self.cursor.remaining_size()
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

    pub fn horizontal(&mut self, builder: impl FnOnce(&mut Ui)) -> &mut Self {
        let pos = self.cursor.pos;
        let extent = self.cursor.extent;

        let bounds = Bounds::from([
            [pos.x(), extent.ymin()],
            [extent.xmax(), pos.y()]
        ]);

        let mut child = Ui {
            renderer: self.renderer,
            cursor: Cursor::new(bounds),
            style: self.style.clone(),
            update: CursorUpdate::Horizontal,
        };

        (builder)(&mut child);

        self.cursor.bounds = self.cursor.bounds.union(&child.cursor.bounds);
        self.cursor.pos = Point(self.cursor.pos.x(), self.cursor.bounds.ymin());

        self
    }

    pub fn vertical(&mut self, builder: impl FnOnce(&mut Ui)) -> &mut Self {
        let pos = self.cursor.pos;
        let extent = self.cursor.extent;
        let bounds = Bounds::<Canvas>::from((
            [pos.x(), extent.ymin()],
            [extent.xmax() - pos.x(), pos.y() - extent.ymin()]
        ));

        let mut child = Ui {
            renderer: self.renderer,
            cursor: Cursor::new(bounds),
            style: self.style.clone(),
            update: CursorUpdate::Vertical,
        };

        (builder)(&mut child);

        self.cursor.bounds = self.cursor.bounds.union(&child.cursor.bounds);
        self.cursor.pos = Point(self.cursor.bounds.xmax(), self.cursor.pos.y());

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
        let cursor = Cursor::new(renderer.pos().clone());
        
        let mut ui = Ui::new(renderer, cursor);
        (f)(&mut ui)
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
        }
    }
}


#[derive(Clone, Debug)]
pub struct Cursor {
    pos: Point,
    bounds: Bounds<Canvas>, // current bounds allocated by the cursor
    extent: Bounds<Canvas>, // full extent of the canvas
}

impl Cursor {
    pub(crate) fn new(pos: Bounds<Canvas>) -> Cursor {
        let point = Point(pos.xmin(), pos.ymax());

        Cursor {
            pos: point,
            bounds: Bounds::from(point),
            extent: pos,
        }
    }

    pub(crate) fn remaining_size(&self) -> Size {
        Size(
            self.extent.xmax() - self.pos.x(),
            self.pos.y() - self.extent.ymin()
        )
    }
}

enum CursorUpdate {
    Vertical,
    Horizontal,
}

impl CursorUpdate {
    fn alloc(&self, size: Size, cursor: &mut Cursor) -> Bounds<Canvas> {
        match self {
            CursorUpdate::Vertical => {
                let rect = Bounds::<Canvas>::new(
                    [cursor.pos.x(), cursor.pos.y() - size.height()],
                    [cursor.pos.x() + size.width(), cursor.pos.y()],
                );
        
                cursor.pos = Point(cursor.pos.x(), cursor.pos.y() - size.height());
                cursor.bounds = cursor.bounds.union(&rect);

                rect
            },
            CursorUpdate::Horizontal => {
                let rect = Bounds::<Canvas>::new(
                    [cursor.pos.x(), cursor.pos.y() - size.height()],
                    [cursor.pos.x() + size.width(), cursor.pos.y()],
                );
        
                cursor.pos = Point(cursor.pos.x() + size.width(), cursor.pos.y());
                cursor.bounds = cursor.bounds.union(&rect);

                rect
            }
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



pub trait Widget {
    fn ui(
        &mut self, 
        ui: &mut Ui,
    ) -> Response;
}
