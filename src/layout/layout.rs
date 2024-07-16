use core::{alloc, fmt};
use std::{any::{Any, TypeId}, marker::PhantomData, mem::ManuallyDrop, ptr::NonNull, sync::{Arc, Mutex}};

//use downcast_rs::Downcast;
//use downcast_rs::impl_downcast;
use essay_graphics_api::{Bounds, Canvas, Point, Coord, driver::Renderer, CanvasEvent};

#[derive(Clone)]
pub struct Layout(Arc<Mutex<LayoutInner>>);

impl Layout {
    pub(crate) fn new() -> Layout {
        Layout(Arc::new(Mutex::new(LayoutInner::new())))
    }
}

impl Layout {
    #[inline]
    pub fn bounds(&self) -> Bounds<Grid> {
        self.0.lock().unwrap().bounds().clone()
    }

    #[inline]
    pub fn add_view<V: ViewTrait + 'static>(
        &mut self, 
        pos: impl Into<Bounds<Grid>>,
        view: V, 
    ) -> View<V> {
        let mut pos : Bounds<Grid> = pos.into();

        if pos.is_zero() || pos.is_none() {
            let layout = self.bounds();
            pos = Bounds::new(
                Point(0., layout.ymax()),
                Point(1., layout.ymax() + 1.),
            );
        }

        let id = self.0.lock().unwrap().add_view(pos, view);

        View::new(id, self.clone())
    }

    pub(crate) fn update_canvas(&mut self, canvas: &Canvas) {
        self.0.lock().unwrap().layout(&canvas);
    }

    pub(crate) fn draw(&mut self, renderer: &mut dyn Renderer, bounds: &Bounds<Canvas>) {
        self.0.lock().unwrap().draw(renderer, bounds);
    }

    pub(crate) fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        self.0.lock().unwrap().event(renderer, event);
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

pub struct Grid;
impl Coord for Grid {}

struct LayoutInner {
    sizes: LayoutSizes,
    
    extent: Bounds<Grid>,

    views: Vec<LayoutBox>,
}

impl LayoutInner {
    pub fn new() -> Self {
        let sizes = LayoutSizes::new();

        Self {
            sizes,            

            extent: Bounds::unit(),

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

        self.extent = self.extent.union(&pos);

        let id = ViewId(self.views.len());

        // let frame = Frame::new(id, Box::new(frame));

        self.views.push(LayoutBox::new(pos, view));

        // self.frame_mut(id)

        id
    }

    /*
    #[inline]
    pub fn frame(&self, id: FrameId) -> &Frame {
        self.frames[id.index()].frame()
    }

    #[inline]
    pub fn frame_mut(&mut self, id: FrameId) -> &mut Frame {
        self.frames[id.index()].frame_mut()
    }
    */
    #[inline]
    fn pos(&self, id: ViewId) -> &Bounds<Canvas> {
        self.views[id.index()].pos()
    }

    fn bounds(&self) -> &Bounds<Grid> {
        &self.extent
    }

    fn layout(&mut self, canvas: &Canvas) {
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

        for item in &mut self.views {
            let pos_layout = &item.pos_grid;

            let pos = Bounds::new(
                Point(x_min + dw * pos_layout.xmin(), y_max - dh * pos_layout.ymax()),
                Point(x_min + dw * pos_layout.xmax(), y_max - dh * pos_layout.ymin()),
            );

            item.update(&pos, canvas);
        }
    }

    fn layout_bounds(&self) -> Bounds<Grid> {
        let mut bounds = Bounds::unit();

        for item in &self.views {
            bounds = bounds.union(&item.pos_grid);
        }

        bounds
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        bounds: &Bounds<Canvas>,
    ) {
        let canvas = Canvas::new(bounds.clone(), renderer.to_px(1.));
        
        self.layout(&canvas);

        for item in &mut self.views {
            item.draw(renderer);
        }
    }

    fn event(&mut self, _renderer: &mut dyn Renderer, event: &CanvasEvent) {
        for item in &mut self.views {
            // let frame = item.frame_mut();

            //if frame.pos().contains(event.point()) {
                // frame.event(renderer, event);
            //}
        }
    }

    fn _write<R>(&mut self, fun: impl FnOnce(&mut LayoutInner) -> R) -> R {
        fun(self)
    }
}

struct LayoutSizes {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl LayoutSizes {
    fn new() -> Self {
        let bottom = 0.;
        let top = 1.;
        let left = 0.;
        let right = 1.;

        Self {
            bottom,
            top, 
            left,
            right, 
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ViewId(usize);

impl ViewId {
    pub fn index(&self) -> usize {
        self.0
    }
}

struct LayoutBox {
    pos_grid: Bounds<Grid>,
    pos_canvas: Bounds<Canvas>,

    ptr: Box<dyn Any>,
    handle: Box<dyn ViewHandleTrait>,
    // view: Box<dyn ViewTrait>,
}

impl LayoutBox {
    fn new<T: ViewTrait + 'static>(pos: Bounds<Grid>, view: T) -> Self {
        Self {
            pos_grid: pos.into(),
            pos_canvas: Bounds::unit(),

            ptr: Box::new(view),
            handle: Box::new(ViewHandle::<T>::new()),
        }
    }

    #[inline]
    fn pos(&self) -> &Bounds<Canvas> {
        &self.pos_canvas
    }

    #[inline]
    fn update(&mut self, pos: &Bounds<Canvas>, canvas: &Canvas) {
        self.pos_canvas = pos.clone();

        self.handle.update(self.ptr.as_mut(), pos, canvas);
    }

    #[inline]
    fn draw(&mut self, renderer: &mut dyn Renderer) {
        self.handle.draw(self.ptr.as_mut(), renderer);
    }

    fn read<T: 'static, R>(&self, fun: impl FnOnce(&T) -> R) -> R {
        fun(self.ptr.downcast_ref::<T>().unwrap())
    }

    fn write<T: 'static, R>(&mut self, fun: impl FnOnce(&mut T) -> R) -> R {
        fun(self.ptr.downcast_mut::<T>().unwrap())
    }
}

struct ViewHandle<V: ViewTrait> {
    marker: PhantomData<V>,
}

impl<V: ViewTrait> ViewHandle<V> {
    fn new() -> Self {
        Self {
            marker: PhantomData::default(),
        }
    }
}

trait ViewHandleTrait {
    fn update(&mut self, any: &mut dyn Any, pos: &Bounds<Canvas>, canvas: &Canvas);
    fn draw(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer);
}

impl<V: ViewTrait + 'static> ViewHandleTrait for ViewHandle<V> {
    fn update(&mut self, any: &mut dyn Any, pos: &Bounds<Canvas>, canvas: &Canvas) {
        any.downcast_mut::<V>().unwrap().update(pos, canvas)
        // self.view.draw(renderer)
    }

    fn draw(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer) {
        any.downcast_mut::<V>().unwrap().draw(renderer)
        // self.view.draw(renderer)
    }

    /*
    fn read<T: 'static, R>(&self, fun: impl FnOnce(&T) -> R) -> R{
        fun(self.view.as_any().downcast_ref::<T>().unwrap())
    }

    fn write<T: 'static, R>(&mut self, fun: impl FnOnce(&mut T) -> R) -> R{
        fun(self.view.as_any_mut().downcast_mut::<T>().unwrap())
    }
    */
}

// TODO: replace with downcast crate

struct Ptr {
    type_id: TypeId, 
    data: NonNull<u8>,
}

impl Ptr {
    fn new<T: 'static>(view: T) -> Self {
        let layout = std::alloc::Layout::new::<T>();
        let data = unsafe { std::alloc::alloc(layout) };
        let mut value = ManuallyDrop::new(view);
        let source: NonNull<u8> = NonNull::from(&mut *value).cast();

        let src = source.as_ptr();
        let count = std::mem::size_of::<T>();

        // TODO: drop

        unsafe {
            std::ptr::copy_nonoverlapping::<u8>(src, data, count);
        }

        Self {
            type_id: TypeId::of::<T>(),
            data: NonNull::new(data).unwrap(),
        }
    }

    unsafe fn deref<T: 'static>(&self) -> &T {
        assert_eq!(self.type_id, TypeId::of::<T>());

        &*self.data.as_ptr().cast::<T>()
    }

    pub unsafe fn deref_mut<T>(&self) -> &mut T
    where
        T: ViewTrait + 'static
    {
        assert_eq!(self.type_id, TypeId::of::<T>());

        &mut *self.data.as_ptr().cast::<T>()
    }
}

#[derive(Clone)]
pub struct View<T: ViewTrait> {
    id: ViewId,

    layout: Layout,

    marker: PhantomData<T>,
}

impl<T: ViewTrait + 'static> View<T> {
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

impl<T: ViewTrait> fmt::Debug for View<T> {
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

pub trait ViewTrait : Send + Sync {
    fn update(&mut self, pos: &Bounds<Canvas>, canvas: &Canvas);

    ///
    /// Sets the device bounds and propagates to children
    /// 
    /// The position for a frame is the size of the data box. The frame,
    /// axes and titles are relative to the data box.
    /// 

    fn draw(&mut self, renderer: &mut dyn Renderer);
}
