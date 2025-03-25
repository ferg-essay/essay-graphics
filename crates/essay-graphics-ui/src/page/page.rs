use essay_graphics_api::{
    renderer::{Result, Canvas, Drawable, Event, Renderer}, 
    Bounds, Coord, Point
};

use super::{view::ViewArc, View};

#[derive(Clone)]
pub struct Page {
    views: Vec<ViewItem>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            views: Vec::new(),
        }
    }

    ///
    /// Adds a drawable view in page coordinates, returning a view handle
    /// to the drawable.
    /// 
    /// Page coordinates are (0, 0) upper left and (1, 1) low right,
    /// but normalized to the minimum and maximum of all added views.
    /// ((1., 1.), (2., 2.)) is allowed, as are negative values.
    /// 
    /// If the position is unassigned, the new position will be a unit
    /// box below any current box, such as ((0., -1), (0., 0.))
    /// 
    pub fn view<T: Drawable + Send + 'static>(
        &mut self, 
        pos: impl Into<Bounds<Page>>,
        view: impl Into<View<T>>,
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

        let view = view.into();

        // let id = self.views.len();

        self.views.push(ViewItem::new(pos, &view));

        view
    }

    pub fn _subview<T: Drawable + Send + 'static>(
        &mut self, 
        id: ViewId,
        index: usize,
        drawable: T
    ) -> View<T> {
        assert!(index > 0);

        let view_item = &mut self.views[id.0];

        view_item._insert(index, drawable)
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

            item.pos_canvas = item.ptrs[0].resize(renderer, &pos);

            for ptr in item.ptrs.iter_mut().skip(1) {
                ptr.resize(renderer, &item.pos_canvas);
            }

            // TODO: remove?
            for ptr in &mut item.ptrs {
                ptr.event(renderer, &Event::Resize(item.pos_canvas.clone()));
            }
        }
    }

    fn bounds(&self) -> Bounds<Page> {
        let mut bounds = Bounds::unit();

        for item in &self.views {
            bounds = bounds.union(&item.pos_grid);
        }

        bounds
    }
}

impl Drawable for Page {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        for item in &mut self.views {
            for view in &mut item.ptrs {
                renderer.draw_with(&item.pos_canvas, view)?;
            }
        }

        Ok(())
    }

    fn resize(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) -> Bounds<Canvas> {
        self.layout(renderer, pos);

        pos.clone()
    }

    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) {
        for view in &mut self.views {
            if event.in_bounds(&view.pos_canvas) {
                for ptr in &mut view.ptrs {
                    ptr.event(renderer, event);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ViewId(usize);
#[derive(Clone)]
struct ViewItem {
    pos_grid: Bounds<Page>,
    pos_canvas: Bounds<Canvas>,

    ptrs: Vec<ViewArc>,
}

impl ViewItem {
    fn new<T: Drawable + Send + 'static>(pos: Bounds<Page>, view: &View<T>) -> Self {
        let mut ptrs = Vec::new();

        ptrs.push(view.arc().clone());

        Self {
            pos_grid: pos,
            pos_canvas: Bounds::none(),
            ptrs,
        }
    }

    fn _insert<T>(&mut self, index: usize, drawable: T) -> View<T> 
    where
        T: Drawable + Send + 'static
    {
        // let view = View::new(drawable);
    
        /*
        for i in (0..self.ptrs.len()).rev() {
            if self.ptrs[i].index() <= index {
                self.ptrs.insert(i + 1, view.arc().clone());
            }
        }

        view
        */

        todo!()
    }
}

impl Coord for Page {}
#[cfg(test)]
mod test {
    use essay_graphics_api::{renderer::{Drawable, Event}, Bounds};
    use essay_graphics_test::TestRenderer;

    use crate::page::PosView;

    use super::Page;

    #[test]
    fn layout_basic() {
        let mut layout = Page::new();

        let bounds = Bounds::from([100., 200.]);
        let mut renderer = TestRenderer::new(&bounds);

        layout.event(&mut renderer, &Event::Resize(bounds));

        assert_eq!(renderer.drain(), Vec::<String>::new().as_slice());
    }

    #[test]
    fn layout_single_pos() {
        let mut layout = Page::new();

        let bounds = Bounds::from([100., 200.]);
        let mut renderer = TestRenderer::new(&bounds);

        let view = layout.view((), PosView::new());

        layout.event(&mut renderer, &Event::Resize(bounds));

        assert_eq!(renderer.drain(), Vec::<String>::new().as_slice());

        assert_eq!(view.read(|v| v.pos()), Bounds::from(((0., 0.), [100., 200.])));
    }

    #[test]
    fn layout_dual_pos() {
        let mut layout = Page::new();

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
        let mut layout = Page::new();

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
        let mut layout = Page::new();

        let bounds = Bounds::from([360., 3600.]);
        let mut renderer = TestRenderer::new(&bounds);

        let view = layout.view(((0., 0.), [0.25, 0.5]), PosView::new());

        layout.event(&mut renderer, &Event::Resize(bounds));

        assert_eq!(renderer.drain(), Vec::<String>::new().as_slice());

        assert_eq!(view.read(|v| v.pos()), Bounds::from(((0., 0.), [90., 1800.])));
    }

    #[test]
    fn layout_small_pos_ur() {
        let mut layout = Page::new();

        let bounds = Bounds::from([360., 3600.]);
        let mut renderer = TestRenderer::new(&bounds);

        let view = layout.view(((0.75, 0.5), [0.25, 0.5]), PosView::new());

        layout.event(&mut renderer, &Event::Resize(bounds));

        assert_eq!(renderer.drain(), Vec::<String>::new().as_slice());

        assert_eq!(view.read(|v| v.pos()), Bounds::from(((270., 1800.), [90., 1800.])));
    }
}