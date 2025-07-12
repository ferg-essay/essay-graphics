use essay_graphics_api::path_style::MeshStyle;
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

    let colors = vec![
        Color::from("red"),
        Color::from("teal"),
    ];

    let scale = ten!([
        1.,
        0.5,
    ]);

    MainLoop::new().show(move |ui: &mut dyn Renderer| {
        let to_canvas = Bounds::<Data>::new([0., 0.], [1., 1.])
            .affine_to(ui.extent());

        let path = to_canvas.transform_path(&path);
        let xy = to_canvas.transform(&markers);

        let styles: Vec<MeshStyle> = xy.iter_row()
            .zip(colors.as_slice().iter())
            .zip(scale.iter())
            .map(|((xy, color), scale)| {
                let affine = affine2d::scale(*scale as f32, *scale as f32)
                    .translate(xy[0], xy[1]);

                MeshStyle::from((*color, affine))
            })
            .collect();

        let mut style = PathStyle::new();
        style.face_color(Color::none());
        style.edge_color(Color::black());
        style.line_width(1.5);

        ui.draw_markers(&path, &style, styles.as_slice())
    })
}

struct Data;
impl Coord for Data {}
