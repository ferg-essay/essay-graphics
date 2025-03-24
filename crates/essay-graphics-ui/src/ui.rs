use essay_graphics_api::{renderer::{self, Canvas, Renderer}, Bounds, Point, Size, TextStyle};

use crate::ui_view::UiInput;

use super::{button::UiButton, label::UiLabel, style::UiStyle};

pub struct Ui<'a> {
    renderer: &'a mut dyn Renderer,
    input: &'a UiInput,
    style: UiStyle,
    cursor: Cursor,
    update: CursorUpdate,
}

impl<'a> Ui<'a> {
    pub(super) fn new(
        renderer: &'a mut dyn Renderer,
        cursor: Cursor,
        input: &'a UiInput,
    ) -> Self {
        let mut style = UiStyle::new();
        style.button_press.color("red");

        Self {
            cursor,
            renderer,
            style,
            input,
            update: CursorUpdate::Vertical,
        }
    }

    pub fn renderer(&mut self) -> &mut dyn Renderer {
        self.renderer
    }

    pub fn input(&self) -> &UiInput {
        self.input
    }

    pub fn style(&mut self) -> &UiStyle {
        &self.style
    }

    pub fn allocate_rect(&mut self, size: Size) -> Bounds<Canvas> {
        self.update.alloc(size, &mut self.cursor)
    }

    pub fn add(&mut self, mut widget: impl Widget) {
        widget.ui(self).unwrap();
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        let label = UiLabel::new(label);

        self.add(label);

        self
    }

    pub fn button(&mut self, label: &str) -> &mut Self {
        let button = UiButton::new(label);

        self.add(button);

        self
    }

    pub fn horizontal(&mut self, builder: impl FnOnce(&mut Ui)) -> &mut Self {
        let pos = self.cursor.pos;

        let mut child = Ui {
            renderer: self.renderer,
            cursor: Cursor::new(Bounds::<Canvas>::from(pos)),
            style: self.style.clone(),
            update: CursorUpdate::Horizontal,
            input: self.input,
        };

        (builder)(&mut child);

        self.cursor.bounds = self.cursor.bounds.union(&child.cursor.bounds);
        self.cursor.pos = Point(self.cursor.pos.x(), self.cursor.bounds.ymin());

        self
    }

    pub fn vertical(&mut self, builder: impl FnOnce(&mut Ui)) -> &mut Self {
        let pos = self.cursor.pos;

        let mut child = Ui {
            renderer: self.renderer,
            cursor: Cursor::new(Bounds::<Canvas>::from(pos)),
            style: self.style.clone(),
            update: CursorUpdate::Vertical,
            input: self.input,
        };

        (builder)(&mut child);

        self.cursor.bounds = self.cursor.bounds.union(&child.cursor.bounds);
        self.cursor.pos = Point(self.cursor.bounds.xmax(), self.cursor.pos.y());

        self
    }
    
    pub fn text_size(&mut self, label: &str, style_text: &TextStyle) -> Size {
        self.renderer.text_size(label, style_text)
    }
}

#[derive(Clone, Debug)]
pub struct Cursor {
    bounds: Bounds<Canvas>,
    pos: Point,
}

impl Cursor {
    pub(crate) fn new(pos: Bounds<Canvas>) -> Cursor {
        let pos = Point(pos.xmin(), pos.ymax());

        Cursor {
            pos,
            bounds: Bounds::from(pos),
        }
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
                    (cursor.pos.x(), cursor.pos.y() - size.height()),
                    (cursor.pos.x() + size.width(), cursor.pos.y()),
                );
        
                cursor.pos = Point(cursor.pos.x(), cursor.pos.y() - size.height());
                cursor.bounds = cursor.bounds.union(&rect);

                rect
            },
            CursorUpdate::Horizontal => {
                let rect = Bounds::<Canvas>::new(
                    (cursor.pos.x(), cursor.pos.y() - size.height()),
                    (cursor.pos.x() + size.width(), cursor.pos.y()),
                );
        
                cursor.pos = Point(cursor.pos.x() + size.width(), cursor.pos.y());
                cursor.bounds = cursor.bounds.union(&rect);

                rect
            }
        }
    }
}



pub trait Widget : Send + 'static {
    fn ui(
        &mut self, 
        ui: &mut Ui,
    ) -> renderer::Result<()>;
}
