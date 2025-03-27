use std::{any::Any, marker::PhantomData, sync::{Arc, Mutex}};

use essay_graphics_api::{
    renderer::{Result, Canvas, Drawable, Event, Renderer}, 
    Bounds,
};

pub struct View<T> {
    // id: ViewId,

    view_arc: ViewArc,

    marker: PhantomData<fn(T)>,
}

impl<T: Drawable + Send + 'static> View<T> {
    pub(crate) fn new(drawable: T) -> Self 
    {
        let arc = Arc::new(Mutex::new(ViewPtr::new(drawable)));

        Self {
            view_arc: ViewArc(arc),
            marker: Default::default(),
        }
    }

    //#[inline]
    //pub fn id(&self) -> ViewId {
    //    self.id.clone()
    //}

    pub fn arc(&self) -> &ViewArc {
        &self.view_arc
    }

    pub fn drawable(&self) -> ViewArcDraw {
        self.view_arc.drawable()
    }

    #[inline]
    pub fn read<R>(&self, fun: impl FnOnce(&T) -> R) -> R {
        self.view_arc.0.lock().unwrap().read(fun)
    }

    #[inline]
    pub fn write<R>(&mut self, fun: impl FnOnce(&mut T) -> R) -> R {
        self.view_arc.0.lock().unwrap().write(fun)
    }
}

impl<T: 'static> Clone for View<T> {
    fn clone(&self) -> Self {
        Self { 
            // id: self.id.clone(),
            view_arc: self.view_arc.clone(), 
            marker: PhantomData,
        }
    }
}

impl<T: Drawable + Send + 'static> From<T> for View<T> {
    fn from(drawable: T) -> Self {
        View::new(drawable)
    }
}

impl<T: Drawable + Send + 'static> From<View<T>> for ViewArc {
    fn from(view: View<T>) -> Self {
        view.arc().clone()
    }
}

impl<T: Drawable + Send + 'static> From<T> for ViewArc {
    fn from(drawable: T) -> Self {
        View::new(drawable).arc().clone()
    }
}



/*
impl<T: Drawable + Send + 'static> Drawable for View<T> {
    #[inline]
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        self.view_arc.draw(renderer)
    }

    #[inline]
    fn resize(
        &mut self, 
        renderer: &mut dyn Renderer, 
        pos: &Bounds<Canvas>
    ) -> Bounds<Canvas> {
        self.view_arc.resize(renderer, pos)
    }

    #[inline]
    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) {
        self.view_arc.event(renderer, event);
    }
}
    */


#[derive(Clone)]
pub struct ViewArc(Arc<Mutex<ViewPtr>>);

impl ViewArc {
    pub fn drawable(&self) -> ViewArcDraw {
        ViewArcDraw(self.0.clone())
    }
}

#[derive(Clone)]
pub struct ViewArcDraw(Arc<Mutex<ViewPtr>>);

impl Drawable for ViewArcDraw {
    #[inline]
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        let mut view = self.0.lock().unwrap();
        
        view.draw(renderer)
    }

    #[inline]
    fn resize(
        &mut self, 
        renderer: &mut dyn Renderer, 
        pos: &Bounds<Canvas>
    ) -> Bounds<Canvas> {
        let mut view = self.0.lock().unwrap();
        
        view.resize(renderer, pos)
    }

    #[inline]
    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) {
        let mut view = self.0.lock().unwrap();
        
        view.event(renderer, event);
    }
}

struct ViewPtr {
    ptr: Box<dyn Any + Send>,
    handle: Box<dyn ViewHandleTrait>,
}

impl ViewPtr {
    fn new<T: Drawable + Send + 'static>(view: T) -> Self {
        Self {
            ptr: Box::new(view),
            handle: Box::new(ViewHandle::<T>::new()),
        }
    }

    #[inline]
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        self.handle.draw(self.ptr.as_mut(), renderer)
    }

    #[inline]
    fn resize(
        &mut self, 
        renderer: &mut dyn Renderer, 
        bounds: &Bounds<Canvas>
    ) -> Bounds<Canvas> {
        self.handle.resize(self.ptr.as_mut(), renderer, bounds)
    }

    #[inline]
    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) {
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

trait ViewHandleTrait : Send {
    fn draw(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer) -> Result<()>;
    fn resize(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, bounds: &Bounds<Canvas>) -> Bounds<Canvas>;
    fn event(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, event: &Event);
}

struct ViewHandle<T: Drawable> {
    marker: PhantomData<fn(T)>,
}

impl<T: Drawable> ViewHandle<T> {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<V: Drawable + 'static> ViewHandleTrait for ViewHandle<V> {
    #[inline]
    fn draw(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer) -> Result<()> {
        any.downcast_mut::<V>().unwrap().draw(renderer)
    }

    #[inline]
    fn resize(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) -> Bounds<Canvas> {
        any.downcast_mut::<V>().unwrap().resize(renderer, pos)
    }

    #[inline]
    fn event(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, event: &Event) {
        any.downcast_mut::<V>().unwrap().event(renderer, event)
    }
}

pub struct PosView {
    pos: Bounds<Canvas>,
}

impl PosView {
    pub fn new() -> Self {
        Self {
            pos: Bounds::none(),
        }
    }

    pub fn pos(&self) -> Bounds<Canvas> {
        self.pos.clone()
    }
}

impl Drawable for PosView {
    fn draw(&mut self, _renderer: &mut dyn Renderer) -> Result<()> {
        Ok(())
    }

    fn event(&mut self, _renderer: &mut dyn Renderer, event: &Event) {
        if let Event::Resize(pos) = event {
            self.pos = pos.clone();
        }
    }
}
