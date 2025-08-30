use essay_graphics_ui::{main_loop::MainLoop, ui::CentralPanel, widget2::{button, column, row, Element}};

fn main() { 
    let mut state = State::default();
    MainLoop::new().show(move |cxt| {
        CentralPanel::new().show(cxt, |ui| {
            ui.app(&mut state, State::update, State::view)
        });
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
        row([
            button("button A").press(self.a).on_press(Message::A).into(),
            button("button B").press(self.b).on_press(Message::B).into(),
            "text".into(),
        ])
    }
}

#[derive(Clone, Debug)]
enum Message {
    A,
    B,
}