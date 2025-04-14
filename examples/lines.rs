use essay_graphics_api::{renderer::{Canvas, Renderer}, CapStyle, JoinStyle, Path, PathStyle, Point};
use essay_graphics::layout::MainLoop;

fn main() { 
    MainLoop::new().show(Box::new(|ui: &mut dyn Renderer| {
        let mut path_style = PathStyle::new();
        path_style.line_width(10.);
        // alpha shows overlaps
        path_style.alpha(0.2);
        
        let path = right_angle([100., 100.]);
        ui.draw_path(&path, &path_style)?;

        let path = right_angle([300., 100.]);
        path_style.join_style(JoinStyle::Round);
        ui.draw_path(&path, &path_style)?;

        let path = right_angle([500., 100.]);
        path_style.cap_style(CapStyle::Round);
        ui.draw_path(&path, &path_style)?;
        
        path_style.cap_style(CapStyle::Butt);

        let path = bezier_one_side([100., 300.]);
        ui.draw_path(&path, &path_style)?;

        let path = bezier_corner([300., 300.]);
        ui.draw_path(&path, &path_style)?;

        let path = bezier_zig_zag([500., 300.]);
        ui.draw_path(&path, &path_style)?;

        Ok(())
    }));
}

fn right_angle(point: impl Into<Point>) -> Path<Canvas> {
    let Point(x, y) = point.into();

    Path::move_to(x, y)
        .line_to(x + 100., y)
        .line_to(x + 100., y + 100.)
        .to_path()
}

fn bezier_one_side(point: impl Into<Point>) -> Path<Canvas> {
    let Point(x, y) = point.into();

    Path::move_to(x - 50., y + 50.)
        .line_to(x, y)
        .bezier2_to([x + 50., y - 50.], [x + 100., y])
        .line_to(x + 150., y + 50.)
        .to_path()
}

fn bezier_corner(point: impl Into<Point>) -> Path<Canvas> {
    let Point(x, y) = point.into();

    Path::move_to(x, y)
        .bezier2_to([x + 50., y - 50.], [x + 100., y])
        .bezier2_to([x + 150., y + 50.], [x + 100., y + 100.])
        .to_path()
}

fn bezier_zig_zag(point: impl Into<Point>) -> Path<Canvas> {
    let Point(x, y) = point.into();

    Path::move_to(x, y)
        .bezier2_to([x + 50., y - 50.], [x + 100., y])
        .bezier2_to([x + 150., y + 50.], [x + 200., y])
        .to_path()
}
