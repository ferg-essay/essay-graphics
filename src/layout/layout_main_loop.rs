use essay_graphics_wgpu::WgpuBackend;

use essay_graphics_api::{
    driver::{Renderer, Backend, FigureApi},
    Bounds, CanvasEvent, Canvas,
};

use super::{layout::Grid, Layout, View, ViewTrait};

pub struct LayoutMainLoop {
    backend: Box<dyn Backend>,
    inner: MainLoopInner,
}

impl LayoutMainLoop {
    pub fn new() -> Self {
        Self {
            inner: MainLoopInner::new(),
            backend: Box::new(WgpuBackend::new()),
        }
    }

    pub fn add_view<T: ViewTrait + 'static>(
        &mut self, 
        pos: impl Into<Bounds<Grid>>, 
        view: T
    ) -> View<T> {
        self.inner.layout.add_view(pos, view)
    }

    pub fn show(self) {
        // let mut figure = self;
        let inner = self.inner;
        let mut device = self.backend;

        device.main_loop(Box::new(inner)).unwrap();
    }

    pub fn get_width(&self) -> f32 {
        self.inner.size.0
    }

    pub fn get_height(&self) -> f32 {
        self.inner.size.1
    }

    pub fn get_dpi(&self) -> f32 {
        self.inner.dpi
    }

    pub fn save(&mut self, path: impl AsRef<std::path::Path>, dpi: f32) {
        crate::wgpu::draw_hardcopy(
            self.get_width() * dpi,
            self.get_height() * dpi,
            dpi,
            &mut self.inner, 
            path
        );    
    }
}

struct MainLoopInner {
    layout: Layout,

    size: (f32, f32),
    dpi: f32,
}

impl MainLoopInner {
    fn new() -> Self {
        Self {
            size: (6.4, 4.8),
            dpi: 200.,

            layout: Layout::new(),
        }
    }

    fn _update_canvas(&mut self, canvas: &Canvas) {
        self.layout.update_canvas(canvas);
    }
}

impl FigureApi for MainLoopInner {
    fn draw(&mut self, renderer: &mut dyn Renderer, bounds: &Bounds<Canvas>) {
        let canvas = Canvas::new(bounds.clone(), renderer.to_px(1.));
        
        self.layout.update_canvas(&canvas);

        self.layout.draw(renderer);
    }

    fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        self.layout.event(renderer, event);
    }
}

