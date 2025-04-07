use essay_tensor::ten;
use renderer::Renderer;
use essay_graphics::prelude::*;
use essay_graphics::layout::MainLoop;
use essay_graphics_api::Coord;

fn main() { 
    let path = Path::<Data>::move_to(0.0, 0.0)
        .line_to(0.1, 0.0)
        .close_poly(0.1, 0.1)
        .to_path();

    let markers = ten![
        [0.25, 0.25],
        [0.75, 0.75]
    ];

    let colors = ten![
        Color::from("red").to_rgba(),
        Color::from("teal").to_rgba(),
    ];

    let scale = ten!([
        1.,
        0.5,
    ]);

    MainLoop::new().show(move |ui: &mut dyn Renderer| {
        let to_canvas = Bounds::<Data>::new((0., 0.), (1., 1.))
            .affine_to(ui.extent());

        let path = to_canvas.transform_path(&path);
        let xy = to_canvas.transform(&markers);

        let style = PathStyleBase::new();
        ui.draw_markers(&path, &xy, &scale, &colors, &style)
    })
}

struct Data;
impl Coord for Data {}
