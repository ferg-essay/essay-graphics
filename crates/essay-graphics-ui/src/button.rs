use essay_graphics_api::{renderer::{self, Renderer}, Point};

use super::{style::UiStyle, ui::UiItem};

pub(crate) struct UiButton {
    pos: Point,
    label: String,
    press: bool,
}

impl UiButton {
    pub(crate) fn new(pos: Point, label: &str) -> Self {
        Self {
            pos,
            label: String::from(label),
            press: false,
        }
    }
}

impl UiItem for UiButton {
    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer, 
        style: &UiStyle
    ) -> renderer::Result<()> {
        if self.press { 
            renderer.draw_text(self.pos, &self.label, 0., &style.button_press, &style.button_text)
        } else {
            renderer.draw_text(self.pos, &self.label, 0., &style.button, &style.button_text)
        }
    }
    
    fn event(
        &mut self,
        event: &renderer::Event,
    ) -> renderer::Result<()> {
        match event {
            renderer::Event::MouseLeftPress(_) => {
                self.press = !self.press;
            }
            _ => {

            }
        }

        Ok(())
    }
}
