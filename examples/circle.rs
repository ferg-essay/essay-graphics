use renderer::Renderer;
use essay_graphics::prelude::*;
use essay_graphics::layout::MainLoop;
use essay_graphics_api::Coord;

fn main() { 
    let path: Path<Data> = circle()
        .scale::<Data>(0.5, 0.5)
        .translate(0.5, 0.5);

    MainLoop::new().show(move |ui: &mut dyn Renderer| {
        let to_canvas = Bounds::<Data>::from([1., 1.])
            .affine_to(ui.extent());

        let path = to_canvas.transform_path(&path);

        let mut style = PathStyle::new();
        style.alpha(0.25);
        ui.draw_path(&path, &style)
    })
}

// Via matplotlib
// Lancaster, Don.  `Approximating a Circle or an Ellipse Using Four
// Bezier Cubic Splines <https://www.tinaja.com/glib/ellipse4.pdf>`_.
fn circle() -> Path<Data> {
    let magic = 0.2652031;
    let sqrt_half = 0.5f32.sqrt();
    let magic_45 = sqrt_half * magic;

    Path::from([
        PathCode::MoveTo(Point::new(0., -1.)),
        PathCode::Bezier3(
            Point::new(magic, -1.),
            Point::new(sqrt_half - magic_45, -sqrt_half - magic_45),
            Point::new(sqrt_half, -sqrt_half),
        ),
        PathCode::Bezier3(
            Point::new(sqrt_half + magic_45, -sqrt_half + magic_45),
            Point::new(1., -magic),
            Point::new(1., 0.),
        ),
        PathCode::Bezier3(
            Point::new(1.0, magic),
            Point::new(sqrt_half + magic_45, sqrt_half - magic_45),
            Point::new(sqrt_half, sqrt_half),
        ),
        PathCode::Bezier3(
            Point::new(sqrt_half - magic_45, sqrt_half + magic_45),
            Point::new(magic, 1.),
            Point::new(0., 1.),
        ),
        PathCode::Bezier3(
            Point::new(-magic, 1.0),
            Point::new(-sqrt_half + magic_45, sqrt_half + magic_45),
            Point::new(-sqrt_half, sqrt_half),
        ),
        PathCode::Bezier3(
            Point::new(-sqrt_half - magic_45, sqrt_half - magic_45),
            Point::new(-1.0, magic),
            Point::new(-1., 0.),
        ),
        PathCode::Bezier3(
            Point::new(-1., -magic),
            Point::new(-sqrt_half - magic_45, -sqrt_half + magic_45),
            Point::new(-sqrt_half, -sqrt_half),
        ),
        PathCode::Bezier3(
            Point::new(-sqrt_half + magic_45, -sqrt_half - magic_45),
            Point::new(-magic, -1.0),
            Point::new(0., -1.),
        ),
        PathCode::ClosePoly(Point::new(0., -1.)),
    ])
}

struct Data;
impl Coord for Data {}
