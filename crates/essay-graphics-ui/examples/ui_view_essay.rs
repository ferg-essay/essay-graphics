use essay_graphics_api::{Length, Size};
use essay_graphics_ui::{main_loop::MainLoop, ui::{Frame}};

fn main() { 
    MainLoop::new().show(move |ui| {
        ui.row_with(Size::new(Length::Shrink, Length::View(0.5)), |ui| {
            ui.row_with(Length::Fill, |ui| {
                Frame::group(ui).background("amber").show(ui, |ui| {
                    ui.view(|ui| {
                        ui.label("A")
                    });
                });
                Frame::group(ui).background("azure").show(ui, |ui| {
                    ui.view(|ui| {
                        ui.label("B")
                    });
                });
                Frame::group(ui).background("teal").show(ui, |ui| {
                    ui.view(|ui| {
                        ui.label("C")
                    });
                });
            });
            ui.row_with(Length::Fill, |ui| {
                Frame::group(ui).background("red").show(ui, |ui| {
                    ui.view(|ui| {
                        ui.label("1")
                    });
                });
            });
        });
        ui.row(|ui| {
            Frame::group(ui).background("orange").show(ui, |ui| {
                ui.view(|ui| {
                    ui.label("1")
                });
            });
        });
    });
}
