use renderer::{Canvas, Drawable, Renderer};
use essay_graphics::prelude::*;
use essay_graphics::layout::{MainLoop, View};
use essay_graphics_api::Coord;

fn main() { 
    let path = Path::<Data>::move_to(0.25, 0.25)
        .line_to(0.5, 0.25)
        .close_poly(0.25, 0.5)
        .to_path();

    let view = View::from(PathView::new(path));

    println!("Path {:?} ", view.read(|t| t.path()));

    MainLoop::new().show(view.drawable());
}

struct Data;
impl Coord for Data {}

struct PathView {
    path_data: Path<Data>,
    path: Path<Canvas>,
}

impl PathView {
    fn new(path: Path<Data>) -> Self {
        Self {
            path_data: path,
            path: Path::move_to(0., 0.).to_path(),
        }
    }

    fn path(&self) -> Path<Canvas> {
        self.path.clone()
    }
}

impl Drawable for PathView {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        println!("Pos {:?}", renderer.extent());
        let to_canvas = Bounds::<Data>::new((0., 0.), (1., 1.)).affine_to(renderer.extent());

        let path = self.path_data.transform(&to_canvas);

        let style = PathStyleBase::new();
        renderer.draw_path(&path, &style)
    }
}
