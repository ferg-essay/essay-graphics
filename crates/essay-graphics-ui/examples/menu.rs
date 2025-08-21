use essay_graphics_ui::{main_loop::MainLoop, ui::CentralPanel};

fn main() { 
    MainLoop::new().show(move |cxt| {
        CentralPanel::new().show(cxt, |ui| {
            ui.menu_button("hello, world", |ui| {
                ui.button("Option A", false);
                ui.button("Option B", false);
            });
        });
    });
}
