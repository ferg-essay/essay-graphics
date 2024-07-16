use essay_graphics::{artist::{patch::Patch, PathStyle}, frame::Data, layout::{LayoutMainLoop, ViewTrait}, prelude::*};

fn main() { 
    let mut figure = LayoutMainLoop::new();

    let path = Path::<Data>::move_to(0.25, 0.25)
        .line_to(0.5, 0.25)
        .close_poly(0.25, 0.5)
        .to_path();

    let view = figure.add_view((), TestPatch::new(path));

    /*
    let path = Path::<Data>::new(vec![
        PathCode::MoveTo(Point(1., 1.)),
        PathCode::LineTo(Point(11., 11.)),
        PathCode::LineTo(Point(6., 11.)),
        PathCode::LineTo(Point(7.7, 10.)),
        PathCode::ClosePoly(Point(0., 2.)),
    ]);
    */

    println!("Path {:?} ", view.read(|t| t.path()));

    figure.show();
}

struct TestPatch {
    path_data: Path<Data>,
    path: Path<Canvas>,
}

impl TestPatch {
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

impl ViewTrait for TestPatch {
    fn update(&mut self, pos: &Bounds<Canvas>, canvas: &Canvas) {
        let to_canvas = Bounds::<Data>::new((0., 0.), (1., 1.)).affine_to(pos);

        self.path = self.path_data.transform(&to_canvas);
    }

    fn draw(&mut self, renderer: &mut dyn driver::Renderer) {

        let style = PathStyle::new();

        renderer.draw_path(&self.path, &style, &Clip::None).unwrap();
    }
}
