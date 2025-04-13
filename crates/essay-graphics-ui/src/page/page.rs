use essay_graphics_api::{
    renderer::{Canvas, Drawable, Renderer, Result}, 
    Bounds, Coord, Size
};

use crate::ui::{Tabs, Ui, UiSize, UiView};

pub struct Page {
    views: Vec<ViewItem>,
}

impl Page {
    pub fn new(view: impl Drawable + Send + 'static) -> Page 
    {
        Self::build(|ui| {
            ui.view(view);
        })
    }

    pub fn build(f: impl FnOnce(&mut PageBuilder)) -> Page {
        let mut builder = PageBuilder::new();
        (f)(&mut builder);

        builder.build()
    }

    pub fn view_bounds(&self, id: ViewId) -> Bounds<Page> {
        self.views[id.0].pos.clone()
    }

    pub fn render<R>(
        &mut self, 
        id: ViewId, 
        ui: &mut dyn Renderer,
        f: impl FnOnce(&mut dyn Renderer) -> Result<R>
    ) -> Result<R> {
        let pos = self.views[id.0].pos(ui);

        let mut result: Option<R> = None;

        ui.draw_with(pos, Box::new(|ui| {
            result = Some((f)(ui)?);
            Ok(())
        }))?;

        Ok(result.unwrap())
    }
}

impl Drawable for Page {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        for item in &mut self.views {
            item.draw(renderer)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ViewId(usize);

pub struct PageBuilder {
    size: Size,
    view: Option<Box<dyn Drawable + Send>>,
    children: Vec<PageBuilder>,
    update: CursorUpdate,

    id: usize,
}

impl PageBuilder {
    pub fn new() -> Self {
        Self {
            size: Size(1., 1.),
            view: None,
            children: Vec::new(),
            update: CursorUpdate::Vertical,
            id: 0,
        }
    }

    pub fn view(&mut self, view: impl Drawable + Send + 'static) -> ViewId {
        self.view_size(Size(1., 1.), view)
    }

    pub fn view_size(
        &mut self, 
        size: impl Into<Size>,
        view: impl Drawable + Send + 'static, // <T>>
    ) -> ViewId
    //where
    //    T: Drawable + Send + 'static
    {
        let id = ViewId(self.id);
        self.id += 1;

        self.children.push(Self {
            size: size.into(),
            view: Some(Box::new(view)),
            children: Vec::new(),
            update: CursorUpdate::Single,
            id: id.0,
        });

        id
    }

    pub fn horizontal<R>(&mut self, f: impl FnOnce(&mut PageBuilder) -> R) -> R {
        self.horizontal_size(1., f)
    }

    pub fn horizontal_size<R>(&mut self, size: f32, f: impl FnOnce(&mut PageBuilder) -> R) -> R {
        let mut sub = Self {
            size: Size(size, size),
            view: None,
            children: Vec::new(),
            update: CursorUpdate::Horizontal,
            id: self.id,
        };

        let result = (f)(&mut sub);

        self.id = sub.id;

        self.children.push(sub);

        result
    }

    pub fn vertical<R>(&mut self, f: impl FnOnce(&mut PageBuilder) -> R) -> R {
        self.vertical_size(1., f)
    }

    pub fn vertical_size<R>(
        &mut self, 
        size: f32, 
        f: impl FnOnce(&mut PageBuilder) -> R
    ) -> R {
        let mut sub = Self {
            size: Size(size, size),
            view: None,
            children: Vec::new(),
            update: CursorUpdate::Vertical,
            id: self.id,
        };

        let result = (f)(&mut sub);

        self.id = sub.id;

        self.children.push(sub);

        result
    }

    pub fn build(self) -> Page {
        let mut views = Vec::<ViewItem>::new();

        let pos = Bounds::from([1., 1.]);

        let update = self.update.clone();
        let mut own = self;
        update.build(&mut views, pos, &mut own);

        Page {
            views,
        }
    }
}

pub struct Page2 {
    ui_view: UiView,
}

impl Page2 {
    pub fn new(view: impl Drawable + Send + 'static) -> Self {
        Self::build(|ui| {
            ui.view(view);
        })
    }

    pub fn build(f: impl FnOnce(&mut PageBuilder2)) -> Self {
        let mut builder = PageBuilder2::new();

        (f)(&mut builder);

        let mut items = builder.children;

        Self {
            ui_view: UiView::new(move |ui| {
                for item in &mut items {
                    item.draw(ui);
                }
            }),
        }
    }
}

impl Drawable for Page2 {
    fn draw(&mut self, ui: &mut dyn Renderer) -> Result<()> {
        self.ui_view.draw(ui)
    }
}

pub struct PageBuilder2 {
    // update: CursorUpdate,
    children: Vec<Box<dyn PageDraw>>,
}

impl PageBuilder2 {
    pub fn new() -> Self {
        Self {
            // update: CursorUpdate::Vertical,
            children: Vec::new(),
        }
    }

    pub fn view(&mut self, view: impl Drawable + Send + 'static) {
        self.view_size(Size(1., 1.), view)
    }

    pub fn view_size(
        &mut self, 
        size: impl Into<Size>,
        view: impl Drawable + Send + 'static, // <T>>
    ) {
        self.children.push(Box::new(PageDrawable {
            size: size.into(),
            draw: Box::new(view),
        }));
    }

    pub fn horizontal<R>(&mut self, f: impl FnOnce(&mut PageBuilder2) -> R) -> R {
        self.horizontal_size(1., f)
    }

    pub fn horizontal_size<R>(&mut self, size: f32, f: impl FnOnce(&mut PageBuilder2) -> R) -> R {
        let mut sub = Self {
            children: Vec::new(),
        };

        let result = (f)(&mut sub);

        self.children.push(Box::new(PageHoriz {
            size: UiSize::Page(size, size),
            children: sub.children,
        }));

        result
    }

    pub fn vertical<R>(
        &mut self, 
        add_content: impl FnOnce(&mut PageBuilder2) -> R
    ) -> R {
        self.vertical_size(1., add_content)
    }

    pub fn vertical_size<R>(
        &mut self, 
        size: f32, 
        add_content: impl FnOnce(&mut PageBuilder2) -> R
    ) -> R {
        let mut sub = Self {
            children: Vec::new(),
        };

        let result = (add_content)(&mut sub);

        self.children.push(Box::new(PageVert {
            size: UiSize::Page(size, size),
            children: sub.children,
        }));


        result
    }

    pub fn tabs(
        &mut self, 
        add_content: impl FnOnce(&mut BuildTabs)
    ) {
        let mut tabs = BuildTabs {
            tabs: Vec::new(),
        };

        let result = (add_content)(&mut tabs);

        self.children.push(Box::new(PageTabs::new(tabs.tabs)));


        result
    }
}

pub struct BuildTabs {
    tabs: Vec<(String, Box<dyn PageDraw>)>,
}

impl BuildTabs {
    pub fn tab<R>(&mut self, label: String, add_content: impl FnOnce(&mut PageBuilder2) -> R) -> R {
        let mut content = PageBuilder2::new();

        let result = (add_content)(&mut content);

        self.tabs.push((label, Box::new(PageVert { 
            size: UiSize::Page(1., 1.),
            children: content.children 
        })));

        result
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
        build: &mut PageBuilder,
    ) {
        match self {
            CursorUpdate::Single => {
                if let Some(view) = build.view.take() {
                    vec.push(ViewItem::new(pos, view));
                }
            }
            CursorUpdate::Vertical => {
                let mut height = 0.;

                for item in &build.children {
                    height += item.size.height();
                }

                let factor = pos.height() / height.max(1e-6);

                let mut ymax = pos.ymax();

                for child in &mut build.children {
                    let height = factor * child.size.height();
                    let ymin = ymax - height;

                    let pos = Bounds::from([
                        [pos.xmin(), ymin],
                        [pos.xmax(), ymax],
                    ]);

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

                for child in &mut build.children {
                    let xmax = x + factor * child.size.width();

                    let pos = Bounds::from([
                        [x, pos.ymin()],
                        [xmax, pos.ymax()],
                    ]);

                    let update = child.update.clone();

                    update.build(vec, pos, child);

                    x = xmax;
                }
            },
        }
    }
}

//#[derive(Clone)]
struct ViewItem {
    pos: Bounds<Page>,

    view: Box<dyn Drawable + Send>,
}

impl ViewItem {
    fn new(pos: Bounds<Page>, view: Box<dyn Drawable + Send>) -> Self {
        Self {
            pos,
            view,
        }
    }

    fn pos(&self, ui: &mut dyn Renderer) -> Bounds::<Canvas> {
        let pos = ui.pos().clone();

        [
            [pos.xmin() + self.pos.xmin() * pos.width(),
            pos.ymin() + self.pos.ymin() * pos.height()],
            [pos.xmin() + self.pos.xmax() * pos.width(),
            pos.ymin() + self.pos.ymax() * pos.height()],
        ].into()
    }

    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        let pos = self.pos(renderer);

        renderer.draw_with(pos, Box::new(|ui| 
            self.view.draw(ui)
        ))
    }
}

trait PageDraw : Send + 'static {
    fn draw(&mut self, ui: &mut Ui);
}

struct PageDrawable {
    size: Size,
    draw: Box<dyn Drawable + Send>,
}

impl PageDraw for PageDrawable {
    fn draw(&mut self, ui: &mut Ui) {
        ui.draw_size(self.size, &mut self.draw);
    }
}

struct _PageUi {
    size: UiSize,
    add_content: Box<dyn FnMut(&mut Ui) + Send>,
}

impl PageDraw for _PageUi {
    fn draw(&mut self, ui: &mut Ui) {
        ui.vertical_view(self.size, |ui| {
            (self.add_content)(ui)
        });
    }
}

struct PageHoriz {
    size: UiSize,
    children: Vec<Box<dyn PageDraw>>,
}

impl PageDraw for PageHoriz {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal_view(self.size, |ui| {
            for child in &mut self.children {
                child.draw(ui);
            }
        });
    }
}

struct PageVert {
    size: UiSize,
    children: Vec<Box<dyn PageDraw>>,
}

impl PageDraw for PageVert {
    fn draw(&mut self, ui: &mut Ui) {
        ui.vertical_view(self.size, |ui| {
            for child in &mut self.children {
                child.draw(ui);
            }
        });
    }
}

struct PageTabs {
    value: String,
    children: Vec<(String, Box<dyn PageDraw>)>,
}

impl PageTabs {
    fn new(children: Vec<(String, Box<dyn PageDraw>)>) -> Self {
        assert!(children.len() > 0, "tabs must have at least one item");

        Self {
            value: children[0].0.clone(),
            children,
        }
    }
}

impl PageDraw for PageTabs {
    fn draw(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            let mut tabs = Tabs::<String>::new(self.value.clone());

            for (label, draw) in &mut self.children {
                tabs.item(label.clone(), |ui| {
                    draw.draw(ui);
                })
            }

            if let Some(value) = tabs.show(ui) {
                self.value = value;
            };
        });
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