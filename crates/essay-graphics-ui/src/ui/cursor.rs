use essay_graphics_api::{renderer::Canvas, Bounds, Point, Size};

use crate::page::Page;

pub struct Cursor {
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

    pub(super) fn child_view(
        &self, 
        canvas_extent: Bounds<Canvas>,
        page_extent: Bounds<Page>,
        fixed_extent: Bounds<Canvas>,
    ) -> Self {
        let canvas_pos = Point(canvas_extent.xmin(), canvas_extent.ymax());

        Self {
            canvas_extent,
            page_extent,
            fixed_extent,

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

#[derive(Clone, Copy)]
pub enum CursorUpdate {
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
                let rect = self.alloc_view_canvas(size, cursor);
        
                cursor.canvas_pos = Point(cursor.canvas_pos.x(), cursor.canvas_pos.y() - rect.height());

                let page_rect = self.alloc_view_page(size, cursor);

                cursor.page_pos = Point(page_rect.xmin(), page_rect.ymin());

                rect
            },
            CursorUpdate::Horizontal => {
                let rect = self.alloc_view_canvas(size, cursor);
        
                cursor.canvas_pos = Point(rect.xmax(), rect.ymax());

                let page_rect = self.alloc_view_page(size, cursor);

                cursor.page_pos = Point(page_rect.xmax(), page_rect.ymax());

                rect
            }
        }
    }

    pub fn alloc_view_canvas(&self, size: Size, cursor: &mut Cursor) -> Bounds<Canvas> {
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
        
        cursor.canvas_allocated = cursor.canvas_allocated.union(&rect);

        rect
    }

    pub fn alloc_view_page(&self, size: Size, cursor: &mut Cursor) -> Bounds<Page> {
        let page_rect = Bounds::<Page>::from((
            [cursor.page_pos.x(), cursor.page_pos.y() - size.height()],
            [size.width(), size.height()],
        ));

        cursor.page_allocated = cursor.page_allocated.union(&page_rect);

        page_rect
    }
}

#[derive(Clone)]
pub(super) struct ViewSizeCache {
    pub page: Bounds<Page>,
    pub canvas: Bounds<Canvas>,
    pub children: Vec<Option<ViewSizeCache>>,
}

impl ViewSizeCache {
    pub(super) fn new() -> Self {
        Self {
            page: Bounds::unit(),
            canvas: Bounds::from(Point(0., 0.)),
            children: Vec::default(),
        }
    }
    
    pub(super) fn get(&self, index: usize) -> Option<&ViewSizeCache> {
        match self.children.get(index) {
            Some(view) => view.as_ref(),
            None => None
        }
    }
    
    pub(super) fn push(&mut self, index: usize) -> &mut ViewSizeCache {
        assert!(self.children.len() <= index);

        self.children.resize(index + 1, None);

        self.children[index] = Some(ViewSizeCache::new());

        self.children[index].as_mut().unwrap()
    }
    
    pub(crate) fn merge(&mut self, prev_cache: &ViewSizeCache) -> bool {
        if self.page != prev_cache.page {
            return false;
        }

        if self.children.len() != prev_cache.children.len() {
            return false;
        }

        for (next, prev) in self.children.iter_mut().zip(prev_cache.children.iter()) {
            if next.is_none() != prev.is_none() {
                return false;
            }

            if let Some(next) = next {
                if let Some(prev) = prev {
                    if ! next.merge(prev) {
                        return false;
                    }
                }
            }
        }

        true
    }
}


impl Default for ViewSizeCache {
    fn default() -> Self {
        Self {
            page: Bounds::unit(),
            canvas: Bounds::from(Point(0., 0.)),
            children: Vec::default(),
        }
    }
}
