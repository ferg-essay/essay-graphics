use essay_graphics_ui::{main_loop::MainLoop, ui::CentralPanel};

fn main() { 
    let mut button = false;
    MainLoop::new().show(move |cxt| {
        CentralPanel::new().show(cxt, |ui| {
            ui.label("hello, world");
            if ui.button("button-g", button).clicked() {
                button = !button;
            }
            ui.label("second label");
        }).on_hover_ui(|ui| {
            ui.label("Testing tooltip");
        });
    });
}
