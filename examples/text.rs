use renderer::Renderer;
use essay_graphics::prelude::*;
use essay_graphics::layout::MainLoop;

fn main() { 
    MainLoop::new().show(Box::new(move |ui: &mut dyn Renderer| {
        let path_style = PathStyle::new();
        let mut text_style = TextStyle::new();
        text_style.halign(HorizAlign::Left);

        ui.draw_text(Point(0., 0.), "The quick brown fox jumped over the lazy dog.", 0., &path_style, &text_style)?;
        ui.draw_text(Point(100., 100.), "sample text", 0., &path_style, &text_style)?;
        ui.draw_text(Point(100., 150.), "Coffee \u{2615}", 0., &path_style, &text_style)?;
        ui.draw_text(Point(100., 200.), "Cowboy \u{1f920}", 0., &path_style, &text_style)?;

        Ok(())
    }));
}
