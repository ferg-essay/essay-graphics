use essay_graphics_ui::UiView;
use essay_graphics::layout::LayoutMainLoop;

fn main() { 
    let mut figure = LayoutMainLoop::new();

    figure.view((0., 0., 2., 2.) , UiView::new(|ui| {
        ui.button("hello");
        ui.button("there");
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
