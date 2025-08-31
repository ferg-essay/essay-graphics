use essay_graphics_ui::{main_loop::MainLoop};

fn main() { 
    MainLoop::new().show(move |ui| {
        ui.label("hello, world");
    });
}
