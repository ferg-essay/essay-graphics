use essay_graphics_ui::{main_loop::MainLoop, widget2::{button, column, menu_button, selectable_label, Element}};

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
    c: bool,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::A => { self.a = !self.a; }
            Message::B => { self.b = !self.b; }
            Message::C => { self.c = !self.c; }
        }
    }

    fn view(&self) -> impl Into<Element<'_, Message>> {
        let mut vec = Vec::new();

        vec.push(menu_button("Menu", vec![
            selectable_label("A").on_press(Message::A),
            selectable_label("B").on_press(Message::B),
            selectable_label("C").on_press(Message::C),
        ]).into());

        if self.a {
            vec.push(button("Button A").into());
        }

        if self.b {
            vec.push(button("Button B").into());
        }

        if self.c {
            vec.push(button("Button C").into());
        }

        column(vec)
    }
}

#[derive(Clone, Debug)]
enum Message {
    A,
    B,
    C,
}