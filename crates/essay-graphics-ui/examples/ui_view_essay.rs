use essay_graphics_api::Length;
use essay_graphics_ui::{main_loop::MainLoop, ui::{Frame}};

fn main() { 
    MainLoop::new().show(move |ui| {
        ui.row(|ui| {
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
    });
}
