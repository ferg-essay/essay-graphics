use std::sync::Arc;

use essay_graphics_api::{renderer::{self, Canvas, Drawable, Renderer}, Bounds, Path, PathStyle};
use essay_graphics_ui::ui::{ui2::ResponseValue, CentralPanel, Context, MainLoop, UiView};

fn main() { 
    MainLoop::new().show(|cxt| {
        CentralPanel::new().show(cxt, |ui| {
            ui.label("hello, world");
            ui.label("second label");
        }).on_hover_ui(|ui| {
            ui.label("Testing tooltip");
        });
    });
}
