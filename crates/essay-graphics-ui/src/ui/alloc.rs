use essay_graphics_api::{renderer::{Canvas, Pos}, Bounds, Margin, Point, Size};

use crate::{context::CacheAlloc, page::Page};

#[derive(Debug)]
pub struct Alloc {
    pub update: AllocUpdate,

    pub bounds: Bounds<Canvas>, // extent of the canvas managed by the cursor
    pub margin: Margin,
    pub view_bounds: Bounds<Page>,     
    pub _fixed_bounds: Bounds<Canvas>,  // total size of fixed elements managed by the cursor

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
            None => { (Bounds::zero(), Bounds::zero()) }
        };

        Self {
            update,

            bounds,
            margin: Margin::ZERO,
            view_bounds: view_cache,
            _fixed_bounds: fixed_cache,

            view_width: view_width(bounds.width(), fixed_cache, view_cache),
            view_height: view_height(bounds.height(), fixed_cache, view_cache),

            alloc: Bounds::from(point),
            view_alloc: Bounds::zero(),
            fixed_alloc: Bounds::zero(),
        }
    }

    pub(super) fn child(
        &self, 
        parent_free: Pos,
        view: Option<Size>,
        margin: Margin,
        update: AllocUpdate,
        cache: Option<CacheAlloc>
    ) -> Self {
        let (view_cache, fixed_cache) = match cache {
            Some(cache) => { (cache.view, cache.fixed) },
            None => { (Bounds::zero(), Bounds::zero()) }
        };

        let view_bounds = match update {
            AllocUpdate::Vertical => {
                Bounds::from(Size(view_cache.width(), self.view_bounds.height()))
            }
            AllocUpdate::Horizontal => {
                Bounds::from(Size(self.view_bounds.width(), view_cache.height()))
            }
        };

        let view_bounds2 = match self.update {
            AllocUpdate::Vertical => {
                match update {
                    AllocUpdate::Vertical => {
                        let height = init_height(self.view_height, fixed_cache, view_cache);
                        ViewBounds {
                            width: parent_free.width(),
                            height: height,
                            view_width: parent_free.width(),
                            view_height: view_height(
                                height - margin.height(),
                                fixed_cache,
                                view_cache
                            )
                        }
                    },
                    AllocUpdate::Horizontal => {
                        ViewBounds {
                            width: parent_free.width(),
                            height: self.view_height,
                            view_width: view_width(
                                parent_free.width() - margin.width(),
                                fixed_cache, 
                                view_cache,
                            ),
                            view_height: self.view_height,
                        }
                    },
                }
            }
            AllocUpdate::Horizontal => {
                match update {
                    AllocUpdate::Vertical => {
                        ViewBounds {
                            width: self.view_width,
                            height: parent_free.height(),
                            view_width: self.view_width,
                            view_height: view_height(
                                parent_free.height() - margin.height(), 
                                fixed_cache,
                                view_cache
                            ),
                        }
                    },
                    AllocUpdate::Horizontal => {
                        let width = init_width(self.view_width, fixed_cache, view_cache);

                        ViewBounds {
                            width,
                            height: parent_free.height(),
                            view_width: view_width(
                                width - margin.width(),
                                fixed_cache,
                                view_cache
                            ),
                            view_height: parent_free.height(),
                        }
                    },
                }
            }
        };

        let mut bounds = Bounds::from((
            parent_free.p0(),
            Size(view_bounds2.width, view_bounds2.height),
        ));

        bounds = bounds - margin;

        let view_width = view_bounds2.view_width;
        let view_height = view_bounds2.view_height;

        /*
        println!("  ChildBounds {:?} -> {:?} {:?}", self.update, update, bounds);
        println!("    View {:?} Fixed {:?}", view_cache, fixed_cache);
        println!("    f_view {:?}, {:?}", view_width, view_height);
        */
        let alloc = Bounds::from(bounds.p0());

        let view_alloc = if let Some(size) = view {
            Bounds::from(size)
        } else {
            Bounds::zero()
        };

        Self {
            update,

            bounds,
            margin,
            view_bounds, // view_cache,
            _fixed_bounds: fixed_cache,

            view_width,
            view_height,

            alloc,
            view_alloc,
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
            view: self.view_alloc, // .union(Bounds::unit()),
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

                //let _page_rect = self.view_alloc_view(size);

                //cursor.view_pos = Point(page_rect.xmin(), page_rect.ymax());

                rect
            },
            AllocUpdate::Horizontal => {
                let rect = self.view_alloc_canvas(size);
        
                //cursor.pos = Point(rect.xmax(), rect.ymin());

                //let _page_rect = self.view_alloc_view(size);

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

    fn _view_alloc_view(&mut self, size: Size) -> Bounds<Page> {
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
    ) {
        if child.view_alloc.is_none() {
            self.alloc = self.alloc.union(child.alloc + child.margin);
            self.fixed_alloc = self.fixed_alloc.union(child.fixed_alloc); // + self.margin;
        } else {
            self.alloc = self.alloc.union(child.bounds + child.margin);
        }
            
        let view = self.view_alloc;
        let child = child.view_alloc;

        self.view_alloc = match self.update {
            AllocUpdate::Vertical => {
                Bounds::from((
                    [view.x0(), view.y0()],
                    [
                        view.width().max(child.width()).min(1.),
                        view.height() + child.height(), // .min(1.)
                    ]
                ))
            },
            AllocUpdate::Horizontal => {
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

fn view_width(width: f32, fixed: Bounds<Canvas>, view: Bounds<Page>) -> f32 {
    ((width - fixed.width()) / view.width().max(1.)).floor()
}

fn view_height(height: f32, fixed: Bounds<Canvas>, view: Bounds<Page>) -> f32 {
    ((height - fixed.height()) / view.height().max(1.)).floor()
}

fn init_width(factor: f32, fixed: Bounds<Canvas>, view: Bounds<Page>) -> f32 {
    if view.width() > 0. {
        factor * view.width()
    } else {
        fixed.width()
    }
}

fn init_height(factor: f32, fixed: Bounds<Canvas>, view: Bounds<Page>) -> f32 {
    if view.height() > 0. {
        factor * view.height()
    } else {
        fixed.height()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum AllocUpdate {
    Vertical,
    Horizontal,
}

impl AllocUpdate {
}

pub(crate) struct ViewBounds {
    width: f32,
    height: f32,
    view_width: f32,
    view_height: f32,
}

#[cfg(test)]
mod test {
    use essay_graphics_test::{TestGraphicsContext, TestRenderer};

    use crate::{context::Context, ui::{CentralPanel, Frame}};

    #[test]
    fn vertical_label() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                ui.label("Test");
            });
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'Test'");
    }

    #[test]
    fn vertical_label_three() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                ui.label("A");
                ui.label("B");
                ui.label("C");
            });
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
text (0.0,26.7) 'B'
text (0.0,53.3) 'C'");
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                ui.label("A");
                ui.label("B");
                ui.label("C");
            });
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
text (0.0,26.7) 'B'
text (0.0,53.3) 'C'");
    }

    #[test]
    fn vert_vert() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                ui.label("A");
                ui.vertical(|ui| {
                    ui.label("B");
                });
                ui.label("C");
            });
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
text (0.0,26.7) 'B'
text (0.0,53.3) 'C'");
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                ui.label("A");
                ui.vertical(|ui| {
                    ui.label("B");
                });
                ui.label("C");
            });
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

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                ui.view(|ui| {
                    ui.label("A");
                });
                ui.view(|ui| {
                    ui.label("B");
                });
            });
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
text (0.0,600.0) 'B'");
    }

    #[test]
    fn vertical_view_frame_text() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                Frame::group(ui).show(ui, |ui| {
                    ui.view(|ui| {
                        ui.label("A");
                    });
                })
            });
        }).unwrap();

        assert_eq!(test.take(), "rect (0.0,0.0) 1200.0x1200.0 #ffffffff
text (16.0,16.0) 'A'");
    }

    #[test]
    fn horizontal_view_frame() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    Frame::group(ui).show(ui, |ui| {
                        ui.view(|_| {});
                    })
                })
            });
        }).unwrap();

        assert_eq!(test.take(), "rect (0.0,0.0) 1200.0x1200.0 #ffffffff");
    }

    #[test]
    fn vertical_view_frame_inner_frame() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                Frame::group(ui).show(ui, |ui| {
                    ui.view(|ui| {
                        Frame::group(ui).background(0xff0000).show(ui, |ui| {
                            ui.view(|_| {});
                        });
                    });
                })
            });
        }).unwrap();

        assert_eq!(test.take(), "rect (0.0,0.0) 1200.0x1200.0 #ffffffff
rect (16.0,16.0) 1168.0x1168.0 #ff0000ff");
    }

    #[test]
    fn vertical_view_frame() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
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

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
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

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                Frame::group(ui).show(ui, |ui| {
                    ui.view(|ui| {
                        ui.label("A");
                    });
                });
                Frame::group(ui).show(ui, |ui| {
                    ui.horizontal(|ui| {
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

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        Frame::group(ui).show(ui, |ui| {
                            ui.view(|_| {});
                        });
                    });
                    ui.vertical(|ui| {
                        Frame::group(ui).show(ui, |ui| {
                            ui.view(|_| {});
                        });
                        Frame::group(ui).show(ui, |ui| {
                            ui.view(|_| {});
                        });
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

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        Frame::group(ui).show(ui, |ui| {
                            ui.view(|_| {});
                        });
                    });
                    ui.horizontal(|ui| {
                        Frame::group(ui).show(ui, |ui| {
                            ui.view(|_| {});
                        });
                        Frame::group(ui).show(ui, |ui| {
                            ui.view(|_| {});
                        });
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

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Test");
                });
            });
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'Test'");
    }

    #[test]
    fn horizontal_label_two() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ctx| {
            CentralPanel::new().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("A");
                    ui.label("B");
                });
            });
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'A'
text (26.7,0.0) 'B'");
    }
}