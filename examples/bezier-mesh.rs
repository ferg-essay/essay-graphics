use renderer::{Canvas, Drawable, Renderer};
use essay_graphics::prelude::*;
use essay_graphics::layout::{MainLoop, View};
use essay_graphics_api::Coord;

fn main() { 
    MainLoop::new().show(move |ui: &mut dyn Renderer| {
        draw_mesh(ui, [100., 200.], 1., 0., "teal")?;
        draw_mesh(ui, [300., 200.], 0., 1., "teal")?;
        draw_mesh(ui, [500., 200.], 0., 0., "teal")?;
        draw_mesh(ui, [700., 200.], 1., 1., "teal")?;

        draw_mesh(ui, [100., 400.], 0.5, 0.5, "teal")?;
        draw_mesh(ui, [300., 400.], 0.25, 0.25, "teal")?;
        draw_mesh(ui, [500., 400.], 0.25, 0.0, "teal")?;
        draw_mesh(ui, [700., 400.], 0.0, 0.25, "teal")?;

        Ok(())
    });
}

fn draw_mesh(
    ui: &mut dyn Renderer, 
    xy: impl Into<Point>, 
    width_above: f32, 
    width_below: f32,
    color: impl Into<Color>,
) -> renderer::Result<()> {
    let xy = xy.into();
    let p0 = Point(200., 0.) + xy;
    let p1 = Point(100., 100.) + xy;
    let p2 = Point(0., 0.) + xy;

    let path = Path::move_to(p0.0, p0.1)
        .line_to(p1.0, p1.1)
        .line_to(p2.0, p2.1)
        .line_to(p0.0, p0.1)
        .to_path();
    let style = PathStyle::new();

    ui.draw_path(&path, &style)?;

    let mut mesh = BezierMesh2d::new();

    mesh.triangle(p0, p1, p2, width_above, width_below);

    ui.draw_bezier_mesh(&mesh, color.into())
}
