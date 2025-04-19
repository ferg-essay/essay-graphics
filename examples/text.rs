use essay_graphics_api::color::Grey;
use renderer::Renderer;
use essay_graphics::prelude::*;
use essay_graphics::layout::MainLoop;
use essay_graphics_api::Coord;

fn main() { 
    MainLoop::new().show(Box::new(move |ui: &mut dyn Renderer| {
        let path_style = PathStyle::new();
        let mut text_style = TextStyle::new();
        text_style.halign(HorizAlign::Left);

        ui.draw_text(Point(100., 400.), "sample text", 0., &path_style, &text_style)?;
        ui.draw_text(Point(100., 350.), "Coffee \u{2615}", 0., &path_style, &text_style)?;
        ui.draw_text(Point(100., 300.), "Cowboy \u{1f920}", 0., &path_style, &text_style)?;

        Ok(())
    }));
}
