use essay_graphics::ui::{Tabs, UiView};
use essay_graphics::layout::MainLoop;

fn main() { 
    let mut hello = false;
    let mut there = false;
    let mut button_b = false;
    let mut selected = String::from("a");

    let view = UiView::new(move |ui| {
        let mut tabs = Tabs::<String>::new(selected.clone());
        tabs.item(String::from("a"), |ui| {
            ui.label("Option A");
            ui.button("hello", hello).onclick(|| { hello=!hello; });
            ui.button("there", there).onclick(|| { there=!there; });
        });
        tabs.item(String::from("b"), |ui| {
            ui.label("Option B");
            ui.button("button", button_b).onclick(|| { button_b=!button_b; });
        });
        selected = tabs.show(ui).unwrap();
    });

    MainLoop::new().show(view);
}
