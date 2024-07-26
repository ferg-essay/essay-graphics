use renderer::{Canvas, Drawable, Renderer};
use essay_graphics::prelude::*;
use essay_graphics::layout::LayoutMainLoop;
use essay_graphics_api::Coord;

fn main() { 
    let mut figure = LayoutMainLoop::new();

    let path = Path::<Data>::move_to(0.1, 0.1)
        .line_to(2., 0.1)
        .close_poly(2., 2.)
        .to_path();

    //figure.view(((0.0, 0.0), [0.25, 0.25]), PathView::new(path.clone()));
    figure.view((0.2, 0.2, 2., 2.) , PathView::new(path.clone(), "teal"));

    let path = Path::<Data>::move_to(0.1, 0.1)
        .line_to(0.1, 200.)
        .close_poly(200., 200.)
        .to_path();

    /*
    let path = Path::<Data>::move_to(0., 0.)
        .line_to(2., 2.)
        .close_poly(0., 2.)
        .to_path();
    */

    let view = figure.view(((0.6, 0.4), [0.25, 0.25]), PathView::new(path, "orange"));
    // let view = figure.view((), PathView::new(path));

    println!("Path {:?} ", view.read(|t| t.path()));

    figure.show();
}

struct Data;
impl Coord for Data {}

struct PathView {
    path_data: Path<Data>,
    path: Path<Canvas>,
    color: Color,
}

impl PathView {
    fn new(path: Path<Data>, color: impl Into<Color>) -> Self {
        Self {
            path_data: path,
            path: Path::move_to(0., 0.).to_path(),
            color: color.into(),
        }
    }

    fn path(&self) -> Path<Canvas> {
        self.path.clone()
    }
}

impl Drawable for PathView {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        println!("Pos {:?}", renderer.extent());
        let to_canvas = Bounds::<Data>::from([1., 1.]).affine_to(renderer.pos());

        let path = self.path_data.transform(&to_canvas);

        let mut style = PathStyleBase::new();
        style.color(self.color);

        renderer.draw_path(&path, &style)
    }
}
