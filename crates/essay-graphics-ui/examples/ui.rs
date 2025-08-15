use essay_graphics_ui::ui::{CentralPanel, MainLoop};

fn main() { 
    MainLoop::new().show(|cxt| {
        CentralPanel::new().show(cxt, |ui| {
            ui.label("hello, world");
            ui.button("button", true);
            ui.label("second label");
        }).on_hover_ui(|ui| {
            ui.label("Testing tooltip");
        });
    });
}
