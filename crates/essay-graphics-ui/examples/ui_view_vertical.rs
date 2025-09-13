use essay_graphics_api::Color;
use essay_graphics_ui::{main_loop::MainLoop, ui::{Frame}};

fn main() { 
    MainLoop::new().show(move |ui| {
            ui.row(|ui| {
                /*
                ui.vertical(|ui| {
                    ui.view(|ui| {
                        ui.label("A")
                    });
                    ui.view(|ui| {
                        ui.label("B")
                    });
                    ui.view(|ui| {
                        ui.label("C")
                    });
                    ui.view(|ui| {
                        ui.label("D")
                    });
                });
                */
                ui.column(|ui| {
                    ui.view(|ui| {
                        ui.label("A")
                    }).response.on_hover_ui(ui, |ui| {
                        ui.label("Tooltip for Label A"); 
                    });
                    ui.column_with(Color::from("amber"), |ui| {
                        ui.view(|ui| {
                            ui.label("B")
                        });
                        ui.view(|ui| {
                            ui.view_with(Frame::group(), |ui| {
                                ui.label("C")
                            });
                        });
                    });
                    ui.view(|ui| {
                        ui.label("D")
                    });
                });
                ui.column(|ui| {
                    ui.view(|ui| {
                        ui.label("1")
                    });
                    ui.view_with(Color::from("azure"), |ui| {
                        ui.label("2")
                    });
                    ui.view(|ui| {
                        ui.label("3");
                    });
                });
            });
    });
}
