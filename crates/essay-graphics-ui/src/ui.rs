use essay_graphics_api::{renderer::{self, Event, Renderer}, Point};

use super::{button::UiButton, label::UiLabel, style::UiStyle};

pub struct Ui<'a> {
    renderer: &'a mut dyn Renderer,
    // items: Vec<Box<dyn UiItem>>,
    style: UiStyle,
}

impl<'a> Ui<'a> {
    pub(super) fn new(renderer: &'a mut dyn Renderer) -> Self {
        let mut style = UiStyle::new();
        style.button_press.color("red");

        Self {
            renderer,
            style
        }
    }

    pub fn add(&mut self, mut item: impl UiItem) {
        item.draw(self.renderer, &self.style).unwrap();
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        let pos = Point::from((100., 100.));

        let label = UiLabel::new(pos, label);

        self.add(label);

        // self.items.push(Box::new(UiLabel::new(pos, label)));

        self
    }

    pub fn button(&mut self, label: &str) -> &mut Self {
        let pos = Point::from((100., 100.));

        let button = UiButton::new(pos, label);

        self.add( button);

        self
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
