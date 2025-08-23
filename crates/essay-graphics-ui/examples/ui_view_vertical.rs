use essay_graphics_ui::{main_loop::MainLoop, ui::{CentralPanel, Frame, Ui}};

fn main() { 
    MainLoop::new().show(move |cxt| {
        CentralPanel::new().show(cxt, |ui| {
            ui.horizontal(|ui| {
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
                ui.vertical(|ui| {
                    ui.view(|ui| {
                        ui.label("A")
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
                ui.vertical(|ui| {
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
    });
}
