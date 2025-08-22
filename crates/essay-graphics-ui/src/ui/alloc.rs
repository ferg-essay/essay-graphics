use essay_graphics_api::{renderer::{Canvas, Pos}, Bounds, Point, Size};

use crate::{context::CacheAlloc, page::Page};

#[derive(Debug)]
pub struct Alloc {
    pub update: AllocUpdate,

    pub bounds: Bounds<Canvas>, // extent of the canvas managed by the cursor
    pub view_bounds: Bounds<Page>,     
    pub fixed_bounds: Bounds<Canvas>,  // total size of fixed elements managed by the cursor

    view_width: f32,
    view_height: f32,

    //pos: Point,
    //view_pos: Point,

    pub alloc: Bounds<Canvas>, // current bounds allocated by the cursor
    pub view_alloc: Bounds<Page>,
    pub fixed_alloc: Bounds<Canvas>,
}

impl Alloc {
    pub(crate) fn new(
        bounds: Bounds<Canvas>,
        update: AllocUpdate,
        cache: Option<CacheAlloc>,
    ) -> Self {
        let point = Point(bounds.xmin(), bounds.ymin());

        let (view_cache, fixed_cache) = match cache {
            Some(cache) => { (cache.view, cache.fixed) },
            None => { (Bounds::unit(), Bounds::zero()) }
        };

        Self {
            update,

            bounds,
            view_bounds: view_cache,
            fixed_bounds: fixed_cache,

            view_width: ((bounds.width() - fixed_cache.width()) / view_cache.width()).floor(),
            view_height: ((bounds.height() - fixed_cache.height()) / view_cache.height()).floor(),

            //pos: point,
            //view_pos: Point(0., 0.),

            alloc: Bounds::from(point),
            view_alloc: Bounds::zero(),
            fixed_alloc: Bounds::zero(),
        }
    }

    pub(super) fn child(
        &self, 
        bounds: Pos,
        update: AllocUpdate,
        cache: Option<CacheAlloc>
    ) -> Self {
        let (view_cache, fixed_cache) = match cache {
            Some(cache) => { (cache.view, cache.fixed) },
            None => { (Bounds::unit(), Bounds::zero()) }
        };

        let view_bounds = match update {
            AllocUpdate::Vertical => {
                Bounds::from(Size(view_cache.width(), self.view_bounds.height()))
            }
            AllocUpdate::Horizontal => {
                Bounds::from(Size(self.view_bounds.width(), view_cache.height()))
            }
        };

        let mut view_width = self.view_width;
        let mut view_height = self.view_height;

        if self.update != update {
            match update {
                AllocUpdate::Vertical => {
                    view_height = ((bounds.height() - fixed_cache.height()) / view_cache.height()).floor();
                }
                AllocUpdate::Horizontal => {
                    view_width = ((bounds.width() - fixed_cache.width()) / view_cache.width()).floor();
                }
            }
        };

        Self {
            update,

            bounds,
            view_bounds, // view_cache,
            fixed_bounds: fixed_cache,

            //view_width: (bounds.width() / view_cache.width()).floor(),
            //view_height: (bounds.height() / view_cache.height()).floor(),
            view_width,
            view_height,

            //pos,
            //view_pos: self.view_pos,

            alloc: Bounds::from(bounds.p0()),
            view_alloc: Bounds::zero(),
            fixed_alloc: Bounds::zero(),
        }
    }

    pub(crate) fn canvas_free(&self) -> Size {
        match self.update {
            AllocUpdate::Vertical => {
                Size(
                    self.bounds.width(),
                    self.bounds.ymax() - self.alloc.ymax(),
                )
            },
            AllocUpdate::Horizontal => {
                Size(
                    self.bounds.xmax() - self.alloc.xmax(),
                    self.bounds.height(),
                )
            }
        }
    }

    pub(crate) fn available_bounds(&self) -> Bounds<Canvas> {
        match self.update {
            AllocUpdate::Vertical => {
                Bounds::from([
                    [self.bounds.xmin(), self.alloc.ymax()],
                    [self.bounds.xmax(), self.bounds.ymax()],
                ])
            },
            AllocUpdate::Horizontal => {
                Bounds::from([
                    [self.alloc.xmax(), self.bounds.ymin()],
                    [self.bounds.xmax(), self.bounds.ymax()],
                ])
            }
        }
    }

    pub(crate) fn to_cache(self) -> CacheAlloc {
        CacheAlloc {
            view: self.view_alloc.union(Bounds::unit()),
            fixed: self.fixed_alloc,
        }
    }

    pub(super) fn alloc_canvas(&mut self, size: Size) -> Bounds<Canvas> {
        match self.update {
            AllocUpdate::Vertical => {
                let rect = Bounds::<Canvas>::from((
                    Point(self.bounds.xmin(), self.alloc.ymax()),
                    size,
                ));

                self.alloc = self.alloc.union(&rect);
                self.fixed_alloc = self.fixed_alloc.union(&rect);

                rect
            },
            AllocUpdate::Horizontal => {
                let rect = Bounds::<Canvas>::from((
                    Point(self.alloc.xmax(), self.bounds.ymin()),
                    size,
                ));

                self.alloc = self.alloc.union(&rect);
                self.fixed_alloc = self.fixed_alloc.union(&rect);

                rect
            }
        }
    }

    pub fn alloc_view(&mut self, size: Size) -> Bounds<Canvas> {
        match self.update {
            AllocUpdate::Vertical => {
                let rect = self.view_alloc_canvas(size);
        
                //cursor.pos = Point(rect.xmin(), rect.ymax());

                let page_rect = self.view_alloc_view(size);

                //cursor.view_pos = Point(page_rect.xmin(), page_rect.ymax());

                rect
            },
            AllocUpdate::Horizontal => {
                let rect = self.view_alloc_canvas(size);
        
                //cursor.pos = Point(rect.xmax(), rect.ymin());

                let page_rect = self.view_alloc_view(size);

                //cursor.view_pos = Point(page_rect.xmax(), page_rect.ymin());

                rect
            }
        }
    }

    fn view_alloc_canvas(&mut self, size: Size) -> Bounds<Canvas> {
        let canvas_size = Size(
            size.width() * self.view_width,
            size.height() * self.view_height,
        );

        let alloc = match self.update {
            AllocUpdate::Vertical => {
                Bounds::from([
                    [self.bounds.xmin(), self.alloc.ymax()],
                    [
                        (self.bounds.xmin() + canvas_size.width()).min(self.bounds.xmax()),
                        (self.alloc.ymax() + canvas_size.height()).min(self.bounds.ymax()),
                    ]
                ])
            }
            AllocUpdate::Horizontal => {
                Bounds::from([
                    [self.alloc.xmax(), self.bounds.ymin()],
                    [
                        (self.alloc.xmax() + canvas_size.width()).min(self.bounds.xmax()),
                        (self.bounds.ymin() + canvas_size.height()).min(self.bounds.ymax()),
                    ]
                ])
            }
        };
        
        //self.canvas_allocated = self.canvas_allocated.union(&alloc);

        alloc
    }

    fn view_alloc_view(&mut self, size: Size) -> Bounds<Page> {
        let alloc = match self.update {
            AllocUpdate::Vertical => {
                Bounds::from((
                    Point(self.view_bounds.xmin(), self.view_alloc.ymax()),
                    size,
                ))
            }
            AllocUpdate::Horizontal => {
                Bounds::from((
                    Point(self.view_alloc.xmax(), self.view_bounds.ymin()),
                    size,
                ))
            }
        };

        self.view_alloc = self.view_alloc.union(&alloc);

        alloc
    }

    pub(super) fn merge_child(
        &mut self, 
        child: &Self, 
        is_view: bool
    ) {
        if is_view {
            self.alloc = self.alloc.union(child.bounds);
        } else if ! is_view {
            self.alloc = self.alloc.union(child.alloc);
            self.fixed_alloc = self.fixed_alloc.union(child.fixed_alloc);

            let view = self.view_alloc;
            let child = child.view_alloc;
            self.view_alloc = match self.update {
                AllocUpdate::Vertical => {
                    Bounds::from((
                        [view.x0(), view.y0()], 
                        [
                            view.width().max(child.width()),
                            view.height() + child.height(), // .min(1.)
                        ]
                    ))
                },
                AllocUpdate::Horizontal => {
                    Bounds::from((
                        [view.x0(), view.y0()], 
                        [
                            view.width() + child.width(), // .min(1.),
                            view.height().max(child.height())
                        ]
                    ))
                },
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum AllocUpdate {
    Vertical,
    Horizontal,
}

impl AllocUpdate {
}
