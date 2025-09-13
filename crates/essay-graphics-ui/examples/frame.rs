use essay_graphics_ui::{column, main_loop::MainLoop, ui::Frame, widget2::{Element, button}};

fn main() { 
    let mut state = State::default();

    MainLoop::new().show(move |ui| {
        Frame::group().show(ui, |ui| {
            ui.app(&mut state, State::view, State::update);
        });
    });
}

#[derive(Default)]
struct State {
    option_a: bool,
    option_b: bool,
}

impl State {
    fn update(&mut self, message: Event) {
        match message {
            Event::PressA => { self.option_a = !self.option_a; }
            Event::PressB => { self.option_b = !self.option_b; }
        }
    }

    fn view(&self) -> impl Into<Element<'_, Event>> {
        column![
            button("Option A").press(self.option_a).on_press(Event::PressA),
            button("Option B").press(self.option_b).on_press(Event::PressB),
        ]
    }
}

#[derive(Copy, Clone)]
pub enum Event {
    PressA,
    PressB,
}
