use essay_graphics_wgpu::{WgpuBackend};

use essay_graphics_api::{output::Output, renderer::{self, App, Backend, Renderer}};

use crate::context::Context;

pub struct MainLoop {
    device: Box<dyn Backend>,
}

impl MainLoop {
    pub fn new() -> Self {
        Self {
            device: Box::new(WgpuBackend::new()),
        }
    }

    pub fn show(
        self,
        app: impl FnMut(&Context) + Send + Sync + 'static,
    ) {
        let mut device = self.device;

        let context = Context::new(device.context());
        let mut app = Box::new(app);

        /*
        device.main_loop(Box::new(move |ui: &mut dyn Renderer| {
            context.run(ui, |ctx| (app)(ctx))
        })).unwrap();
        */
        device.main_loop(Box::new(ContextApp(context, app))).unwrap();
    }
}

struct ContextApp(Context, Box<dyn FnMut(&Context) + Send + Sync>);

impl App for ContextApp {
    fn render(&mut self, ui: &mut dyn Renderer) -> renderer::Result<Output> {
        self.0.run(ui, |ctx| {
            (self.1)(ctx);
        })
    }
}
