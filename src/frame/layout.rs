use std::sync::{Arc, Mutex};

use essay_graphics_api::{Bounds, Canvas, Point, Coord, driver::Renderer, CanvasEvent};

use crate::{graph::{Config, ConfigArc}, artist::Artist};

use super::{Frame, ArtistId, Data};

#[derive(Clone)]
pub struct LayoutArc(Arc<Mutex<Layout>>);

pub struct Layout {
    config: ConfigArc,

    sizes: LayoutSizes,
    
    extent: Bounds<Layout>,

    frames: Vec<LayoutBox>,
}

impl Layout {
    pub fn new(config: Config) -> Self {
        let sizes = LayoutSizes::new(&config);

        Self {
            config: config.into_arc(),

            sizes,            

            extent: Bounds::unit(),

            frames: Vec::new(),
        }
    }

    #[inline]
    pub fn config(&self) -> &ConfigArc {
        &self.config
    }

    pub fn add_frame(&mut self, bound: impl Into<Bounds<Layout>>) -> &mut Frame {
        let bound = bound.into();

        assert!(bound.xmin() >= 0.);
        assert!(bound.ymin() >= 0.);

        // arbitrary limit for now
        assert!(bound.width() <= 11.);
        assert!(bound.height() <= 11.);

        self.extent = self.extent.union(&bound);

        let id = FrameId(self.frames.len());

        let frame = Frame::new(id, self.config());

        self.frames.push(LayoutBox::new(frame, bound));

        self.frame_mut(id)
    }

    #[inline]
    pub fn frame(&self, id: FrameId) -> &Frame {
        self.frames[id.index()].frame()
    }

    #[inline]
    pub fn frame_mut(&mut self, id: FrameId) -> &mut Frame {
        self.frames[id.index()].frame_mut()
    }

    pub fn bounds(&self) -> &Bounds<Layout> {
        &self.extent
    }

    pub fn layout(&mut self, canvas: &Canvas) {
        let bounds = self.layout_bounds();

        assert!(bounds.xmin() == 0.);
        assert!(bounds.ymin() == 0.);

        assert!(1. <= bounds.width() && bounds.width() <= 11.);
        assert!(1. <= bounds.height() && bounds.height() <= 11.);
        
        let x_min = canvas.x_min() + canvas.width() * self.sizes.left;
        let x_max = canvas.x_min() + canvas.width() * self.sizes.right;
        
        let y_min = canvas.y_min() + canvas.height() * self.sizes.bottom;
        let y_max = canvas.y_min() + canvas.height() * self.sizes.top;

        // TODO: nonlinear grid sizes
        let h = y_max - y_min; // canvas.height();
        let w = x_max - x_min; // canvas.height();
        let dw = w / bounds.width();
        let dh = h / bounds.height();

        for item in &mut self.frames {
            let pos_layout = &item.pos_layout;

            item.pos_canvas = Bounds::new(
                Point(x_min + dw * pos_layout.xmin(), y_max - dh * pos_layout.ymax()),
                Point(x_min + dw * pos_layout.xmax(), y_max - dh * pos_layout.ymin()),
            );
        }
    }

    fn layout_bounds(&self) -> Bounds<Layout> {
        let mut bounds = Bounds::unit();

        for item in &self.frames {
            bounds = bounds.union(&item.pos_layout);
        }

        bounds
    }

    pub(crate) fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        bounds: &Bounds<Canvas>,
    ) {
        let canvas = Canvas::new(bounds.clone(), renderer.to_px(1.));
        
        self.layout(&canvas);

        for item in &mut self.frames {
            item.draw(renderer, &canvas);
        }
    }

    pub(crate) fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        for item in &mut self.frames {
            let frame = item.frame_mut();

            if frame.pos().contains(event.point()) {
                frame.event(renderer, event);
            }
        }
    }

    pub(crate) fn _write<R>(&mut self, fun: impl FnOnce(&mut Layout) -> R) -> R {
        fun(self)
    }

    pub(crate) fn _artist<A>(&self, id: ArtistId) -> &A
    where
        A: Artist<Data> + 'static
    {
        self.frames[id.frame().index()].frame()._data()._artist(id)
    }

    pub(crate) fn _artist_mut<A>(&mut self, id: ArtistId) -> &mut A
    where
        A: Artist<Data> + 'static
    {
        self.frames[id.frame().index()].frame_mut().data_mut().artist_mut(id)
    }
}

impl Coord for Layout {}

impl LayoutArc {
    pub(crate) fn new(config: Config) -> LayoutArc {
        LayoutArc(Arc::new(Mutex::new(Layout::new(config))))
    }
}

impl LayoutArc {
    #[inline]
    pub fn bounds(&self) -> Bounds<Layout> {
        self.0.lock().unwrap().bounds().clone()
    }

    #[inline]
    pub fn add_frame(&mut self, bound: impl Into<Bounds<Layout>>) -> FrameId {
        self.0.lock().unwrap().add_frame(bound).id()
    }

    pub fn read<R>(&self, fun: impl FnOnce(&Layout) -> R) -> R {
        fun(&self.0.lock().unwrap())
    }

    pub fn write<R>(&self, fun: impl FnOnce(&mut Layout) -> R) -> R {
        fun(&mut self.0.lock().unwrap())
    }

    pub fn _write_artist<A, R>(&self, _id: ArtistId, _fun: impl FnOnce(&Layout, &mut A) -> R) -> R
    where
        A: Artist<Data> + 'static,
    {
        //self.0.borrow_mut().write_artist(id, fun)
        todo!()
    }

    /*
    #[inline]
    pub fn frame(&self, id: FrameId) -> &Frame {
        self.0.borrow().frame(id)
    }

    #[inline]
    pub fn frame_mut(&mut self, id: FrameId) -> &mut Frame {
        self.0.borrow_mut().frame_mut(id)
    }
    */

    pub(crate) fn draw(&mut self, renderer: &mut dyn Renderer, bounds: &Bounds<Canvas>) {
        self.0.lock().unwrap().draw(renderer, bounds);
    }

    pub(crate) fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        self.0.lock().unwrap().event(renderer, event);
    }

    pub(crate) fn update_canvas(&mut self, canvas: &Canvas) {
        self.0.lock().unwrap().layout(&canvas);
    }
}

struct LayoutSizes {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl LayoutSizes {
    fn new(cfg: &Config) -> Self {
        let bottom = match cfg.get_as_type("figure.subplot", "bottom") {
            Some(value) => value,
            None => 0.
        };

        let top = match cfg.get_as_type("figure.subplot", "top") {
            Some(value) => value,
            None => 1.
        };

        let left = match cfg.get_as_type("figure.subplot", "left") {
            Some(value) => value,
            None => 0.
        };

        let right = match cfg.get_as_type("figure.subplot", "right") {
            Some(value) => value,
            None => 1.
        };

        Self {
            bottom,
            top, 
            left,
            right, 
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FrameId(usize);

impl FrameId {
    pub fn index(&self) -> usize {
        self.0
    }
}

pub struct LayoutBox {
    pos_layout: Bounds<Layout>,
    pos_canvas: Bounds<Canvas>,

    frame: Frame,
}

impl LayoutBox {
    fn new(frame: Frame, bounds: impl Into<Bounds<Layout>>) -> Self {
        Self {
            pos_layout: bounds.into(),
            pos_canvas: Bounds::unit(),

            frame,
        }
    }

    /*
    #[inline]
    pub fn id(&self) -> FrameId {
        self.frame.id()
    }

    #[inline]
    pub fn layout(&self) -> &Bounds<Layout> {
        &self.pos_layout
    }

    #[inline]
    pub fn pos_canvas(&self) -> &Bounds<Canvas> {
        &self.pos_canvas
    }
    */

    #[inline]
    pub fn frame(&self) -> &Frame {
        &self.frame
    }

    #[inline]
    pub fn frame_mut(&mut self) -> &mut Frame {
        &mut self.frame
    }

    fn draw(&mut self, renderer: &mut dyn Renderer, canvas: &Canvas) {
        self.frame.update(canvas);
        let pos_frame = self.pos_canvas.clone();
        self.frame.set_pos(&pos_frame);

        self.frame.draw(renderer);
    }
}


