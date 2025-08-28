use essay_graphics_ui::{main_loop::MainLoop, ui::CentralPanel, widget2::{button, Element}};

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
    value: bool,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::A => {
                self.value = !self.value;
            }
        }
    }

    fn view(&self) -> impl Into<Element<'_, Message>> {
        button("test").press(self.value).on_press(Message::A)
    }
}

#[derive(Clone, Debug)]
enum Message {
    A
}