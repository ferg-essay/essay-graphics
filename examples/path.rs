use renderer::{Canvas, Drawable, Event, Renderer};
use essay_graphics::prelude::*;
use essay_graphics::layout::LayoutMainLoop;
use essay_graphics_api::Coord;

fn main() { 
    let mut figure = LayoutMainLoop::new();

    let path = Path::<Data>::move_to(0.25, 0.25)
        .line_to(0.5, 0.25)
        .close_poly(0.25, 0.5)
        .to_path();

    figure.add_view((), PathView::new(path));

    let path = Path::<Data>::move_to(0.25, 0.25)
        .line_to(0.5, 0.25)
        .close_poly(0.25, 0.5)
        .to_path();

    let view = figure.add_view((), PathView::new(path));

    println!("Path {:?} ", view.read(|t| t.path()));

    figure.show();
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
    fn event(&mut self, _renderer: &mut dyn Renderer, _event: &Event) {
        /*
        if let Event::Resize(pos) = event {
            let to_canvas = Bounds::<Data>::new((0., 0.), (1., 1.)).affine_to(pos);

            self.path = self.path_data.transform(&to_canvas);
        }
        */
    }

    fn draw(&mut self, renderer: &mut dyn Renderer) {
        println!("Pos {:?}", renderer.extent());
        let to_canvas = Bounds::<Data>::new((0., 0.), (1., 1.)).affine_to(renderer.extent());

        let path = self.path_data.transform(&to_canvas);

        let style = PathStyleBase::new();
        renderer.draw_path(&path, &style, &Clip::None).unwrap();
    }
}
