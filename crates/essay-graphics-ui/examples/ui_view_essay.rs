use essay_graphics_api::{Color, Length, Size};
use essay_graphics_ui::{main_loop::MainLoop, ui::{Frame}};

fn main() { 
    MainLoop::new().show(move |ui| {
        ui.row_with(Size::new(Length::Shrink, Length::View(0.5)), |ui| {
            ui.row_with(Length::Fill, |ui| {
                ui.view_with(Color::from("amber"), |ui| {
                    ui.label("A")
                });
                ui.view_with(Color::from("azure"), |ui| {
                    ui.label("B")
                });
                ui.view_with(Color::from("teal"), |ui| {
                    ui.label("C")
                });
            });
            ui.row_with(Size::new(Length::View(0.5), Length::Fill), |ui| {
                ui.view_with(Color::from("red"), |ui| {
                    ui.label("1")
                });
            });
        });
        ui.row(|ui| {
            ui.row(|ui| {
                ui.view_with(Color::from("orange"), |ui| {
                    ui.label("1")
                });
            });
            ui.column_with(Size::new(Length::View(0.5), Length::Fill), |ui| {
                ui.view_with(Color::from("purple"), |ui| {
                    ui.label("1")
                });
            });
        });
    });
}
