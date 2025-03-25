use essay_graphics_api::{renderer::{self, Canvas, Drawable, Event, Renderer}, Bounds, Point};

use crate::ui::Cursor;

use super::Ui;

pub struct UiView {
    builder: Box<dyn FnMut(&mut Ui)->() + Send>,

    input: UiInput,
    // ui: UiRoot,
}

impl UiView {
    pub fn new(builder: impl FnMut(&mut Ui)->() + 'static + Send) -> Self {
        Self {
            builder: Box::new(builder),
            input: UiInput::default(),
        }
    }
}

impl Drawable for UiView {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        let cursor = Cursor::new(renderer.pos().clone());
        
        let input = self.input.clone();
        let mut ui = Ui::new(renderer, cursor, &input);
        (self.builder)(&mut ui);

        self.input.update();

        Ok(())
    }

    fn resize(
        &mut self, 
        _renderer: &mut dyn Renderer, 
        bounds: &Bounds<Canvas>
    ) -> Bounds<Canvas> {
        // self.ui.resize(renderer, bounds)
        bounds.clone()
    }

    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) {
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

#[derive(Clone)]
pub struct UiInput {
    pub cursor: Option<Point>,
    pub left_press_one: Option<Point>,
    pub left_press: Option<Point>,
}

impl UiInput {
    fn update(&mut self) {
        self.left_press_one = None;
    }
}

impl Default for UiInput {
    fn default() -> Self {
        Self { 
            cursor: Default::default(),
            left_press: Default::default(),
            left_press_one: Default::default(),
        }
    }
}
