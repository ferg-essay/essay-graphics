use essay_graphics_ui::{main_loop::MainLoop, ui::CentralPanel};

fn main() { 
    let mut var = Values::None;

    MainLoop::new().show(move |ui| {
        ui.menu_button("Menu", |ui| {
            ui.selectable_value(&mut var, Values::A, "Option A");
            ui.selectable_value(&mut var, Values::B, "Option B");
            ui.selectable_value(&mut var, Values::C, "Option C");
        });
    });
}

#[derive(Debug, PartialEq)]
enum Values {
    None,
    A,
    B,
    C,
}