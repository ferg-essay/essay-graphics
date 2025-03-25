use essay_graphics_wgpu::{WgpuBackend, WgpuHardcopy};

use essay_graphics_api::renderer::{Backend, Drawable};

pub struct MainLoop {
    device: Box<dyn Backend>,
    // layout: Layout,
    //drawable: Box<dyn Drawable>,

    size: (f32, f32),
    dpi: f32,
}

impl MainLoop {
    //pub fn new(drawable: impl Drawable + Send + 'static) -> Self {
    pub fn new() -> Self {
        Self {
            device: Box::new(WgpuBackend::new()),
            size: (6.4, 4.8),
            dpi: 200.,

            // layout: Layout::new(),
            // drawable: Box::new(drawable),
        }
    }

    pub fn width(&self) -> f32 {
        self.size.0
    }

    pub fn height(&self) -> f32 {
        self.size.1
    }

    pub fn dpi(&self) -> f32 {
        self.dpi
    }

    pub fn show(
        self,
        drawable: impl Drawable + Send + 'static,
    ) {
        let mut device = self.device;

        device.main_loop(Box::new(drawable)).unwrap();
    }

    pub fn save(
        &mut self, 
        path: impl AsRef<std::path::Path>, 
        drawable: impl Drawable + Send + 'static,
        dpi: f32
    ) {
        let width = self.width() * dpi;
        let height = self.height() * dpi;
        let mut hardcopy = WgpuHardcopy::new(width as u32, height as u32);
        hardcopy.scale_factor(dpi / 100.);
    
        let surface = hardcopy.add_surface();
        let mut drawable = drawable;
        hardcopy.draw(&mut drawable);
        hardcopy.save(surface, path, dpi as usize);
    }
}
