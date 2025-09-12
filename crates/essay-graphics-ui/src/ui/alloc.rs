use essay_graphics_api::{renderer::{Canvas, Pos}, Bounds, Length, Padding, Point, Rectangle, Size};

#[derive(Debug)]
pub struct Alloc {
    pub alloc_dir: AllocDirection,

    pub bounds: Bounds<Canvas>, // extent of the canvas managed by the cursor
    pub margin: Padding,

    view_width: f32,
    view_height: f32,

    pub alloc: Bounds<Canvas>, // current bounds allocated by the cursor
    pub alloc_size: AllocSize,
}

impl Alloc {
    pub(super) fn new(
        bounds: Bounds<Canvas>,
        update: AllocDirection,
        cache: Option<AllocPair>,
    ) -> Self {
        let point = Point::new(bounds.xmin(), bounds.ymin());

        let bounds_cache = cache.unwrap_or_else(AllocPair::default);

        let _outer_bounds = bounds_cache.outer;
        let inner_bounds = bounds_cache.inner;

        Self {
            alloc_dir: update,

            bounds: bounds.into(),
            margin: Padding::ZERO,

            view_width: inner_bounds.view_width(bounds.width()),
            view_height: inner_bounds.view_height(bounds.height()),

            alloc: point.into(),
            alloc_size: AllocSize::default(),
        }
    }

    pub(super) fn child(
        &self, 
        parent_free: Pos,
        margin: Padding,
        alloc_dir: AllocDirection,
        cache: Option<AllocPair>
    ) -> Self {
        let bounds_cache = cache.unwrap_or_else(AllocPair::default);

        let outer_bounds = bounds_cache.outer;
        let inner_bounds = bounds_cache.inner;

        let view_bounds = match self.alloc_dir {
            AllocDirection::Column => {
                match alloc_dir {
                    AllocDirection::Column => {
                        let height = outer_bounds.init_height(self.view_height);

                        ViewBounds {
                            width: parent_free.width(),
                            height: height,
                            view_width: parent_free.width(),
                            view_height: inner_bounds.view_height(height - margin.height()),
                        }
                    },
                    AllocDirection::Row => {
                        ViewBounds {
                            width: parent_free.width(),
                            height: self.view_height,
                            view_width: inner_bounds.view_width(
                                parent_free.width() - margin.width(),
                            ),
                            view_height: self.view_height,
                        }
                    },
                }
            }
            AllocDirection::Row => {
                match alloc_dir {
                    AllocDirection::Column => {
                        ViewBounds {
                            width: self.view_width,
                            height: parent_free.height(),
                            view_width: self.view_width,
                            view_height: inner_bounds.view_height(
                                parent_free.height() - margin.height(),
                            ),
                        }
                    },
                    AllocDirection::Row => {
                        let width = outer_bounds.init_width(self.view_width);

                        ViewBounds {
                            width,
                            height: parent_free.height(),
                            view_width: inner_bounds.view_width(width - margin.width()),
                            view_height: parent_free.height(),
                        }
                    },
                }
            }
        };

        let mut bounds = Bounds::new0(
            parent_free.x0(),
            parent_free.y0(),
            view_bounds.width, 
            view_bounds.height,
        );

        bounds = bounds - margin;

        let view_width = view_bounds.view_width;
        let view_height = view_bounds.view_height;

        let alloc = Bounds::from(bounds.p0());

        let alloc_size = AllocSize::default();

        Self {
            alloc_dir,

            bounds,
            margin,

            view_width,
            view_height,

            alloc,
            alloc_size,
        }
    }

    // returns the boundary box for available layout
    pub(super) fn available(&self) -> Bounds<Canvas> {
        match self.alloc_dir {
            AllocDirection::Column => {
                Bounds::from([
                    [self.bounds.x0(), self.alloc.y1()],
                    [self.bounds.x1(), self.bounds.y1()],
                ])
            },
            AllocDirection::Row => {
                Bounds::from([
                    [self.alloc.x1(), self.bounds.y0()],
                    [self.bounds.x1(), self.bounds.y1()],
                ])
            }
        }
    }

    pub(super) fn alloc(&mut self, size: impl Into<Size<Length>>) -> Rectangle {
        let size = size.into();

        let (width, f_width, v_width) = self.width(size.width);
        let (height, f_height, v_height) = self.height(size.height);

        match self.alloc_dir {
            AllocDirection::Column => {
                let alloc = Rectangle::new(
                    self.bounds.x0(), 
                    self.alloc.y1(),
                    width,
                    height,
                );

                self.alloc = self.alloc.union(alloc);

                self.alloc_size.fixed.width = self.alloc_size.fixed.width.max(f_width);
                self.alloc_size.fixed.height = self.alloc_size.fixed.height + f_height;

                self.alloc_size.view.width = self.alloc_size.view.width.max(v_width);
                self.alloc_size.view.height = self.alloc_size.view.height + v_height;

                alloc
            },
            AllocDirection::Row => {
                let alloc = Rectangle::new(
                    self.alloc.x1(), 
                    self.bounds.y0(),
                    width,
                    height,
                );

                self.alloc = self.alloc.union(alloc);

                self.alloc_size.fixed.width = self.alloc_size.fixed.width + f_width;
                self.alloc_size.fixed.height = self.alloc_size.fixed.height.max(f_height);

                self.alloc_size.view.width = self.alloc_size.view.width + v_width;
                self.alloc_size.view.height = self.alloc_size.view.height.max(v_height);

                alloc
            }
        }
    }

    // return (canvas, fixed, view)
    fn width(&mut self, width: Length) -> (f32, f32, f32) {
        match width {
            Length::Shrink => { (0., 0., 0.) },
            Length::Pixels(px) => { (px, px, 0.) },
            Length::Fill => { 
                (self.view_width, 0., 1.)
            },
            Length::View(width) => {
                (width * self.view_width, 0., width)
             },
        }
    }

    // return (canvas, fixed, view)
    fn height(&mut self, height: Length) -> (f32, f32, f32) {
        match height {
            Length::Shrink => { (0., 0., 0.) },
            Length::Pixels(px) => { (px, px, 0.) },
            Length::Fill => { 
                (self.view_height, 0., 1.)
            },
            Length::View(height) => {
                (height * self.view_height, 0., height)
             },
        }
    }

    pub(super) fn merge_child(
        &mut self, 
        child: &mut Self, 
        size: Option<Size<Length>>,
    ) -> AllocPair {
        self.alloc = self.alloc.union(child.bounds + child.margin);
        
        let view = self.alloc_size.view;
        let fixed = self.alloc_size.fixed;

        // top-down size overrides accumulated size
        let outer_size: AllocSize = if let Some(size) = size {
            size.into()
        } else {
            child.alloc_size.clone()
        };

        let c_view = outer_size.view;
        let c_fixed = outer_size.fixed;

        match self.alloc_dir {
            AllocDirection::Column => {
                self.alloc_size.view.width = view.width.max(c_view.width).min(1.);
                self.alloc_size.view.height = view.height + c_view.height;

                self.alloc_size.fixed.width = fixed.width.max(c_fixed.width);
                self.alloc_size.fixed.height = fixed.height + c_fixed.height;
            },
            AllocDirection::Row => {
                self.alloc_size.view.width = view.width + c_view.width;
                self.alloc_size.view.height = view.height.max(c_view.height).min(1.);

                self.alloc_size.fixed.width = fixed.width + c_fixed.width;
                self.alloc_size.fixed.height = fixed.height.max(c_fixed.height);
            },
        };

        AllocPair {
            outer: outer_size,
            inner: child.alloc_size.clone(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum AllocDirection {
    Column,
    Row,
}

impl AllocDirection {
}

#[derive(Debug)]
pub(crate) struct ViewBounds {
    width: f32,
    height: f32,
    view_width: f32,
    view_height: f32,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct AllocPair {
    pub outer: AllocSize,
    pub inner: AllocSize,
}

impl AllocPair {
    pub(crate) fn is_changed(&self, _old_alloc: &Option<AllocPair>) -> bool {
        false
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct AllocSize {
    pub fixed: Size,
    pub view: Size,
}

impl AllocSize {
    pub(crate) fn is_changed(&self, _alloc_cache: &Option<AllocSize>) -> bool {
        false
    }

    fn view_width(&self, width: f32) -> f32 {
        ((width - self.fixed.width) / self.view.width.max(1.)).floor()
    }

    fn view_height(&self, height: f32) -> f32 {
        ((height - self.fixed.height) / self.view.height.max(1.)).floor()
    }

    fn init_width(&self, factor: f32) -> f32 {
        if self.view.width > 0. {
            factor * self.view.width
        } else {
            self.fixed.width
        }
    }

    fn init_height(&self, factor: f32) -> f32 {
        if self.view.height > 0. {
            factor * self.view.height
        } else {
            self.fixed.height
        }
    }
}

impl From<Size<Length>> for AllocSize {
    fn from(value: Size<Length>) -> Self {
        let mut px_width = 0.;
        let mut view_width = 0.;

        match value.width {
            Length::Shrink => {},
            Length::Pixels(width) => { px_width = width; },
            Length::Fill => { view_width = 1.; },
            Length::View(width) => { view_width = width; },
        }

        let mut px_height = 0.;
        let mut view_height = 0.;

        match value.height {
            Length::Shrink => {},
            Length::Pixels(height) => { px_height = height; },
            Length::Fill => { view_height = 1.; },
            Length::View(height) => { view_height = height; },
        }

        AllocSize {
            fixed: Size::new(px_width, px_height).into(),
            view: Size::new(view_width, view_height).into()
        }
    }
}

#[cfg(test)]
mod test {
    use essay_graphics_api::Length;
    use essay_graphics_test::{TestGraphicsContext, TestRenderer};

    use crate::ui::{Context, Frame};

    #[test]
    fn default_label() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.label("Test");
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'Test'");
    }

    #[test]
    fn default_label_three() {
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
    fn default_column_fixed() {
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
    fn column_view_label() {
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
    fn column_view_frame_text() {
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
    fn row_view_frame() {
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
    fn col_view_frame_inner_frame() {
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
    fn col_2_view() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            Frame::group(ui).show(ui, |ui| {
                ui.view(|ui| {
                });
            });
            Frame::group(ui).show(ui, |ui| {
                ui.view(|ui| {
                });
            });
        }).unwrap();

        assert_eq!(test.take(), "rect (0.0,0.0) 1200.0x600.0 #ffffffff
rect (0.0,600.0) 1200.0x600.0 #ffffffff");
    }

    #[test]
    fn default_col_2_view() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.column(|ui| {
                Frame::group(ui).show(ui, |ui| {
                    ui.view(|ui| {
                    });
                });
                Frame::group(ui).show(ui, |ui| {
                    ui.view(|ui| {
                    });
                });
            });
        }).unwrap();

        assert_eq!(test.take(), "rect (0.0,0.0) 1200.0x600.0 #ffffffff
rect (0.0,600.0) 1200.0x600.0 #ffffffff");
    }

    #[test]
    fn col_size_2_view_label() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.column_with(Length::Fill, |ui| {
                ui.view(|ui| {
                    ui.label("a");
                });
                ui.view(|ui| {
                    ui.label("b");
                });
            });
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'a'
text (0.0,600.0) 'b'");
    }

    #[test]
    fn col_size_2_view_frame() {
        let mut test = TestRenderer::new([1200., 1200.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.column_with(Length::Fill, |ui| {
                Frame::group(ui).show(ui, |ui| {
                    ui.view(|ui| {
                    });
                });
                Frame::group(ui).show(ui, |ui| {
                    ui.view(|ui| {
                    });
                });
            });
        }).unwrap();

        assert_eq!(test.take(), "rect (0.0,0.0) 1200.0x600.0 #ffffffff
rect (0.0,600.0) 1200.0x600.0 #ffffffff");
    }

    #[test]
    fn col_view_frame() {
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
    fn row_col1_col2() {
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