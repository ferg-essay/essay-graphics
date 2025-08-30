use essay_graphics_ui::{main_loop::MainLoop, ui::CentralPanel};

fn main() { 
    let mut button = false;
    MainLoop::new().show(move |ui| {
        ui.label("hello, world");
    });
}
