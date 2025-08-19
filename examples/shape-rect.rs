use essay_graphics_api::{renderer::{Canvas, Renderer}, CapStyle, JoinStyle, Path, PathStyle, Point, Shapes, Size};
use essay_graphics::layout::MainLoop;

fn main() { 
    MainLoop::new().show(Box::new(|ui: &mut dyn Renderer| {
        let mut path_style = PathStyle::new();
        path_style.color("azure");

        let size = Size(500., 500.);
        ui.draw_shape(
            &Shapes::Rectangle(Point(50., 50.), size, 60., 20.),
            &path_style,
        );

        let size = Size(60., 60.);

        ui.draw_path(
            &Path::move_to(600., 50.)
                .line_to(600. + size.0, 50.)
                .line_to(600. + size.0, 50. + size.1)
                .close_poly(600., 50. + size.1).to_path(),
            &path_style,
        )
    }));
}
