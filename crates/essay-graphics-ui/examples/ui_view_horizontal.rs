use essay_graphics_ui::{main_loop::MainLoop, ui::{Frame}};

fn main() { 
    MainLoop::new().show(move |ui| {
            ui.row(|ui| {
                ui.view(|ui| {
                    ui.label("A")
                });
                Frame::group().background("amber").show(ui, |ui| {
                    ui.view(|ui| {
                        ui.label("B")
                    });
                    ui.view(|ui| {
                        ui.label("C")
                    });
                });
                ui.view(|ui| {
                    ui.label("D")
                });
            });
            ui.row(|ui| {
                ui.view(|ui| {
                    ui.label("1")
                });
                Frame::group().background("azure").show(ui, |ui| {
                    ui.view(|ui| {
                        ui.label("2")
                    });
                });
                ui.view(|ui| {
                    ui.label("3")
                });
            });
    });
}
