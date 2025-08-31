use essay_graphics_ui::{main_loop::MainLoop};

fn main() { 
    let mut var = Values::None;

    MainLoop::new().show(move |ui| {
        ui.radio_value(&mut var, Values::A, "Option A");
        ui.radio_value(&mut var, Values::B, "Option B");
        ui.radio_value(&mut var, Values::C, "Option C");
    });
}

#[derive(Debug, PartialEq)]
enum Values {
    None,
    A,
    B,
    C,
}