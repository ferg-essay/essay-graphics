use essay_graphics_ui::{
    column,
    main_loop::MainLoop, 
    widget2::{button, text, Element, WidgetFrame}
};

fn main() { 
    let mut state = State::default();
    MainLoop::new().show(move |ui| {
        ui.app(&mut state, State::update, State::view);
    });
}

#[derive(Default)]
struct State {
    a: bool,
    b: bool,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::A => {
                self.a = !self.a;
            }
            Message::B => {
                self.b = !self.b;
            }
        }
    }

    fn view(&self) -> impl Into<Element<'_, Message>> {
        column![
            button("button A").press(self.a).on_press(Message::A).tooltip("Tooltip A"),
            button("button B").press(self.b).on_press(Message::B).tooltip("Tooltip B"),
            text("text")
        ]
    }
}

#[derive(Clone, Debug)]
enum Message {
    A,
    B,
}