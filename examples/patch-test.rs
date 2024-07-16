use essay_graphics::{artist::{patch::Patch, PathStyle}, frame::Data, layout::{Figure, ViewTrait}, prelude::*};

fn main() { 
    let mut figure = Figure::new();
    let mut view = figure.add_view((), TestPatch::default());

    let path = Path::<Data>::new(vec![
        PathCode::MoveTo(Point(1., 1.)),
        PathCode::LineTo(Point(11., 11.)),
        PathCode::LineTo(Point(6., 11.)),
        PathCode::LineTo(Point(7.7, 10.)),
        PathCode::ClosePoly(Point(0., 2.)),
    ]);
    //graph.artist(Patch::new(path));

    //graph.artist(Patch::new(path)).color("teal").edge_color("black");

    //graph.aspect(1.);
    //graph.xlim(0., 20.);
    //graph.ylim(0., 20.);

    println!("Path {:?} ", view.read(|t| t.path()));

    figure.show();
}

struct TestPatch {
    path: Path<Canvas>,
}

impl TestPatch {
    fn path(&self) -> Path<Canvas> {
        self.path.clone()
    }
}

impl Default for TestPatch {
    fn default() -> Self {
        Self {  
            path: Path::move_to(0., 0.).to_path(),
        }
    }
}

impl ViewTrait for TestPatch {
    fn update(&mut self, pos: &Bounds<Canvas>, _canvas: &Canvas) {
        let to_canvas = Bounds::<Data>::new((0., 0.), (1., 1.)).affine_to(pos);

        let path = Path::<Data>::move_to(0.25, 0.25)
            .line_to(0.5, 0.25)
            .close_poly(0.25, 0.5)
            .to_path();

        self.path = path.transform(&to_canvas);
    }

    fn draw(&mut self, renderer: &mut dyn driver::Renderer) {

        let style = PathStyle::new();

        renderer.draw_path(&self.path, &style, &Clip::None);
    }
}
