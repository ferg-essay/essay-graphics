use essay_graphics_ui::{main_loop::MainLoop, ui::CentralPanel};

fn main() { 
    let mut option_a = false;
    let mut option_b = false;

    MainLoop::new().show(move |cxt| {
        CentralPanel::new().show(cxt, |ui| {
            ui.menu_button("hello, world", |ui| {
                if ui.button("Option A", option_a).clicked() { 
                    println!("Click A");
                    option_a = !option_a 
                };
                if ui.button("Option B", option_b).clicked() {
                    println!("Click B");
                    option_b = !option_b
                };
            });
        });
    });
}
