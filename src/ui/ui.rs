use essay_graphics_api::{renderer::{self, Canvas, Drawable, Event, Renderer}, Bounds, Point};

use super::{button::UiButton, label::UiLabel, style::UiStyle};

pub struct Ui {
    items: Vec<Box<dyn UiItem>>,
}

impl Ui {
    pub(super) fn new() -> Ui {
        Self {
            items: Vec::new(),
        }
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        let pos = Point::from((100., 100.));
        self.items.push(Box::new(UiLabel::new(pos, label)));

        self
    }

    pub fn button(&mut self, label: &str) -> &mut Self {
        let pos = Point::from((100., 100.));
        self.items.push(Box::new(UiButton::new(pos, label)));

        self
    }

    pub(super) fn build(mut self) -> UiRoot {
        if let Some(item) = self.items.pop() {
            UiRoot::new(item)
        } else {
            UiRoot::new(Box::new(UiNull))
        }
    }
}

pub(crate) struct UiRoot {
    item: Box<dyn UiItem>,
    style: UiStyle,
}

impl UiRoot {
    fn new(item: Box<dyn UiItem>) -> Self {
        let mut style = UiStyle::new();
        style.button_press.color("red");

        Self {
            item,
            style,
        }
    }

    pub(crate) fn is_dirty(&self) -> bool {
        false
    }
}

impl Drawable for UiRoot {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        self.item.draw(renderer, &self.style)
    }

    fn resize(
        &mut self, 
        _renderer: &mut dyn Renderer, 
        bounds: &Bounds<Canvas>
    ) -> Bounds<Canvas> {
        bounds.clone()
    }

    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) {
        let pos = renderer.pos().clone();
        renderer.request_redraw(&pos);
        self.item.event(event).unwrap();
    }
}

pub trait UiItem : Send + 'static {
    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        style: &UiStyle,
    ) -> renderer::Result<()>;

    fn event(
        &mut self,
        event: &Event,
    ) -> renderer::Result<()>;
}

struct UiNull;

impl UiItem for UiNull {
    fn draw(
        &mut self, 
        _renderer: &mut dyn Renderer,
        _style: &UiStyle
    ) -> renderer::Result<()> {
        Ok(())
    }

    fn event(
        &mut self, 
        _event: &Event,
    ) -> renderer::Result<()> {
        Ok(())
    }
}