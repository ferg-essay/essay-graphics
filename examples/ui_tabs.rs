use essay_graphics::ui::{Tabs, UiSize};
use essay_graphics_ui::main_loop::MainLoop;

fn main() { 
    let mut hello = false;
    let mut there = false;
    let mut button_b = false;
    let mut selected = String::from("a");

    MainLoop::new().show(move |ui| { // let view = UiView::new(move |ui| {

            let mut tabs = Tabs::<String>::new(selected.clone());
            tabs.item(String::from("a"), |ui| {
                ui.row_size(UiSize::View(1., 1.), |ui| {
                    ui.row_size(UiSize::View(1., 1.), |ui| {
                        ui.label("Option A");
                    });
                    ui.row_size(UiSize::View(1., 1.), |ui| {
                        if ui.button("hello", hello).clicked() { hello=!hello; }
                    });
                    ui.row_size(UiSize::View(1., 1.), |ui| {
                        ui.label("more");
                    });
                });
                ui.row_size(UiSize::View(1., 1.), |ui| {
                    if ui.button("there", there).clicked() { there=!there; }
                });
            });

            tabs.item(String::from("b"), |ui| {
                ui.column_size(UiSize::View(1., 1.), |ui| {
                    ui.column_size(UiSize::View(1., 1.), |ui| {
                        ui.label("Option B");
                    });
                    ui.column_size(UiSize::View(1., 1.), |ui| {
                        if ui.button("Button B", button_b).clicked() { button_b=!button_b; }
                    });
                    ui.column_size(UiSize::View(1., 1.), |ui| {
                        ui.label("more");
                    });
                });
                ui.column_size(UiSize::View(1., 1.), |ui| {
                    ui.label("Tail");
                    ui.label("AfterTail");
                });
            });
            selected = tabs.show(ui).unwrap();
    });

    //MainLoop::new().show(view);
}
