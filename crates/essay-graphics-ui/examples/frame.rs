use essay_graphics_ui::{main_loop::MainLoop, ui::{Frame}};

fn main() { 
    let mut option_a = false;
    let mut option_b = false;

    MainLoop::new().show(move |ui| {
        Frame::group(ui).show(ui, |ui| {
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
}
