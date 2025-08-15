use essay_graphics_ui::ui::{CentralPanel, MainLoop};

fn main() { 
    let mut button = false;
    MainLoop::new().show(move |cxt| {
        CentralPanel::new().show(cxt, |ui| {
            ui.label("hello, world");
            if ui.button("button", button).clicked() {
                button = !button;
            }
            ui.label("second label");
        }).on_hover_ui(|ui| {
            ui.label("Testing tooltip");
        });
    });
}
