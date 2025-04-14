use essay_tensor::tensor::Tensor;
use renderer::{Drawable, Renderer};
use essay_graphics::{layout::MainLoop, prelude::*};

fn main() { 
    MainLoop::new().show(Mesh2dColorView::new());
}

struct Mesh2dColorView {
    mesh: Mesh2dColor,
}

impl Mesh2dColorView {
    fn new() -> Self {
        Self {
            mesh: Mesh2dColor::new(),
        }
    }

    fn resize(&mut self, ui: &mut dyn Renderer) {
        let pos = ui.pos();

        let (x0, y0) = (pos.xmin(), pos.ymin());
        let (w, h) = (pos.width(), pos.height());
        let (x1, y1) = (x0 + w, y0 + h);

        let teal = Color::from("teal");
        let red  = Color::from("red");
        let blue = Color::from("blue");
        let orange = Color::from("orange");

        let mut mesh = Mesh2dColor::new();

        mesh.triangle(
            ([x0, y0], teal),
            ([x0, y1], teal),
            ([x1, y1], teal),
        );

        mesh.triangle(
            ([x0, y0], red),
            ([x1, y0], blue),
            ([x1, y1], orange),
        );

        self.mesh = mesh;
    }
}

impl Drawable for Mesh2dColorView {
    fn draw(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        self.resize(ui);

        ui.draw_mesh2d_color(&self.mesh)?;
        
        Ok(())
    }
}
