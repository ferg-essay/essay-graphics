use renderer::{Canvas, Renderer};
use essay_graphics::prelude::*;
use essay_graphics::layout::MainLoop;

fn main() { 
    MainLoop::new().show(Box::new(|ui: &mut dyn Renderer| {
        let mut path_style = PathStyle::new();
        path_style.color("azure");
        path_style.edge_color("black");
        //path_style.face_color(Color::none());

        let path = arch([100., 100.]);
        ui.draw_path(&path, &path_style)?;

        Ok(())
    }));
}

fn arch(point: impl Into<Point>) -> Path<Canvas> {
    let Point(x, y) = point.into();

    Path::move_to(x, y)
        .line_to(x + 50., y)
        .bezier2_to([x + 50., y + 50.], [x + 100., y + 50.])
        .bezier2_to([x + 150., y + 50.], [x + 150., y])
        .line_to(x + 200., y)
        .bezier2_to([x + 200., y + 100.], [x + 150., y + 100.])
        .line_to(x + 50., y + 100.)
        .bezier2_to([x, y + 100.], [x, y])
        .close_poly(x, y)
        .to_path()
}
