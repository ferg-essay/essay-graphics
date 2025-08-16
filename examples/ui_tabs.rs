use essay_graphics::ui::{CentralPanel, Tabs, UiSize};
use essay_graphics_ui::main_loop::MainLoop;

fn main() { 
    let mut hello = false;
    let mut there = false;
    let mut button_b = false;
    let mut selected = String::from("a");

    MainLoop::new().show(move |ctx| { // let view = UiView::new(move |ui| {
        CentralPanel::new().show(ctx, |ui| {
            let mut tabs = Tabs::<String>::new(selected.clone());
            tabs.item(String::from("a"), |ui| {
                ui.horizontal_view(UiSize::Page(1., 1.), |ui| {
                    ui.horizontal_view(UiSize::Page(1., 1.), |ui| {
                        ui.label("Option A");
                    });
                    ui.horizontal_view(UiSize::Page(1., 1.), |ui| {
                        if ui.button("hello", hello).clicked() { hello=!hello; }
                    });
                    ui.horizontal_view(UiSize::Page(1., 1.), |ui| {
                        ui.label("more");
                    });
                });
                ui.horizontal_view(UiSize::Page(1., 1.), |ui| {
                    if ui.button("there", there).clicked() { there=!there; }
                });
            });

            tabs.item(String::from("b"), |ui| {
                ui.vertical_view(UiSize::Page(1., 1.), |ui| {
                    ui.vertical_view(UiSize::Page(1., 1.), |ui| {
                        ui.label("Option B");
                    });
                    ui.vertical_view(UiSize::Page(1., 1.), |ui| {
                        if ui.button("Button B", button_b).clicked() { button_b=!button_b; }
                    });
                    ui.vertical_view(UiSize::Page(1., 1.), |ui| {
                        ui.label("more");
                    });
                });
                ui.vertical_view(UiSize::Page(1., 1.), |ui| {
                    ui.label("Tail");
                    ui.label("AfterTail");
                });
            });
            selected = tabs.show(ui).unwrap();
        });
    });

    //MainLoop::new().show(view);
}
