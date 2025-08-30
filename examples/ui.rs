use essay_graphics::ui::ui::Ui;
//use essay_graphics::ui::UiView;
//use essay_graphics::layout::MainLoop;
use essay_graphics_ui::{main_loop::MainLoop, ui::CentralPanel};


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
