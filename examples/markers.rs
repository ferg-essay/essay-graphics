use essay_tensor::{tensor, tf32, Tensor};
use renderer::{Drawable, Renderer};
use essay_graphics::prelude::*;
use essay_graphics::layout::LayoutMainLoop;
use essay_graphics_api::Coord;

fn main() { 
    let mut figure = LayoutMainLoop::new();

    let path = Path::<Data>::move_to(0.0, 0.0)
        .line_to(0.1, 0.0)
        .close_poly(0.1, 0.1)
        .to_path();

    let markers = tf32!([
        [0.25, 0.25],
        [0.75, 0.75]
    ]);

    let colors = tensor!([
        Color::from("red").to_rgba(),
        Color::from("teal").to_rgba(),
    ]);

    let scale = tensor!([
        1.,
        0.5,
    ]);

    figure.view((), PathView::new(path, markers, colors, scale));

    figure.show();
}

struct Data;
impl Coord for Data {}

struct PathView {
    path: Path<Data>,
    markers: Tensor,
    colors: Tensor<u32>,
    scale: Tensor,
}

impl PathView {
    fn new(path: Path<Data>, markers: Tensor, colors: Tensor<u32>, scale: Tensor) -> Self {
        assert_eq!(markers.rows(), colors.len());

        Self {
            path,
            markers,
            colors,
            scale,
        }
    }
}

impl Drawable for PathView {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        let to_canvas = Bounds::<Data>::new((0., 0.), (1., 1.))
            .affine_to(renderer.extent());

        let path = self.path.transform(&to_canvas);
        let xy = to_canvas.transform(&self.markers);

        let style = PathStyleBase::new();
        renderer.draw_markers(&path, &xy, &self.scale, &self.colors, &style)
    }
}
