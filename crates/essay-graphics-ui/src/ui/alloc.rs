use essay_graphics_api::{renderer::{Canvas, Pos}, Bounds, Margin, Point, Size};

use crate::{page::Page};

#[derive(Debug)]
pub struct Alloc {
    pub alloc_dir: AllocDirection,

    pub bounds: Bounds<Canvas>, // extent of the canvas managed by the cursor
    pub margin: Margin,

    view_width: f32,
    view_height: f32,

    pub alloc: Bounds<Canvas>, // current bounds allocated by the cursor
    pub alloc_cache: AllocCache,
}

impl Alloc {
    pub(crate) fn new(
        bounds: Bounds<Canvas>,
        update: AllocDirection,
        cache: Option<AllocCache>,
    ) -> Self {
        let point = Point::new(bounds.xmin(), bounds.ymin());

        let bounds_cache = cache.unwrap_or_else(AllocCache::default);

        Self {
            alloc_dir: update,

            bounds,
            margin: Margin::ZERO,

            view_width: bounds_cache.view_width(bounds.width()),
            view_height: bounds_cache.view_height(bounds.height()),

            alloc: Bounds::from(point),
            alloc_cache: AllocCache::default(),
        }
    }

    pub(super) fn child(
        &self, 
        parent_free: Pos,
        view: Option<Size>,
        margin: Margin,
        alloc_dir: AllocDirection,
        cache: Option<AllocCache>
    ) -> Self {
        let bounds_cache = cache.unwrap_or_else(AllocCache::default);

        let view_bounds = match self.alloc_dir {
            AllocDirection::Vertical => {
                match alloc_dir {
                    AllocDirection::Vertical => {
                        let height = bounds_cache.init_height(self.view_height);
                        ViewBounds {
                            width: parent_free.width(),
                            height: height,
                            view_width: parent_free.width(),
                            view_height: bounds_cache.view_height(height - margin.height()),
                        }
                    },
                    AllocDirection::Horizontal => {
                        ViewBounds {
                            width: parent_free.width(),
                            height: self.view_height,
                            view_width: bounds_cache.view_width(
                                parent_free.width() - margin.width(),
                            ),
                            view_height: self.view_height,
                        }
                    },
                }
            }
            AllocDirection::Horizontal => {
                match alloc_dir {
                    AllocDirection::Vertical => {
                        ViewBounds {
                            width: self.view_width,
                            height: parent_free.height(),
                            view_width: self.view_width,
                            view_height: bounds_cache.view_height(
                                parent_free.height() - margin.height(),
                            ),
                        }
                    },
                    AllocDirection::Horizontal => {
                        let width = bounds_cache.init_width(self.view_width);

                        ViewBounds {
                            width,
                            height: parent_free.height(),
                            view_width: bounds_cache.view_width(width - margin.width()),
                            view_height: parent_free.height(),
                        }
                    },
                }
            }
        };

        let mut bounds = Bounds::from((
            parent_free.p0(),
            Size::new(view_bounds.width, view_bounds.height),
        ));

        bounds = bounds - margin;

        let view_width = view_bounds.view_width;
        let view_height = view_bounds.view_height;

        let alloc = Bounds::from(bounds.p0());

        let alloc_cache = if let Some(size) = view {
            AllocCache {
                view: Bounds::from(size),
                fixed: Bounds::zero(),
            }
        } else {
            AllocCache::default()
        };

        Self {
            alloc_dir,

            bounds,
            margin,

            view_width,
            view_height,

            alloc,
            alloc_cache,
        }
    }

    pub(crate) fn canvas_free(&self) -> Size {
        match self.alloc_dir {
            AllocDirection::Vertical => {
                Size::new(
                    self.bounds.width(),
                    self.bounds.ymax() - self.alloc.ymax(),
                )
            },
            AllocDirection::Horizontal => {
                Size::new(
                    self.bounds.xmax() - self.alloc.xmax(),
                    self.bounds.height(),
                )
            }
        }
    }

    pub(crate) fn available_bounds(&self) -> Bounds<Canvas> {
        match self.alloc_dir {
            AllocDirection::Vertical => {
                Bounds::from([
                    [self.bounds.xmin(), self.alloc.ymax()],
                    [self.bounds.xmax(), self.bounds.ymax()],
                ])
            },
            AllocDirection::Horizontal => {
                Bounds::from([
                    [self.alloc.xmax(), self.bounds.ymin()],
                    [self.bounds.xmax(), self.bounds.ymax()],
                ])
            }
        }
    }

    pub(crate) fn to_cache(self) -> AllocCache {
        self.alloc_cache.clone()
    }

    pub(super) fn alloc_canvas(&mut self, size: impl Into<Size>) -> Bounds<Canvas> {
        let size = size.into();

        match self.alloc_dir {
            AllocDirection::Vertical => {
                let rect = Bounds::<Canvas>::from((
                    Point::new(self.bounds.xmin(), self.alloc.ymax()),
                    size,
                ));

                self.alloc = self.alloc.union(&rect);
                self.alloc_cache.fixed = self.alloc_cache.fixed.union(&rect);

                rect
            },
            AllocDirection::Horizontal => {
                let rect = Bounds::<Canvas>::from((
                    Point::new(self.alloc.xmax(), self.bounds.ymin()),
                    size,
                ));

                self.alloc = self.alloc.union(&rect);
                self.alloc_cache.fixed = self.alloc_cache.fixed.union(&rect);

                rect
            }
        }
    }

    pub fn alloc_view(&mut self, size: impl Into<Size>) -> Bounds<Canvas> {
        match self.alloc_dir {
            AllocDirection::Vertical => {
                let rect = self.view_alloc_canvas(size);
        
                //cursor.pos = Point(rect.xmin(), rect.ymax());

                //let _page_rect = self.view_alloc_view(size);

                //cursor.view_pos = Point(page_rect.xmin(), page_rect.ymax());

                rect
            },
            AllocDirection::Horizontal => {
                let rect = self.view_alloc_canvas(size);
        
                //cursor.pos = Point(rect.xmax(), rect.ymin());

                //let _page_rect = self.view_alloc_view(size);

                //cursor.view_pos = Point(page_rect.xmax(), page_rect.ymin());

                rect
            }
        }
    }

    fn view_alloc_canvas(&mut self, size: impl Into<Size>) -> Bounds<Canvas> {
        let size = size.into();

        let canvas_size = Size::new(
            size.width * self.view_width,
            size.height * self.view_height,
        );

        let alloc = match self.alloc_dir {
            AllocDirection::Vertical => {
                Bounds::from([
                    [self.bounds.xmin(), self.alloc.ymax()],
                    [
                        (self.bounds.xmin() + canvas_size.width).min(self.bounds.xmax()),
                        (self.alloc.ymax() + canvas_size.height).min(self.bounds.ymax()),
                    ]
                ])
            }
            AllocDirection::Horizontal => {
                Bounds::from([
                    [self.alloc.xmax(), self.bounds.ymin()],
                    [
                        (self.alloc.xmax() + canvas_size.width).min(self.bounds.xmax()),
                        (self.bounds.ymin() + canvas_size.height).min(self.bounds.ymax()),
                    ]
                ])
            }
        };
        
        alloc
    }

    pub(super) fn merge_child(
        &mut self, 
        child: &Self, 
    ) {
        if child.alloc_cache.view.is_none() {
            self.alloc = self.alloc.union(child.alloc + child.margin);
            self.alloc_cache.fixed = self.alloc_cache.fixed.union(child.alloc_cache.fixed); // + self.margin;
        } else {
            self.alloc = self.alloc.union(child.bounds + child.margin);
        }
            
        let view = self.alloc_cache.view;
        let child = child.alloc_cache.view;

        self.alloc_cache.view = match self.alloc_dir {
            AllocDirection::Vertical => {
                Bounds::from((
                    [view.x0(), view.y0()],
                    [
                        view.width().max(child.width()).min(1.),
                        view.height() + child.height(), // .min(1.)
                    ]
                ))
            },
            AllocDirection::Horizontal => {
                Bounds::from((
                    [view.x0(), view.y0()], 
                    [
                        view.width() + child.width(), // .min(1.),
                        view.height().max(child.height()).min(1.)
                    ]
                ))
            },
        };
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum AllocDirection {
    Vertical,
    Horizontal,
}

impl AllocDirection {
}

pub(crate) struct ViewBounds {
    width: f32,
    height: f32,
    view_width: f32,
    view_height: f32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AllocCache {
    pub fixed: Bounds<Canvas>,
    pub view: Bounds<Page>,
}

impl AllocCache {
    pub(crate) fn is_changed(&self, _alloc_cache: &Option<AllocCache>) -> bool {
        false
    }

    fn view_width(&self, width: f32) -> f32 {
        ((width - self.fixed.width()) / self.view.width().max(1.)).floor()
    }

    fn view_height(&self, height: f32) -> f32 {
        ((height - self.fixed.height()) / self.view.height().max(1.)).floor()
    }

    fn init_width(&self, factor: f32) -> f32 {
        if self.view.width() > 0. {
            factor * self.view.width()
        } else {
            self.fixed.width()
        }
    }

    fn init_height(&self, factor: f32) -> f32 {
        if self.view.height() > 0. {
            factor * self.view.height()
        } else {
            self.fixed.height()
        }
    }
}

#[cfg(test)]
mod test {
    use essay_graphics_test::{TestGraphicsContext, TestRenderer};

    use crate::{context::Context, ui::{Frame}};

    #[test]
    fn vertical_label() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.label("Test");
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'Test'");
    }

    #[test]
    fn vertical_label_three() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.label("A");
            ui.label("B");
            ui.label("C");
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
text (0.0,26.7) 'B'
text (0.0,53.3) 'C'");
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.label("A");
            ui.label("B");
            ui.label("C");
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
text (0.0,26.7) 'B'
text (0.0,53.3) 'C'");
    }

    #[test]
    fn vert_vert() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.label("A");
            ui.column(|ui| {
                ui.label("B");
            });
            ui.label("C");
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
text (0.0,26.7) 'B'
text (0.0,53.3) 'C'");
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.label("A");
            ui.column(|ui| {
                ui.label("B");
            });
            ui.label("C");
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
text (0.0,26.7) 'B'
text (0.0,53.3) 'C'");
    }

    //
    // Two views stacked vertically that contain labels
    //
    #[test]
    fn vertical_view_label() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.view(|ui| {
                ui.label("A");
            });
            ui.view(|ui| {
                ui.label("B");
            });
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
text (0.0,600.0) 'B'");
    }

    #[test]
    fn vertical_view_frame_text() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            Frame::group(ui).show(ui, |ui| {
                ui.view(|ui| {
                    ui.label("A");
                });
            });
        }).unwrap();

        assert_eq!(test.take(), "rect (0.0,0.0) 1200.0x1200.0 #ffffffff
text (16.0,16.0) 'A'");
    }

    #[test]
    fn horizontal_view_frame() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.row(|ui| {
                Frame::group(ui).show(ui, |ui| {
                    ui.view(|_| {});
                })
            });
        }).unwrap();

        assert_eq!(test.take(), "rect (0.0,0.0) 1200.0x1200.0 #ffffffff");
    }

    #[test]
    fn vertical_view_frame_inner_frame() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            Frame::group(ui).show(ui, |ui| {
                ui.view(|ui| {
                    Frame::group(ui).background(0xff0000).show(ui, |ui| {
                        ui.view(|_| {});
                    });
                });
            });
        }).unwrap();

        assert_eq!(test.take(), "rect (0.0,0.0) 1200.0x1200.0 #ffffffff
rect (16.0,16.0) 1168.0x1168.0 #ff0000ff");
    }

    #[test]
    fn vertical_view_frame() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.view(|ui| {
                ui.label("A");
            });
            Frame::group(ui).show(ui, |ui| {
                ui.view(|ui| {
                    ui.label("B");
                });
            });
            ui.view(|ui| {
                ui.label("C");
            });
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
rect (0.0,400.0) 1200.0x400.0 #ffffffff
text (16.0,416.0) 'B'
text (0.0,800.0) 'C'");
    }

    #[test]
    fn vertical_view_frames_sub_view_vertical() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            Frame::group(ui).show(ui, |ui| {
                ui.view(|ui| {
                    ui.label("A");
                });
            });
            Frame::group(ui).show(ui, |ui| {
                ui.view(|ui| {
                    ui.label("B");
                });
                ui.view(|ui| {
                    ui.label("C");
                });
            });
            Frame::group(ui).show(ui, |ui| {
                ui.view(|ui| {
                    ui.label("D");
                });
            });
        }).unwrap();

        assert_eq!(test.take(), "rect (0.0,0.0) 1200.0x300.0 #ffffffff
text (16.0,16.0) 'A'
rect (0.0,300.0) 1200.0x600.0 #ffffffff
text (16.0,316.0) 'B'
text (16.0,600.0) 'C'
rect (0.0,900.0) 1200.0x300.0 #ffffffff
text (16.0,916.0) 'D'");
    }

    #[test]
    fn vertical_view_frames_sub_view_horizontal() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            Frame::group(ui).show(ui, |ui| {
                ui.view(|ui| {
                    ui.label("A");
                });
            });
            Frame::group(ui).show(ui, |ui| {
                ui.row(|ui| {
                    ui.view(|ui| {
                        ui.label("B");
                    });
                    ui.view(|ui| {
                        ui.label("C");
                    });
                });
            });
            Frame::group(ui).show(ui, |ui| {
                ui.view(|ui| {
                    ui.label("D");
                });
            });
        }).unwrap();

        assert_eq!(test.take(), "rect (0.0,0.0) 1200.0x400.0 #ffffffff
text (16.0,16.0) 'A'
rect (0.0,400.0) 1200.0x400.0 #ffffffff
text (16.0,416.0) 'B'
text (600.0,416.0) 'C'
rect (0.0,800.0) 1200.0x400.0 #ffffffff
text (16.0,816.0) 'D'");
    }

    #[test]
    fn horiz_vert1_vert2() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.row(|ui| {
                ui.column(|ui| {
                    Frame::group(ui).show(ui, |ui| {
                        ui.view(|_| {});
                    });
                });
                ui.column(|ui| {
                    Frame::group(ui).show(ui, |ui| {
                        ui.view(|_| {});
                    });
                    Frame::group(ui).show(ui, |ui| {
                        ui.view(|_| {});
                    });
                });
            });
        }).unwrap();

        assert_eq!(test.take(), "rect (0.0,0.0) 600.0x1200.0 #ffffffff
rect (600.0,0.0) 600.0x600.0 #ffffffff
rect (600.0,600.0) 600.0x600.0 #ffffffff");
    }

    #[test]
    fn vert_horiz1_horiz2() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.column(|ui| {
                ui.row(|ui| {
                    Frame::group(ui).show(ui, |ui| {
                        ui.view(|_| {});
                    });
                });
                ui.row(|ui| {
                    Frame::group(ui).show(ui, |ui| {
                        ui.view(|_| {});
                    });
                    Frame::group(ui).show(ui, |ui| {
                        ui.view(|_| {});
                    });
                });
            });
        }).unwrap();

        assert_eq!(test.take(), "rect (0.0,0.0) 1200.0x600.0 #ffffffff
rect (0.0,600.0) 600.0x600.0 #ffffffff
rect (600.0,600.0) 600.0x600.0 #ffffffff");
    }

    #[test]
    fn horizontal_label() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.row(|ui| {
                ui.label("Test");
            });
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'Test'");
    }

    #[test]
    fn horizontal_label_two() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.row(|ui| {
                ui.label("A");
                ui.label("B");
            });
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
text (26.7,0.0) 'B'");
    }
}