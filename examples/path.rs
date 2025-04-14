use essay_graphics_api::color::Grey;
use renderer::Renderer;
use essay_graphics::prelude::*;
use essay_graphics::layout::MainLoop;
use essay_graphics_api::Coord;

fn main() { 
    MainLoop::new().show(Box::new(move |ui: &mut dyn Renderer| {
        let path = Path::<Data>::move_to(0.25, 0.25)
            .line_to(0.5, 0.25)
            .close_poly(0.25, 0.5)
            .to_path();

        let to_canvas = Bounds::<Data>::new([0., 0.], [1., 1.]).affine_to(ui.extent());

        let path = path.transform(&to_canvas);

        let mut style = PathStyle::new();

        style.line_width(3.);
        style.edge_color("azure");
        style.face_color(Grey(0.9));

        ui.draw_path(&path, &style)
    }));
}

struct Data;
impl Coord for Data {}
