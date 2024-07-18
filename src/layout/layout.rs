use core::fmt;
use std::{any::Any, marker::PhantomData, sync::{Arc, Mutex}};

use essay_graphics_api::{driver::{Drawable, Renderer}, Bounds, Canvas, CanvasEvent, Coord, Point};

#[derive(Clone)]
pub struct Layout(Arc<Mutex<LayoutInner>>);

impl Layout {
    pub fn new() -> Layout {
        Layout(Arc::new(Mutex::new(LayoutInner::new())))
    }
}

impl Layout {
    #[inline]
    pub fn bounds(&self) -> Bounds<Grid> {
        self.0.lock().unwrap().grid_bounds().clone()
    }

    #[inline]
    pub fn add_view<V: ViewTrait + 'static>(
        &mut self, 
        pos: impl Into<Bounds<Grid>>,
        view: V, 
    ) -> ViewHandle<V> {
        let mut pos : Bounds<Grid> = pos.into();

        if pos.is_zero() || pos.is_none() {
            let layout = self.bounds();
            pos = Bounds::new(
                Point(0., layout.ymax()),
                Point(1., layout.ymax() + 1.),
            );
        }

        let id = self.0.lock().unwrap().add_view(pos, view);

        ViewHandle::new(id, self.clone())
    }

    #[inline]
    fn pos(&self, id: ViewId) -> Bounds<Canvas> {
        self.0.lock().unwrap().pos(id).clone()
    }

    #[inline]
    fn read<T: ViewTrait + 'static, R>(&self, id: ViewId, fun: impl FnOnce(&T) -> R) -> R {
        self.0.lock().unwrap().views[id.0].read(fun)
    }

    #[inline]
    fn write<T: ViewTrait + 'static, R>(&self, id: ViewId, fun: impl FnOnce(&mut T) -> R) -> R {
        self.0.lock().unwrap().views[id.0].write(fun)
    }
}

impl Drawable for Layout {
    //#[inline]
    //fn update(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
    //    self.0.lock().unwrap().layout(renderer, pos);
    //}

    #[inline]
    fn draw(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        self.0.lock().unwrap().draw(renderer, pos);
    }

    #[inline]
    fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        self.0.lock().unwrap().event(renderer, event);
    }
}

pub struct Grid;
impl Coord for Grid {}

struct LayoutInner {
    views: Vec<ViewBox>,
}

impl LayoutInner {
    pub fn new() -> Self {
        Self {
            views: Vec::new(),
        }
    }

    fn add_view(
        &mut self, 
        pos: impl Into<Bounds<Grid>>,
        view: impl ViewTrait + 'static
    ) -> ViewId {
        let pos = pos.into();

        assert!(pos.xmin() >= 0.);
        assert!(pos.ymin() >= 0.);

        // arbitrary limit for now
        assert!(pos.width() <= 11.);
        assert!(pos.height() <= 11.);

        // self.extent = self.extent.union(&pos);

        let id = ViewId(self.views.len());

        self.views.push(ViewBox::new(pos, view));

        id
    }

    #[inline]
    fn pos(&self, id: ViewId) -> &Bounds<Canvas> {
        self.views[id.index()].pos()
    }

    fn layout(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        let bounds = self.grid_bounds();
        
        assert!(bounds.xmin() == 0.);
        assert!(bounds.ymin() == 0.);

        assert!(1. <= bounds.width() && bounds.width() <= 11.);
        assert!(1. <= bounds.height() && bounds.height() <= 11.);
        
        let x_min = pos.xmin();
        let x_max = pos.xmin() + pos.width();
        
        let y_min = pos.ymin();
        let y_max = pos.ymin() + pos.height();

        // TODO: nonlinear grid sizes
        let h = y_max - y_min; // canvas.height();
        let w = x_max - x_min; // canvas.height();
        let dw = w / bounds.width();
        let dh = h / bounds.height();

        for item in &mut self.views {
            let pos_layout = &item.pos_grid;

            let pos = Bounds::new(
                Point(x_min + dw * pos_layout.xmin(), y_max - dh * pos_layout.ymax()),
                Point(x_min + dw * pos_layout.xmax(), y_max - dh * pos_layout.ymin()),
            );

            item.pos_canvas = pos.clone();

            item.event(renderer, &CanvasEvent::Resize(pos));
        }
    }

    fn grid_bounds(&self) -> Bounds<Grid> {
        let mut bounds = Bounds::zero();

        for item in &self.views {
            bounds = bounds.union(&item.pos_grid);
        }

        bounds
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        pos: &Bounds<Canvas>,
    ) {
        for item in &mut self.views {
            item.draw(renderer, pos);
        }
    }

    fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        match event {
            CanvasEvent::Resize(bounds) => {
                self.layout(renderer, bounds);
            },
            _ => {
                let point = event.point();

                for view in &mut self.views {
                    if view.pos().contains(point) {
                        view.event(renderer, event);
                    }
                }
            }
        }
    }
}

struct ViewBox {
    pos_grid: Bounds<Grid>,
    pos_canvas: Bounds<Canvas>,

    ptr: Box<dyn Any>,
    handle: Box<dyn ViewHandleTrait>,
}

impl ViewBox {
    fn new<T: ViewTrait + 'static>(pos: Bounds<Grid>, view: T) -> Self {
        Self {
            pos_grid: pos.into(),
            pos_canvas: Bounds::unit(),

            ptr: Box::new(view),
            handle: Box::new(ViewTraitHandle::<T>::new()),
        }
    }

    #[inline]
    fn pos(&self) -> &Bounds<Canvas> {
        &self.pos_canvas
    }

    // #[inline]
    //fn update(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
    //    self.pos_canvas = pos.clone();
    //
    //    self.handle.update(self.ptr.as_mut(), renderer, pos);
    //}

    #[inline]
    fn draw(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        self.handle.draw(self.ptr.as_mut(), renderer, pos);
    }

    #[inline]
    fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        self.handle.event(self.ptr.as_mut(), renderer, event);
    }

    fn read<T: 'static, R>(&self, fun: impl FnOnce(&T) -> R) -> R {
        fun(self.ptr.downcast_ref::<T>().unwrap())
    }

    fn write<T: 'static, R>(&mut self, fun: impl FnOnce(&mut T) -> R) -> R {
        fun(self.ptr.downcast_mut::<T>().unwrap())
    }
}

struct ViewTraitHandle<V: ViewTrait> {
    marker: PhantomData<V>,
}

impl<V: ViewTrait> ViewTraitHandle<V> {
    fn new() -> Self {
        Self {
            marker: PhantomData::default(),
        }
    }
}

trait ViewHandleTrait {
    //fn update(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>);
    fn draw(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>);
    fn event(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, event: &CanvasEvent);
}

impl<V: ViewTrait + 'static> ViewHandleTrait for ViewTraitHandle<V> {
    //fn update(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
    //    any.downcast_mut::<V>().unwrap().update_pos(renderer, pos)
    //}

    fn draw(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        any.downcast_mut::<V>().unwrap().draw(renderer, pos)
    }

    fn event(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        any.downcast_mut::<V>().unwrap().event(renderer, event)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ViewId(usize);

impl ViewId {
    pub fn index(&self) -> usize {
        self.0
    }
}

pub struct ViewHandle<T: ViewTrait> {
    id: ViewId,

    layout: Layout,

    marker: PhantomData<T>,
}

impl<T: ViewTrait> Clone for ViewHandle<T> {
    fn clone(&self) -> Self {
        Self { 
            id: self.id.clone(), 
            layout: self.layout.clone(), 
            marker: Default::default(),
        }
    }
}

impl<T: ViewTrait + 'static> ViewHandle<T> {
    fn new(id: ViewId, layout: Layout) -> Self {
        let frame = Self {
            id,
            layout,
            marker: Default::default(),
        };

        frame
    }

    #[inline]
    pub fn pos(&self) -> Bounds<Canvas> {
        self.layout.pos(self.id)
    }

    #[inline]
    pub fn read<R>(&self, fun: impl FnOnce(&T) -> R) -> R {
        self.layout.read(self.id, fun)
    }

    #[inline]
    pub fn write<R>(&self, fun: impl FnOnce(&mut T) -> R) -> R {
        self.layout.write(self.id, fun)
    }
}

impl<T: ViewTrait> fmt::Debug for ViewHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pos = self.layout.pos(self.id);

        write!(f, "View[{}]({},{}; {}x{})",
            self.id.index(),
            pos.xmin(),
            pos.ymin(),
            pos.width(),
            pos.height(),
        )
    }
}

pub trait ViewTrait { // }: Send + Sync {
    ///
    /// Update the view position. The position will be the same as any
    /// subsequent draw.
    /// 
    //#[allow(unused_variables)]
    //fn update_pos(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
    //}

    ///
    /// Draws the view in the renderer.
    /// 
    /// Pos is the same as the most recent CanvasEvent::Resize pos.
    /// 
    fn draw(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>);

    #[allow(unused_variables)]
    fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
    }
}