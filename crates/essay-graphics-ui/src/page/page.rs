use essay_graphics_api::{
    renderer::{Drawable, Renderer, Result}, 
    Bounds, Coord, Size
};

use super::{view::ViewArc, View};

#[derive(Clone)]
pub struct Page {
    views: Vec<ViewItem>,
}

impl Page {
    pub fn new<T>(view: impl Into<View<T>>) -> Page 
    where
        T: Drawable + Send + 'static
    {
        let mut builder = Self::builder();
        builder.view(view);
        builder.build()
    }

    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl Drawable for Page {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        for item in &mut self.views {
            item.draw(renderer)?;
        }

        Ok(())
    }

    /*
    fn resize(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) -> Bounds<Canvas> {
        self.layout(renderer, pos);

        pos.clone()
    }
    */

    /*
    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) {
        for view in &mut self.views {
            if event.in_bounds(&view.pos_canvas) {
                 ptr.event(renderer, event);
            }
        }
    }
    */
}

pub struct Builder {
    size: Size,
    view: Option<ViewArc>,
    children: Vec<Builder>,
    update: CursorUpdate,
}

impl Builder {
    fn new() -> Self {
        Self {
            size: Size(1., 1.),
            view: None,
            children: Vec::new(),
            update: CursorUpdate::Vertical,
        }
    }

    pub fn size(&mut self, size: impl Into<Size>) -> &mut Self {
        if let Some(child) = self.children.last_mut() {
            child.size = size.into();
        }

        self
    }

    pub fn width(&mut self, width: f32) -> &mut Self {
        if let Some(child) = self.children.last_mut() {
            child.size = Size(width, child.size.height());
        }

        self
    }

    pub fn height(&mut self, height: f32) -> &mut Self {
        if let Some(child) = self.children.last_mut() {
            child.size = Size(child.size.width(), height);
        }

        self
    }

    pub fn view<T>(&mut self, view: impl Into<View<T>>) -> View<T>
    where
        T: Drawable + Send + 'static
    {
        self.view_size(Size(1., 1.), view)
    }

    pub fn view_size<T>(
        &mut self, 
        size: impl Into<Size>,
        view: impl Into<View<T>>
    ) -> View<T>
    where
        T: Drawable + Send + 'static
    {
        let view = view.into();

        self.children.push(Self {
            size: size.into(),
            view: Some(view.arc().clone()),
            children: Vec::new(),
            update: CursorUpdate::Single,
        });

        view
    }

    pub fn horizontal(&mut self, builder: impl FnOnce(&mut Builder)) -> &mut Self {
        self.horizontal_height(1., builder)
    }

    pub fn horizontal_height(
        &mut self, 
        height: f32, 
        builder: impl FnOnce(&mut Builder)
    ) -> &mut Self {
        self.children.push(Self {
            size: Size(1., height),
            view: None,
            children: Vec::new(),
            update: CursorUpdate::Horizontal,
        });

        (builder)(self.children.last_mut().unwrap());

        self
    }

    pub fn vertical(&mut self, builder: impl FnOnce(&mut Builder)) {
        self.vertical_width(1., builder)
    }

    pub fn vertical_width(
        &mut self, 
        width: f32, 
        builder: impl FnOnce(&mut Builder)
    ) {
        self.children.push(Self {
            size: Size(width, 1.),
            view: None,
            children: Vec::new(),
            update: CursorUpdate::Vertical,
        });

        (builder)(self.children.last_mut().unwrap());
    }

    pub fn build(self) -> Page {
        let mut views = Vec::<ViewItem>::new();

        let pos = Bounds::from([1., 1.]);

        let update = self.update.clone();

        update.build(&mut views, pos, self);

        Page {
            views,
        }
    }
}

#[derive(Clone)]
enum CursorUpdate {
    Single,
    Vertical,
    Horizontal,
}

impl CursorUpdate {
    fn build(
        &self, 
        vec: &mut Vec<ViewItem>, 
        pos: Bounds<Page>,
        mut build: Builder,
    ) {
        match self {
            CursorUpdate::Single => {
                let view_arc = build.view.take().unwrap();

                vec.push(ViewItem::new(pos, view_arc));
            }
            CursorUpdate::Vertical => {
                let mut height = 0.;

                for item in &build.children {
                    height += item.size.height();
                }

                let factor = pos.height() / height.max(1e-6);

                let mut ymax = pos.ymax();

                for child in build.children.drain(..) {
                    let height = factor * child.size.height();
                    let ymin = ymax - height;

                    let pos = Bounds::from((
                        pos.xmin(), ymin,
                        pos.xmax(), ymax,
                    ));

                    let update = child.update.clone();
                    update.build(vec, pos, child);

                    ymax = ymin;
                }
            },
            CursorUpdate::Horizontal => {
                let mut width = 0.;

                for item in &build.children {
                    width += item.size.width();
                }

                let factor = pos.width() / width.max(1e-6);

                let mut x = pos.xmin();

                for child in build.children.drain(..) {
                    let xmax = x + factor * child.size.width();

                    let pos = Bounds::from((
                        x, pos.ymin(),
                        xmax, pos.ymax(),
                    ));

                    let update = child.update.clone();

                    update.build(vec, pos, child);

                    x = xmax;
                }
            },
        }
    }
}

#[derive(Clone)]
struct ViewItem {
    pos: Bounds<Page>,

    view: ViewArc,
}

impl ViewItem {
    fn new(pos: Bounds<Page>, view: ViewArc) -> Self {
        Self {
            pos,
            view,
        }
    }

    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        let pos = renderer.pos().clone();

        let pos = (
            pos.xmin() + self.pos.xmin() * pos.width(),
            pos.ymin() + self.pos.ymin() * pos.height(),
            pos.xmin() + self.pos.xmax() * pos.width(),
            pos.ymin() + self.pos.ymax() * pos.height(),
        ).into();

        renderer.draw_with(&pos, &mut self.view)

    }
}

impl Coord for Page {}
#[cfg(test)]
mod test {
    /*
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
    */
}