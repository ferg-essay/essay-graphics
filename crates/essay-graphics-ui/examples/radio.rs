use essay_graphics_ui::{column, main_loop::MainLoop, widget2::{radio_value, Element}};

fn main() { 
    let mut state = State::default();

    MainLoop::new().show(move |ui| {
        ui.app(&mut state, State::view, State::update);
    });
}

struct State {
    value: Values,
}

impl Default for State {
    fn default() -> Self {
        Self { value: Values::None, }
    }
}

impl State {
    fn view(&self) -> impl Into<Element<'_, Message>> {
        column![
            radio_value("button A", self.value, Values::A).on_press_with(Message::Radio),
            radio_value("button B", self.value, Values::B).on_press_with(Message::Radio),
        ]
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::A => { self.value = Values::A; }
            Message::B => { self.value = Values::B; }
            Message::Radio(value) => { self.value = value; }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(unused)]
enum Values {
    None,
    A,
    B,
    C,
}

#[derive(Clone)]
#[allow(unused)]
enum Message {
    A,
    B,
    Radio(Values),
}