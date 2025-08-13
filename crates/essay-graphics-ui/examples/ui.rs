use std::sync::Arc;

use essay_graphics_api::{renderer::{self, Canvas, Drawable, Renderer}, Bounds, Path, PathStyle};
use essay_graphics_ui::{page::MainLoop, ui::{CentralPanel, Context, UiView}};

fn main() { 
    let cxt = Context::new();
    
    MainLoop::new().show(ViewBox(Box::new(move |ui| {
        let response = cxt.run_ui(ui, |ui| {
            ui.label("hello, world");
            ui.label("second label");
            /*
            CentralPanel::default().show(ui, |ui| {
                let style = Arc::new(PathStyle::new());

                ui.painter_mut().add(DrawPath::rect(([100., 100.], [200., 200.]), &style));
            })
            */
        });

        response.tooltip("Testing tooltip");

        /*
        ctx.run_ui(ui, |ui| {
            let style = Arc::new(PathStyle::new());

            ui.painter_mut().add(DrawPath::rect(([100., 100.], [200., 200.]), &style));
        });
        */

        Ok(())
    })));
}

struct DrawPath {
    path: Path<Canvas>,
    style: Arc<PathStyle>,
}

impl DrawPath {
    fn rect(bounds: impl Into<Bounds<Canvas>>, style: &Arc<PathStyle>) -> Self {
        let bounds = bounds.into();

        Self {
            path: Path::move_to(bounds.x0(), bounds.y0())
                .line_to(bounds.x0(), bounds.y1())
                .line_to(bounds.x1(), bounds.y1())
                .line_to(bounds.x1(), bounds.y0())
                .line_to(bounds.x0(), bounds.y0())
                .to_path(),
            style: style.clone(),
        }
    }
}

impl Drawable for DrawPath {
    fn draw(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        ui.draw_path(&self.path, self.style.as_ref())
    }
}

struct ViewBox(Box<dyn FnMut(&mut dyn Renderer)->renderer::Result<()> + Send + Sync>);

impl Drawable for ViewBox {
    fn draw(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        (self.0)(ui)
    }
}
