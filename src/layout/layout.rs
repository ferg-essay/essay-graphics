use core::fmt;
use std::{any::Any, marker::PhantomData, sync::{Arc, Mutex}};

use essay_graphics_api::{driver::{Drawable, Renderer}, Bounds, Canvas, CanvasEvent, Coord, Point};

#[derive(Clone)]
pub struct Layout(Arc<Mutex<LayoutInner>>);

impl Layout {
    pub fn new() -> Layout {
        Layout(Arc::new(Mutex::new(LayoutInner::new())))
    }

    #[inline]
    pub fn bounds(&self) -> Bounds<Layout> {
        self.0.lock().unwrap().bounds().clone()
    }

    ///
    /// Adds a drawable view in layout coordinates, returning a view handle
    /// to the drawable.
    /// 
    /// Layout coordinates are (0, 0) lower left and (1, 1) upper right,
    /// but normalized to the minimum and maximum of all added views.
    /// ((1., 1.), (2., 2.)) is allowed, as are negative values.
    /// 
    /// If the position is unassigned, the new position will be a unit
    /// box below any current box, such as ((0., -1), (0., 0.))
    /// 
    #[inline]
    pub fn view<T: Drawable + Send + 'static>(
        &mut self, 
        pos: impl Into<Bounds<Layout>>,
        view: T, 
    ) -> View<T> {
        let mut pos = pos.into();

        // If unassigned, layout below all other views
        if pos.is_zero() || pos.is_none() {
            let layout = self.bounds();
            pos = Bounds::new(
                Point(0., layout.ymin() - 1.),
                Point(1., layout.ymax()),
            );
        }

        let id = self.0.lock().unwrap().add_view(pos, view);

        View::new(id, self.clone())
    }

    #[inline]
    fn pos(&self, id: ViewId) -> Bounds<Canvas> {
        self.0.lock().unwrap().pos(id).clone()
    }

    #[inline]
    fn read<T: 'static, R>(&self, id: ViewId, fun: impl FnOnce(&T) -> R) -> R {
        self.0.lock().unwrap().views[id.0].read(fun)
    }

    #[inline]
    fn write<T: 'static, R>(&self, id: ViewId, fun: impl FnOnce(&mut T) -> R) -> R {
        self.0.lock().unwrap().views[id.0].write(fun)
    }
}

impl Drawable for Layout {
    #[inline]
    fn draw(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        self.0.lock().unwrap().draw(renderer, pos);
    }

    #[inline]
    fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        self.0.lock().unwrap().event(renderer, event);
    }
}

impl Coord for Layout {}

struct LayoutInner {
    views: Vec<ViewItem>,
}

impl LayoutInner {
    pub fn new() -> Self {
        Self {
            views: Vec::new(),
        }
    }

    fn add_view(
        &mut self, 
        pos: impl Into<Bounds<Layout>>,
        view: impl Drawable + Send + 'static
    ) -> ViewId {
        let pos = pos.into();

        assert!(! pos.is_none() && ! pos.is_zero());
        //assert!(pos.xmin() >= 0.);
        //assert!(pos.ymin() >= 0.);

        // arbitrary limit for now
        //assert!(pos.width() <= 11.);
        //assert!(pos.height() <= 11.);

        // self.extent = self.extent.union(&pos);

        let id = ViewId(self.views.len());

        self.views.push(ViewItem::new(pos, view));

        id
    }

    #[inline]
    fn pos(&self, id: ViewId) -> &Bounds<Canvas> {
        self.views[id.index()].pos()
    }

    fn layout(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        let bounds = self.bounds();
        
        let (p_x0, p_y0) = pos.min();

        let h = pos.height();
        let w = pos.width();

        let l_x0 = bounds.xmin().min(0.);
        let l_y0 = bounds.ymin().min(0.);

        let dw = w / bounds.width().max(1.);
        let dh = h / bounds.height().max(1.);

        for item in &mut self.views {
            let (x0, y0) = item.pos_grid.min();
            let (x1, y1) = item.pos_grid.max();

            let pos = Bounds::new(
                Point(p_x0 + dw * (x0 - l_x0), p_y0 + dh * (y0 - l_y0)),
                Point(p_x0 + dw * (x1 - l_x0), p_y0 + dh * (y1 - l_y0)),
            );

            item.pos_canvas = pos.clone();

            item.event(renderer, &CanvasEvent::Resize(pos));
        }
    }

    fn bounds(&self) -> Bounds<Layout> {
        let mut bounds = Bounds::none();

        for item in &self.views {
            bounds = bounds.union(&item.pos_grid);
        }

        bounds
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        _pos: &Bounds<Canvas>,
    ) {
        for item in &mut self.views {
            let pos = item.pos().clone();
            item.draw(renderer, &pos);
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

struct ViewItem {
    pos_grid: Bounds<Layout>,
    pos_canvas: Bounds<Canvas>,

    ptr: Box<dyn Any + Send>,
    handle: Box<dyn DrawableHandle>,
}

impl ViewItem {
    fn new<T: Drawable + Send + 'static>(pos: Bounds<Layout>, view: T) -> Self {
        Self {
            pos_grid: pos.into(),
            pos_canvas: Bounds::unit(),

            ptr: Box::new(view),
            handle: Box::new(ViewDrawableHandle::<T>::new()),
        }
    }

    #[inline]
    fn pos(&self) -> &Bounds<Canvas> {
        &self.pos_canvas
    }

    #[inline]
    fn draw(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        self.handle.draw(self.ptr.as_mut(), renderer, pos);
    }

    #[inline]
    fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        self.handle.event(self.ptr.as_mut(), renderer, event);
    }

    #[inline]
    fn read<T: 'static, R>(&self, fun: impl FnOnce(&T) -> R) -> R {
        fun(self.ptr.downcast_ref::<T>().unwrap())
    }

    #[inline]
    fn write<T: 'static, R>(&mut self, fun: impl FnOnce(&mut T) -> R) -> R {
        fun(self.ptr.downcast_mut::<T>().unwrap())
    }
}

trait DrawableHandle: Send {
    fn draw(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>);
    fn event(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, event: &CanvasEvent);
}

struct ViewDrawableHandle<T: Drawable> {
    marker: PhantomData<fn(T)>,
}

impl<T: Drawable> ViewDrawableHandle<T> {
    fn new() -> Self {
        Self {
            marker: PhantomData::default(),
        }
    }
}

impl<V: Drawable + 'static> DrawableHandle for ViewDrawableHandle<V> {
    #[inline]
    fn draw(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        any.downcast_mut::<V>().unwrap().draw(renderer, pos)
    }

    #[inline]
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

pub struct View<T> {
    id: ViewId,

    layout: Layout,

    marker: PhantomData<fn(T)>,
}

impl<T: 'static> Clone for View<T> {
    fn clone(&self) -> Self {
        Self { 
            id: self.id.clone(), 
            layout: self.layout.clone(), 
            marker: Default::default(),
        }
    }
}

impl<T: 'static> View<T> {
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
    pub fn write<R>(&mut self, fun: impl FnOnce(&mut T) -> R) -> R {
        self.layout.write(self.id, fun)
    }
}

impl<T: Send> fmt::Debug for View<T> {
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

/*
pub trait ViewDrawable: Send {
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
    */