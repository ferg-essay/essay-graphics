use essay_graphics_api::{renderer::{Canvas}, Bounds, Point, Size};

use crate::{context::CacheAlloc, page::Page};

#[derive(Debug)]
pub struct Alloc {
    pub canvas_extent: Bounds<Canvas>, // extent of the canvas managed by the cursor
    pub view_extent: Bounds<Page>,     
    pub fixed_extent: Bounds<Canvas>,  // total size of fixed elements managed by the cursor

    pub pos: Point,
    pub view_pos: Point,

    pub canvas_allocated: Bounds<Canvas>, // current bounds allocated by the cursor
    pub view_allocated: Bounds<Page>,
    pub fixed_allocated: Bounds<Canvas>,
}

impl Alloc {
    pub(crate) fn new(
        canvas: Bounds<Canvas>,
        cache: Option<CacheAlloc>,
    ) -> Self {
        let point = Point(canvas.xmin(), canvas.ymin());

        let (view_cache, fixed_cache) = match cache {
            Some(cache) => { (cache.view, cache.fixed) },
            None => { (Bounds::unit(), Bounds::zero()) }
        };

        Self {
            canvas_extent: canvas,
            view_extent: view_cache,
            fixed_extent: fixed_cache,

            pos: point,
            view_pos: Point(0., 0.),

            canvas_allocated: Bounds::from(point),
            view_allocated: Bounds::zero(),
            fixed_allocated: Bounds::zero(),
        }
    }

    pub(super) fn child(
        &self, 
        pos: Point, 
        update: AllocUpdate,
        cache: Option<CacheAlloc>
    ) -> Self {
        let (view_cache, fixed_cache) = match cache {
            Some(cache) => { (cache.view, cache.fixed) },
            None => { (Bounds::unit(), Bounds::zero()) }
        };

        let view_extent = match update {
            AllocUpdate::Vertical => {
                Bounds::from(Size(view_cache.width(), self.view_extent.height()))
            }
            AllocUpdate::Horizontal => {
                Bounds::from(Size(self.view_extent.width(), view_cache.height()))
            }
        };        

        Self {
            canvas_extent: self.canvas_extent,
            view_extent, // view_cache,
            fixed_extent: fixed_cache,

            pos,
            view_pos: self.view_pos,

            canvas_allocated: Bounds::from(pos),
            view_allocated: Bounds::zero(),
            fixed_allocated: Bounds::zero(),
        }
    }

    pub(super) fn merge_child(
        &mut self, 
        child: &Self, 
        update: AllocUpdate,
        is_view: bool
    ) {
        self.canvas_allocated = self.canvas_allocated.union(child.canvas_allocated);

        if ! is_view {
            self.fixed_allocated = self.fixed_allocated.union(child.fixed_allocated);

            let view = self.view_allocated;
            let child = child.view_allocated;
            self.view_allocated = match update {
                AllocUpdate::Vertical => {
                    Bounds::from((
                        [view.x0(), view.y0()], 
                        [
                            view.width().max(child.width()),
                            view.height() + child.height().min(1.)
                        ]
                    ))
                },
                AllocUpdate::Horizontal => {
                    Bounds::from((
                        [view.x0(), view.y0()], 
                        [
                            view.width() + child.width().min(1.),
                            view.height().max(child.height())
                        ]
                    ))
                },
            }
        }
    }

    pub(crate) fn canvas_free(&self) -> Size {
        Size(
            self.canvas_extent.xmax() - self.pos.x(),
            self.canvas_extent.ymax() - self.pos.y()
        )
    }

    pub(crate) fn available_bounds(&self) -> Bounds<Canvas> {
        Bounds::new(
            Point(self.pos.x(), self.pos.y()),
            Point(self.canvas_extent.xmax(), self.canvas_extent.ymax()),
        )
    }

    pub(crate) fn to_cache(self) -> CacheAlloc {
        CacheAlloc {
            view: self.view_allocated.union(Bounds::unit()),
            fixed: self.fixed_allocated,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum AllocUpdate {
    Vertical,
    Horizontal,
}

impl AllocUpdate {
    pub fn alloc_canvas(&self, size: Size, cursor: &mut Alloc) -> Bounds<Canvas> {
        match self {
            AllocUpdate::Vertical => {
                let rect = Bounds::<Canvas>::new(
                    [cursor.pos.x(), cursor.pos.y()],
                    [
                        cursor.pos.x() + size.width(), 
                        cursor.pos.y() + size.height(), 
                    ],
                );
        
                cursor.pos = Point(
                    cursor.pos.x(),
                    cursor.pos.y() + size.height(),
                );

                cursor.canvas_allocated = cursor.canvas_allocated.union(&rect);
                cursor.fixed_allocated = cursor.fixed_allocated.union(&rect);

                rect
            },
            AllocUpdate::Horizontal => {
                let rect = Bounds::<Canvas>::new(
                    [cursor.pos.x(), cursor.pos.y()],
                    [
                        cursor.pos.x() + size.width(), 
                        cursor.pos.y() + size.height()
                    ],
                );
        
                cursor.pos = Point(
                    cursor.pos.x() + size.width(), 
                    cursor.pos.y(),
                );

                cursor.canvas_allocated = cursor.canvas_allocated.union(&rect);
                cursor.fixed_allocated = cursor.fixed_allocated.union(&rect);

                rect
            }
        }
    }

    pub fn alloc_view(&self, size: Size, cursor: &mut Alloc) -> Bounds<Canvas> {
        match self {
            AllocUpdate::Vertical => {
                let rect = self.alloc_view_canvas(size, cursor);
        
                cursor.pos = Point(rect.xmin(), rect.ymax());

                let page_rect = self.alloc_view_page(size, cursor);

                cursor.view_pos = Point(page_rect.xmin(), page_rect.ymax());

                rect
            },
            AllocUpdate::Horizontal => {
                let rect = self.alloc_view_canvas(size, cursor);
        
                cursor.pos = Point(rect.xmax(), rect.ymin());

                let page_rect = self.alloc_view_page(size, cursor);

                cursor.view_pos = Point(page_rect.xmax(), page_rect.ymin());

                rect
            }
        }
    }

    pub fn alloc_view_canvas(&self, size: Size, cursor: &mut Alloc) -> Bounds<Canvas> {
        let f_width = size.width() / cursor.view_extent.width();
        let f_height = size.height() / cursor.view_extent.height();

        let canvas_size = Size(
            f_width * (cursor.canvas_extent.width() - cursor.fixed_extent.width()),
            f_height * (cursor.canvas_extent.height() - cursor.fixed_extent.height()),
        );

        let alloc = Bounds::<Canvas>::new(
            [
                    cursor.pos.x(),
                    cursor.pos.y(),
                ],
            [
                    (cursor.pos.x() + canvas_size.width()).min(cursor.canvas_extent.xmax()), 
                    (cursor.pos.y() + canvas_size.height()).min(cursor.canvas_extent.ymax()), 
                ],
        );
        
        cursor.canvas_allocated = cursor.canvas_allocated.union(&alloc);

        alloc
    }

    pub fn alloc_view_page(&self, size: Size, cursor: &mut Alloc) -> Bounds<Page> {
        let page_rect = Bounds::<Page>::from((
            [cursor.view_pos.x(), cursor.view_pos.y()],
            [size.width(), size.height()],
        ));

        cursor.view_allocated = cursor.view_allocated.union(&page_rect);

        page_rect
    }
}
