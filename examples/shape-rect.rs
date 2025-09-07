use essay_graphics_api::{renderer::{Renderer}, CapStyle, Color, JoinStyle, Path, PathStyle, Point, Shapes, Size};
use essay_graphics::layout::MainLoop;

fn main() { 
    MainLoop::new().show(Box::new(|ui: &mut dyn Renderer| {
        let mut path_style = PathStyle::new();
        path_style.color("azure");

        let size = Size::new(500., 500.);
        ui.draw_shape(
            &Shapes::rect(Point::new(50., 50.), size, 60., Color::from("azure")),
        )?;

        let size = Size::new(500., 500.);
        ui.draw_shape(
            &Shapes::quad(
                Point::new(50., 600.), 
                size, 
                60., 
                Color::from("amber"),
                50., 
                Color::from("azure"),
            ),
        )?;

        let size = Size::new(60., 60.);

        ui.draw_path(
            &Path::move_to(600., 50.)
                .line_to(600. + size.width, 50.)
                .line_to(600. + size.width, 50. + size.height)
                .close_poly(600., 50. + size.height).to_path(),
            &path_style,
        )
    }));
}
