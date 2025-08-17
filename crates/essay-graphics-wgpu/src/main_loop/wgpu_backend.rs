use essay_graphics_api::{renderer::{self, App, Backend, GraphicsContext}};

use crate::{render::context::WgpuGraphicsContext, WgpuMainLoop};

// use super::main_loop::main_loop;

pub struct WgpuBackend {
    main_loop: WgpuMainLoop,
}

impl WgpuBackend {
    pub fn new() -> Self {
        Self {
            main_loop: WgpuMainLoop::new(),
        }
    }
}

impl Backend for WgpuBackend {
    fn main_loop(&mut self, figure: Box<dyn App>) -> renderer::Result<()> {
        self.main_loop.main_loop(figure)
    }
    
    fn context(&self) -> Box<dyn GraphicsContext> {
        Box::new(WgpuGraphicsContext::new())
    }
    /*
    fn renderer(&mut self) -> &dyn Renderer {
        todo!()
    }
    */
}
