use essay_graphics::ui::ui2::Ui2;
//use essay_graphics::ui::UiView;
//use essay_graphics::layout::MainLoop;
use essay_graphics_ui::ui::{CentralPanel, MainLoop};


fn main() { 
    let mut hello = false;
    let mut there = false;

    MainLoop::new().show(move |ctx| {
        CentralPanel::new().show(ctx, move |ui: &mut Ui2| {
            ui.button("hello", hello).onclick(|| { hello=!hello; });
            ui.button("there", there).onclick(|| { there=!there; });
            ui.horizontal(|ui| {
                ui.label("gab");

                ui.vertical(|ui| {
                    ui.label("bag");
                    ui.label("gaba");
                });

                ui.label("c");
            });
            ui.label("tail");
        });
    });
}
