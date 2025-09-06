use essay_graphics_ui::{main_loop::MainLoop, ui::{Frame, Ui}};

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
                    Frame::group(ui).background("amber").show(ui, |ui| {
                        ui.view(|ui| {
                            ui.label("B")
                        });
                        ui.view(|ui| {
                            Frame::group(ui).show(ui, |ui| {
                                ui.view(|ui| {
                                    ui.label("C")
                                });
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
                    Frame::group(ui).background("azure").show(ui, |ui| {
                        ui.view(|ui| {
                            ui.label("2")
                        });
                    });
                    ui.view(|ui| {
                        ui.label("3");
                    });
                });
                /*
                ui.vertical(|ui| {
                    ui.view(|ui| {
                        ui.label("1")
                    });
                        ui.view(|ui| {
                            ui.label("2")
                        });
                    ui.view(|ui| {
                        ui.label("3");
                    });
                });
                */
            });
    });
}
