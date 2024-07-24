use std::{any::Any, marker::PhantomData, sync::{Arc, Mutex}};

use essay_graphics_api::{
    renderer::{Canvas, Drawable, Event, Renderer}, Bounds, Coord, Point
};

#[derive(Clone)]
pub struct Layout {
    views: Vec<ViewItem>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            views: Vec::new(),
        }
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
    pub fn view<T: Drawable + Send + 'static>(
        &mut self, 
        pos: impl Into<Bounds<Layout>>,
        view: T
    ) -> View<T> {
        let mut pos = pos.into();

        // If unassigned, layout below all other views
        if pos.is_zero() || pos.is_none() {
            if self.views.len() == 0 {
                pos = Bounds::from([1., 1.])
            } else {
                let layout = self.bounds();
                pos = Bounds::new(
                    Point(0., layout.ymin() - 1.),
                    Point(1., layout.ymin()),
                );
            }
        }

        let id = self.views.len();

        self.views.push(ViewItem::new(pos, view));

        View {
            view_arc: self.views[id].ptr.clone(),
            marker: PhantomData,
        }
    }

    fn layout(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        let bounds = self.bounds();

        let p_x0 = pos.xmin().min(0.);
        let p_y0 = pos.ymin().min(0.);

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

            item.ptr.event(renderer, &Event::Resize(pos));
        }
    }

    fn bounds(&self) -> Bounds<Layout> {
        let mut bounds = Bounds::unit();

        for item in &self.views {
            bounds = bounds.union(&item.pos_grid);
        }

        bounds
    }
}

impl Drawable for Layout {
    fn draw(&mut self, renderer: &mut dyn Renderer) {
        for item in &mut self.views {
            renderer.draw_with(&item.pos_canvas, &mut item.ptr);
        }
    }

    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) {
        match event {
            Event::Resize(bounds) => {
                self.layout(renderer, bounds);

                for view in &mut self.views {
                    view.ptr.event(renderer, &Event::Resize(view.pos_canvas.clone()));
                }
            },
            _ => {
                // let point = event.point();

                for view in &mut self.views {
                    if event.in_bounds(&view.pos_canvas) {
                        view.ptr.event(renderer, event);
                    }
                }
            }
        }
    }
}

impl Coord for Layout {}

#[derive(Clone)]
struct ViewItem {
    pos_grid: Bounds<Layout>,
    pos_canvas: Bounds<Canvas>,

    ptr: ViewArc,
}

impl ViewItem {
    fn new<T: Drawable + Send + 'static>(pos: Bounds<Layout>, view: T) -> Self {
        Self {
            pos_grid: pos,
            pos_canvas: Bounds::none(),
            ptr: ViewArc(Arc::new(Mutex::new(ViewPtr::new(view))))
        }
    }
}

#[derive(Clone)]
struct ViewArc(Arc<Mutex<ViewPtr>>);

impl ViewArc {
    #[inline]
    fn read<T: 'static, R>(&self, fun: impl FnOnce(&T) -> R) -> R {
        self.0.lock().unwrap().read(fun)
    }

    #[inline]
    fn write<T: 'static, R>(&mut self, fun: impl FnOnce(&mut T) -> R) -> R {
        self.0.lock().unwrap().write(fun)
    }
}

impl Drawable for ViewArc {
    #[inline]
    fn draw(&mut self, renderer: &mut dyn Renderer) {
        let mut view = self.0.lock().unwrap();
        
        view.draw(renderer);
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
    fn draw(&mut self, renderer: &mut dyn Renderer) {
        self.handle.draw(self.ptr.as_mut(), renderer);
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

trait ViewHandleTrait {
    fn draw(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer);
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
    fn draw(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer) {
        any.downcast_mut::<V>().unwrap().draw(renderer)
    }

    #[inline]
    fn event(&mut self, any: &mut dyn Any, renderer: &mut dyn Renderer, event: &Event) {
        any.downcast_mut::<V>().unwrap().event(renderer, event)
    }
}

pub struct View<T> {
    view_arc: ViewArc,

    marker: PhantomData<fn(T)>,
}

impl<T: 'static> Clone for View<T> {
    fn clone(&self) -> Self {
        Self { 
            view_arc: self.view_arc.clone(), 
            marker: PhantomData,
        }
    }
}

impl<T: 'static> View<T> {
    #[inline]
    pub fn read<R>(&self, fun: impl FnOnce(&T) -> R) -> R {
        self.view_arc.read(fun)
    }

    #[inline]
    pub fn write<R>(&mut self, fun: impl FnOnce(&mut T) -> R) -> R {
        self.view_arc.write(fun)
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
    fn draw(&mut self, _renderer: &mut dyn Renderer) {
    }

    fn event(&mut self, _renderer: &mut dyn Renderer, event: &Event) {
        if let Event::Resize(pos) = event {
            self.pos = pos.clone();
        }
    }
}

#[cfg(test)]
mod test {
    use essay_graphics_api::{renderer::{Drawable, Event}, Bounds};
    use essay_graphics_test::TestRenderer;

    use crate::layout::PosView;

    use super::Layout;

    #[test]
    fn layout_basic() {
        let mut layout = Layout::new();

        let bounds = Bounds::from([100., 200.]);
        let mut renderer = TestRenderer::new(&bounds);

        layout.event(&mut renderer, &Event::Resize(bounds));

        assert_eq!(renderer.drain(), Vec::<String>::new().as_slice());
    }

    #[test]
    fn layout_single_pos() {
        let mut layout = Layout::new();

        let bounds = Bounds::from([100., 200.]);
        let mut renderer = TestRenderer::new(&bounds);

        let view = layout.view((), PosView::new());

        layout.event(&mut renderer, &Event::Resize(bounds));

        assert_eq!(renderer.drain(), Vec::<String>::new().as_slice());

        assert_eq!(view.read(|v| v.pos()), Bounds::from(((0., 0.), [100., 200.])));
    }

    #[test]
    fn layout_dual_pos() {
        let mut layout = Layout::new();

        let bounds = Bounds::from([360., 3600.]);
        let mut renderer = TestRenderer::new(&bounds);

        let view1 = layout.view((), PosView::new());
        let view2 = layout.view((), PosView::new());

        layout.event(&mut renderer, &Event::Resize(bounds));

        assert_eq!(renderer.drain(), Vec::<String>::new().as_slice());

        assert_eq!(view1.read(|v| v.pos()), Bounds::from(((0., 1800.), [360., 1800.])));
        assert_eq!(view2.read(|v| v.pos()), Bounds::from(((0., 0.), [360., 1800.])));
    }

    #[test]
    fn layout_pos_group() {
        let mut layout = Layout::new();

        let bounds = Bounds::from([360., 3600.]);
        let mut renderer = TestRenderer::new(&bounds);

        let v1 = layout.view(((0., 0.), [2., 2.]), PosView::new());
        let v2 = layout.view(((2., 0.), [1., 1.]), PosView::new());
        let v3 = layout.view(((2., 1.), [1., 1.]), PosView::new());
        let v4 = layout.view(((0., 2.), [1., 1.]), PosView::new());
        let v5 = layout.view(((1., 2.), [1., 1.]), PosView::new());
        let v6 = layout.view(((2., 2.), [1., 1.]), PosView::new());

        layout.event(&mut renderer, &Event::Resize(bounds));

        assert_eq!(renderer.drain(), Vec::<String>::new().as_slice());

        assert_eq!(v1.read(|v| v.pos()), Bounds::from(((0., 0.), [240., 2400.])));
        assert_eq!(v2.read(|v| v.pos()), Bounds::from(((240., 0.), [120., 1200.])));
        assert_eq!(v3.read(|v| v.pos()), Bounds::from(((240., 1200.), [120., 1200.])));
        assert_eq!(v4.read(|v| v.pos()), Bounds::from(((0., 2400.), [120., 1200.])));
        assert_eq!(v5.read(|v| v.pos()), Bounds::from(((120., 2400.), [120., 1200.])));
        assert_eq!(v6.read(|v| v.pos()), Bounds::from(((240., 2400.), [120., 1200.])));
    }

    #[test]
    fn layout_small_pos_ll() {
        let mut layout = Layout::new();

        let bounds = Bounds::from([360., 3600.]);
        let mut renderer = TestRenderer::new(&bounds);

        let view = layout.view(((0., 0.), [0.25, 0.5]), PosView::new());

        layout.event(&mut renderer, &Event::Resize(bounds));

        assert_eq!(renderer.drain(), Vec::<String>::new().as_slice());

        assert_eq!(view.read(|v| v.pos()), Bounds::from(((0., 0.), [90., 1800.])));
    }

    #[test]
    fn layout_small_pos_ur() {
        let mut layout = Layout::new();

        let bounds = Bounds::from([360., 3600.]);
        let mut renderer = TestRenderer::new(&bounds);

        let view = layout.view(((0.75, 0.5), [0.25, 0.5]), PosView::new());

        layout.event(&mut renderer, &Event::Resize(bounds));

        assert_eq!(renderer.drain(), Vec::<String>::new().as_slice());

        assert_eq!(view.read(|v| v.pos()), Bounds::from(((270., 1800.), [90., 1800.])));
    }
}