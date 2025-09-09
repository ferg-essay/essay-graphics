use renderer::{Canvas, Drawable, Renderer};
use essay_graphics::prelude::*;
use essay_graphics::layout::{MainLoop, Page, View};
use essay_graphics_api::Coord;

fn main() { 
    let path_a = Path::<Data>::move_to(0., 0.)
        .line_to(2., 0.)
        .close_poly(2., 2.)
        .to_path();

    let path_b = Path::<Data>::move_to(0., 0.)
        .line_to(0., 200.)
        .close_poly(200., 200.)
        .to_path();

    let view = View::from(PathView::new(path_b.clone(), "blue"));

    let page = Page::build(|ui| {
        ui.horizontal(|ui| {
            ui.view(PathView::new(path_a.clone(), "teal"));
            //ui.view(PathView::new(path_b.clone(), "orange"));
            ui.view(|ui: &mut dyn Renderer| Ok(()));
        });
        /*
        ui.horizontal_size(3., |ui| {
            ui.view_size([3., 3.], PathView::new(path_a.clone(), "red"));
            ui.view(view.drawable());
        });
        */
    });

    // builder.view(view_a);

    println!("Path {:?} ", view.read(|t| t.path()));

    MainLoop::new().show(page);
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
        let to_canvas = Bounds::<Data>::from([1., 1.]).affine_to(renderer.pos());

        let path = self.path_data.transform(&to_canvas);

        let mut style = PathStyle::new();
        style.color(self.color);
        println!("Draw {:?} {:?}", self.color, renderer.pos());

        renderer.draw_path(&path, &style)
    }
}
