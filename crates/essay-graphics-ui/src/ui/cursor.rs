use essay_graphics_api::{renderer::Canvas, Bounds, Point, Size};

use crate::page::Page;

pub(super) struct Cursor {
    pub canvas_extent: Bounds<Canvas>, // extent of the canvas managed by the cursor
    pub page_extent: Bounds<Page>,
    pub fixed_extent: Bounds<Canvas>, // total size of fixed elements managed by the cursor

    pub canvas_pos: Point,
    pub page_pos: Point,

    pub canvas_allocated: Bounds<Canvas>, // current bounds allocated by the cursor
    pub page_allocated: Bounds<Page>,
    pub fixed_allocated: Bounds<Canvas>,
}

impl Cursor {
    pub(crate) fn new(
        canvas: Bounds<Canvas>,
        page: Bounds<Page>,
    ) -> Self {
        let point = Point(canvas.xmin(), canvas.ymax());

        Self {
            canvas_extent: canvas,
            page_extent: page,
            fixed_extent: Bounds::zero(),

            canvas_pos: point,
            page_pos: Point(0., 0.),

            canvas_allocated: Bounds::from(point),
            page_allocated: Bounds::zero(),
            fixed_allocated: Bounds::zero(),
        }
    }

    pub(super) fn child(&self, canvas_pos: Point) -> Self {
        Self {
            canvas_extent: self.canvas_extent,
            page_extent: self.page_extent,
            fixed_extent: self.fixed_extent,

            canvas_pos,
            page_pos: self.page_pos,

            canvas_allocated: Bounds::from(canvas_pos),
            page_allocated: Bounds::from(self.page_pos),
            fixed_allocated: Bounds::zero(),
        }
    }

    pub(super) fn merge_child(&mut self, child: &Self) {
        self.canvas_allocated = self.canvas_allocated.union(child.canvas_allocated);
        self.page_allocated = self.page_allocated.union(child.page_allocated);
        self.fixed_allocated = self.fixed_allocated.union(child.fixed_allocated);
    }

    pub(crate) fn canvas_free(&self) -> Size {
        Size(
            self.canvas_extent.xmax() - self.canvas_pos.x(),
            self.canvas_pos.y() - self.canvas_extent.ymin()
        )
    }
}

pub(super) struct CursorTop {
    pub last_id: ViewSizeId,
    pub prev_state: ViewSizeCache,
    pub next_state: ViewSizeCache,
    pos: Bounds<Canvas>,
}

impl CursorTop {
    pub fn new(
        last_id: ViewSizeId, 
        state: ViewSizeCache,
        pos: Bounds<Canvas>,
    ) -> Self {
        Self {
            last_id,
            prev_state: state,
            next_state: ViewSizeCache::new(last_id),
            pos
        }
    }

    pub fn merge_state(self) -> ViewSizeCache {
        // todo() need to merge because of options like tabs
        self.next_state
    }
}

pub(super) enum CursorUpdate {
    Vertical,
    Horizontal,
}

impl CursorUpdate {
    pub fn alloc_canvas(&self, size: Size, cursor: &mut Cursor) -> Bounds<Canvas> {
        match self {
            CursorUpdate::Vertical => {
                let rect = Bounds::<Canvas>::new(
                    [cursor.canvas_pos.x(), cursor.canvas_pos.y() - size.height()],
                    [cursor.canvas_pos.x() + size.width(), cursor.canvas_pos.y()],
                );
        
                cursor.canvas_pos = Point(cursor.canvas_pos.x(), cursor.canvas_pos.y() - size.height());
                cursor.canvas_allocated = cursor.canvas_allocated.union(&rect);
                cursor.fixed_allocated = cursor.fixed_allocated.union(&rect);

                rect
            },
            CursorUpdate::Horizontal => {
                let rect = Bounds::<Canvas>::new(
                    [cursor.canvas_pos.x(), cursor.canvas_pos.y() - size.height()],
                    [cursor.canvas_pos.x() + size.width(), cursor.canvas_pos.y()],
                );
        
                cursor.canvas_pos = Point(cursor.canvas_pos.x() + size.width(), cursor.canvas_pos.y());
                cursor.canvas_allocated = cursor.canvas_allocated.union(&rect);
                cursor.fixed_allocated = cursor.fixed_allocated.union(&rect);

                rect
            }
        }
    }

    pub fn alloc_page(&self, size: Size, cursor: &mut Cursor) -> Bounds<Canvas> {
        match self {
            CursorUpdate::Vertical => {
                let f_width = size.width() / cursor.page_extent.width();
                let f_height = size.height() / cursor.page_extent.height();

                let canvas_size = Size(
                    f_width * (cursor.canvas_extent.width() - cursor.fixed_extent.width()),
                    f_height * (cursor.canvas_extent.height() - cursor.fixed_extent.height()),
                );

                let rect = Bounds::<Canvas>::new(
                    [
                            cursor.canvas_pos.x(),
                            (cursor.canvas_pos.y() - canvas_size.height()).max(0.)
                        ],
                    [
                            (cursor.canvas_pos.x() + canvas_size.width()).min(cursor.canvas_extent.xmax()), 
                            cursor.canvas_pos.y()
                        ],
                );
        
                cursor.canvas_pos = Point(cursor.canvas_pos.x(), cursor.canvas_pos.y() - rect.height());
                cursor.canvas_allocated = cursor.canvas_allocated.union(&rect);

                let page_rect = Bounds::<Page>::from((
                    [cursor.page_pos.x(), cursor.page_pos.y() - size.height()],
                    [size.width(), size.height()],
                ));

                cursor.page_pos = Point(cursor.page_pos.x(), cursor.page_pos.y() - size.height());
                cursor.page_allocated = cursor.page_allocated.union(&page_rect);

                rect
            },
            CursorUpdate::Horizontal => {
                if true { todo!(); };

                let rect = Bounds::<Canvas>::new(
                    [cursor.canvas_pos.x(), cursor.canvas_pos.y() - size.height()],
                    [cursor.canvas_pos.x() + size.width(), cursor.canvas_pos.y()],
                );
        
                cursor.canvas_pos = Point(cursor.canvas_pos.x() + size.width(), cursor.canvas_pos.y());
                cursor.canvas_allocated = cursor.canvas_allocated.union(&rect);

                rect
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) struct ViewSizeId(usize);

impl ViewSizeId {
    #[must_use]
    fn next(self) -> ViewSizeId {
        Self(self.0 + 1)
    }
}

impl Default for ViewSizeId {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Clone)]
pub(super) struct ViewSizeCache {
    pub id: ViewSizeId,
    pub page: Bounds<Page>,
    pub canvas: Bounds<Canvas>,
    pub children: Vec<Option<ViewSizeCache>>,
}

impl ViewSizeCache {
    pub(super) fn new(id: ViewSizeId) -> Self {
        Self {
            id,
            page: Bounds::unit(),
            canvas: Bounds::from(Point(0., 0.)),
            children: Vec::default(),
        }
    }

    fn add_canvas(&mut self, pos: Bounds<Canvas>) {
        self.canvas = self.canvas.union(pos);
    }

    fn add_page(&mut self, pos: Bounds<Page>) {
        self.page = self.page.union(pos);
    }

    fn child(&self, index: usize) -> Option<&ViewSizeCache> {
        self.children.get(index)
            .map(|v| v.as_ref())
            .unwrap_or(None)
    }

    fn add_child(&mut self, index: usize, id: ViewSizeId, pos: Point) -> &ViewSizeCache {
        if self.children.len() <= index {
            self.children.resize(index, None);
        }

        self.children[index] = Some(ViewSizeCache::new(id));

        self.children[index].as_ref().unwrap()
    }
}