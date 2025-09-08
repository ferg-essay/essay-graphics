use essay_graphics_ui::{main_loop::MainLoop, widget2::{menu_button, selectable_label, Element}};

fn main() { 
    // let mut var = Values::None;
    let mut state = State::default();

    MainLoop::new().show(move |ui| {
        ui.app(&mut state, State::view, State::update);
        /*
        ui.menu_button("Menu", |ui| {
            ui.selectable_value(&mut var, Values::A, "Option A");
            ui.selectable_value(&mut var, Values::B, "Option B");
            ui.selectable_value(&mut var, Values::C, "Option C");
        });
        */
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
        menu_button("Menu", vec![
            selectable_label("A"),
            selectable_label("B"),
            selectable_label("C"),
        ])
    }
}

#[derive(Clone, Debug)]
enum Message {
    A,
    B,
}