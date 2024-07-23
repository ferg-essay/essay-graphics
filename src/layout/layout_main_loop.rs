use essay_graphics_wgpu::WgpuBackend;

use essay_graphics_api::{
    driver::{Backend, Drawable},
    Bounds,
};

use super::{Layout, View};

pub struct LayoutMainLoop {
    backend: Box<dyn Backend>,
    layout: Layout,

    size: (f32, f32),
    dpi: f32,
}

impl LayoutMainLoop {
    pub fn new() -> Self {
        Self {
            backend: Box::new(WgpuBackend::new()),
            size: (6.4, 4.8),
            dpi: 200.,

            layout: Layout::new(),
        }
    }

    pub fn add_view<T: Drawable + Send + 'static>(
        &mut self, 
        pos: impl Into<Bounds<Layout>>, 
        view: T
    ) -> View<T> {
        self.layout.view(pos, view)
    }

    pub fn show(self) {
        let layout = self.layout;
        let mut device = self.backend;

        device.main_loop(Box::new(layout)).unwrap();
    }

    pub fn get_width(&self) -> f32 {
        self.size.0
    }

    pub fn get_height(&self) -> f32 {
        self.size.1
    }

    pub fn get_dpi(&self) -> f32 {
        self.dpi
    }

    pub fn save(&mut self, _path: impl AsRef<std::path::Path>, _dpi: f32) {
        todo!();
        /*
        crate::wgpu::draw_hardcopy(
            self.get_width() * dpi,
            self.get_height() * dpi,
            dpi,
            &mut self.layout, 
            path
        );
        */    
    }
}
