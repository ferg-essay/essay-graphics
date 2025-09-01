use essay_graphics_ui::{main_loop::MainLoop};


fn main() { 
    let mut hello = false;
    let mut there = false;

    MainLoop::new().show(move |ui| {
        if ui.button("hello-g", hello).clicked() { hello=!hello; }
        if ui.button("there", there).clicked() { there=!there; }
        ui.row(|ui| {
            ui.label("gab");

            ui.column(|ui| {
                ui.label("bag");
                ui.label("gaba");
            });

            ui.label("c");
        });
        ui.label("tail");
    });
}
