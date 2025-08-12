use essay_graphics_ui::{page::MainLoop, ui::UiView};

fn main() { 
    let mut hello = false;
    let mut there = false;

    let view = UiView::new(move |ui| {
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

    MainLoop::new().show(view);
}
