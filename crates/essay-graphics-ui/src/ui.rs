use essay_graphics_api::{renderer::{self, Canvas, Renderer}, Bounds, Point};

use super::{button::UiButton, label::UiLabel, style::UiStyle};

pub struct Ui<'a> {
    renderer: &'a mut dyn Renderer,
    style: UiStyle,
    cursor: Cursor,
}

impl<'a> Ui<'a> {
    pub(super) fn new(renderer: &'a mut dyn Renderer) -> Self {
        let mut style = UiStyle::new();
        style.button_press.color("red");

        Self {
            cursor: Cursor::new(renderer.pos().clone()),
            renderer,
            style,
        }
    }

    pub fn renderer(&mut self) -> &mut dyn Renderer {
        self.renderer
    }

    pub fn style(&mut self) -> &UiStyle {
        &self.style
    }

    pub fn allocate_rect(&mut self, size: Point) -> Bounds<Canvas> {
        let rect = Bounds::<Canvas>::new(
            (self.cursor.pos.x(), self.cursor.pos.y() - size.y()),
            (self.cursor.pos.x() + size.x(), self.cursor.pos.y()),
        );

        self.cursor.pos = Point(self.cursor.pos.x(), self.cursor.pos.y() - size.y());

        rect
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
}

pub struct Cursor {
    _bounds: Bounds<Canvas>,
    pos: Point,
}

impl Cursor {
    fn new(bounds: Bounds<Canvas>) -> Self {
        let pos = (bounds.xmin(), bounds.ymax());
        Self {
            _bounds: bounds,
            pos: pos.into(),
        }
    }
}



pub trait Widget : Send + 'static {
    fn ui(
        &mut self, 
        ui: &mut Ui,
    ) -> renderer::Result<()>;
}
