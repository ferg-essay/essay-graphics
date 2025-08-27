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
        println!("Update {:?}", message);
    }

    fn view(&self) -> impl Into<Element<'_, Message>> {
        println!("View");
        button("test")
    }
}

#[derive(Debug)]
enum Message {
    A
}