use essay_graphics_wgpu::WgpuBackend;

use essay_graphics_api::{
    driver::{Renderer, Backend, FigureApi},
    Bounds, Point, CanvasEvent, Canvas,
};

use super::{layout::Grid, Layout, View, ViewTrait};

//use super::config::read_config;

pub struct Figure {
    // inner: Arc<Mutex<FigureInner>>,
    device: Box<dyn Backend>,
    inner: FigureInner,
}

impl Figure {
    pub fn new() -> Self {
        Self {
            // inner: Arc::new(Mutex::new(FigureInner::new())),
            inner: FigureInner::new(),
            device: Box::new(WgpuBackend::new()),
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
        let mut device = self.device;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphId(usize);

impl GraphId {
    #[inline]
    pub fn index(&self) -> usize {
        self.0
    }
}

pub struct FigureInner {
    layout: Layout,

    size: (f32, f32),
    dpi: f32,

    // graphs: Vec<Graph>,
}

impl FigureInner {
    pub fn new() -> Self {
        Self {
            size: (6.4, 4.8),
            dpi: 200.,

            layout: Layout::new(),
        }
    }

    pub fn new_frame<T: ViewTrait + 'static>(
        &mut self, 
        pos: impl Into<Bounds<Grid>>, 
        view: T
    ) -> View<T> {
        self.layout.add_view(pos, view)
    }

    pub fn update_canvas(&mut self, canvas: &Canvas) {
        self.layout.update_canvas(canvas);
    }
}

impl FigureApi for FigureInner {
    fn draw(&mut self, renderer: &mut dyn Renderer, bounds: &Bounds<Canvas>) {
        self.layout.draw(renderer, bounds);
    }

    fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        self.layout.event(renderer, event);
    }
}

