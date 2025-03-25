use essay_graphics_ui::UiView;
use essay_graphics::layout::LayoutMainLoop;

fn main() { 
    let mut figure = LayoutMainLoop::new();

    let mut hello = false;
    let mut there = false;

    figure.view((0., 0., 2., 2.) , UiView::new(move |ui| {
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
    }));

    figure.show();
}
