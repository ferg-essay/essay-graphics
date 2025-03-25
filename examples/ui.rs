use essay_graphics::ui::UiView;
use essay_graphics::layout::{MainLoop, Page};

fn main() { 
    let mut page = Page::new();

    let mut hello = false;
    let mut there = false;

    page.view((0., 0., 2., 2.) , UiView::new(move |ui| {
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

    MainLoop::new().show(page);
}
