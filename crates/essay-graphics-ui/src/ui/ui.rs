use essay_graphics_api::{
    renderer::{self, Canvas, Event, Renderer}, 
    Bounds, Point, Size, TextStyle
};

use crate::ui::{
    button::UiButton, 
    label::UiLabel, 
    style::UiStyle,
    ui_view::UiInput,
};

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

    pub fn add(&mut self, mut widget: impl Widget) -> Response {
        widget.ui(self)
    }

    pub fn label(&mut self, label: &str) -> Response {
        let label = UiLabel::new(label);

        self.add(label)
    }

    pub fn button(&mut self, label: &str, press: bool) -> Response {
        let button = UiButton::new(label, press);

        self.add(button)
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

pub struct UiState {
    input: UiInput,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            input: UiInput::default(),
        }
    }

    pub fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        f: impl FnOnce(&mut Ui)
    ) -> renderer::Result<()> {
        let cursor = Cursor::new(renderer.pos().clone());
        
        let input = self.input.clone();
        let mut ui = Ui::new(renderer, cursor, &input);
        (f)(&mut ui);

        self.input.update();

        Ok(())
    }

    pub fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) {
        match event {
            Event::MouseMove(p) => {
                self.input.cursor = Some(*p);
            }
            Event::MouseLeftPress(p) => {
                self.input.left_press_one = Some(*p);
                self.input.left_press = Some(*p);
            }
            Event::MouseLeftRelease(_) => {
                self.input.left_press = None;
            }
            _ => {}

        }
        renderer.request_redraw(&Bounds::none());
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



pub trait Widget : Send + 'static {
    fn ui(
        &mut self, 
        ui: &mut Ui,
    ) -> Response;
}
