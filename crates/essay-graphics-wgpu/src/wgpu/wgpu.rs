use essay_graphics_api::driver::{Backend, Drawable, DeviceErr};

use crate::WgpuMainLoop;

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
    fn main_loop(&mut self, figure: Box<dyn Drawable>) -> Result<(), DeviceErr> {
        self.main_loop.main_loop(figure)
    }
    /*
    fn renderer(&mut self) -> &dyn Renderer {
        todo!()
    }
    */
}
