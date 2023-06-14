use std::f32::consts::PI;

use essay_plot_base::{
    Style, PathCode, Path, StyleOpt,
    driver::{Renderer}, Bounds, Canvas, Affine2d, Point, CanvasEvent, 
};

use crate::artist::{patch::{DisplayPatch, Line, PathPatch}, Text, ArtistTrait};

use super::{databox::DataBox, axis::Axis, tick_formatter::{Formatter, TickFormatter}};

pub struct Frame {
    pos: Bounds<Canvas>,

    to_canvas: Affine2d,
    style: Style,

    data: DataBox,

    bottom: BottomFrame,
    left: LeftFrame,
    top: TopFrame,
    right: RightFrame,
}

impl Frame {
    pub(crate) fn new() -> Self {
        Self {
            pos: Bounds::none(),

            data: DataBox::new(),

            bottom: BottomFrame::new(),
            left: LeftFrame::new(),
            top: TopFrame::new(),
            right: RightFrame::new(),

            style: Style::default(),

            to_canvas: Affine2d::eye(),
        }
    }

    pub(crate) fn pos(&self) -> &Bounds<Canvas> {
        &self.pos
    }

    pub(crate) fn update_extent(&mut self, canvas: &Canvas) {
        self.bottom.update_extent(canvas);
        self.left.update_extent(canvas);
        self.top.update_extent(canvas);
        self.right.update_extent(canvas);
    }
        ///
    /// Sets the device bounds and propagates to children
    /// 
    pub(crate) fn set_pos(&mut self, pos: &Bounds<Canvas>) -> &mut Self {
        self.pos = pos.clone();

        let bottom = self.bottom.get_extent();
        let left = self.left.get_extent();
        let top = self.top.get_extent();
        let right = self.right.get_extent();

        let pos_data = Bounds::<Canvas>::new(
            Point(pos.xmin() + left.width(), pos.ymin() + bottom.height()), 
            Point(pos.xmax() - right.width(), pos.ymax() - top.height())
        );

        self.data.set_pos(&pos_data);

        let pos_bottom = Bounds::<Canvas>::new(
            Point(pos_data.xmin(), pos.ymin()),
            Point(pos_data.xmax(), pos_data.ymin()),
        );
        self.bottom.set_pos(pos_bottom);

        let pos_left = Bounds::<Canvas>::new(
            Point(pos.xmin(), pos_data.ymin()),
            Point(pos_data.xmin(), pos_data.ymax()),
        );
        self.left.set_pos(pos_left);

        let pos_top = Bounds::<Canvas>::new(
            Point(pos_data.xmin(), pos_data.ymax()),
            Point(pos_data.xmax(), pos.ymax()),
        );
        self.top.set_pos(pos_top);

        let pos_right = Bounds::<Canvas>::new(
            Point(pos_data.xmax(), pos_data.ymin()),
            Point(pos.xmax(), pos_data.ymax()),
        );
        self.right.set_pos(pos_right);

        self.bottom.calculate_axis(&self.data);
        self.left.calculate_axis(&self.data);

        self
    }

    pub(crate) fn data_mut(&mut self) -> &mut DataBox {
        &mut self.data
    }

    pub(crate) fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        if self.data.get_pos().contains(event.point()) {
            if self.data.event(renderer, event) {
                self.left.calculate_axis(&self.data);
                self.bottom.calculate_axis(&self.data);

                renderer.request_redraw(&self.pos);
            };
        }
    }

    pub(crate) fn draw(&mut self, renderer: &mut dyn Renderer) {
        self.bottom.draw(renderer, &self.to_canvas, &self.pos, &self.style);
        self.left.draw(renderer, &self.to_canvas, &self.pos, &self.style);

        self.top.draw(renderer, &self.to_canvas, &self.pos, &self.style);
        self.right.draw(renderer, &self.to_canvas, &self.pos, &self.style);

        // TODO: grid order
        self.data.draw(renderer, &self.to_canvas, &self.pos, &self.style);
    }

    pub fn xlabel(&mut self, text: &str) -> &mut Text {
        self.bottom.label(text)
    }

    pub fn ylabel(&mut self, text: &str) -> &mut Text {
        self.left.label(text)
    }
}

//
// FrameExtent
//

pub struct FrameExtent {
    data: Bounds<Canvas>,

    bottom: Bounds<Canvas>,
    left: Bounds<Canvas>,
    top: Bounds<Canvas>,
    right: Bounds<Canvas>,
}

//
// Top Frame
//

pub struct TopFrame {
    bounds: Bounds<Canvas>,
    pos: Bounds<Canvas>,
    spine: Option<DisplayPatch>,
}

impl TopFrame {
    pub fn new() -> Self {
        Self {
            bounds: Bounds::new(Point(0., 0.), Point(0., 20.)),
            pos: Bounds::none(),
            spine: Some(DisplayPatch::new(Line::new(Point(0., 0.), Point(1., 0.)))),
        }
    }

    pub fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.pos = pos.clone();

        if let Some(spine) = &mut self.spine {
            spine.set_pos(Bounds::new(
                Point(pos.xmin(), pos.ymin()),
                Point(pos.xmax(), pos.ymin() + 1.),
            ))
        }
    }
}

impl ArtistTrait<Canvas> for TopFrame {
    fn get_extent(&mut self) -> Bounds<Canvas> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn StyleOpt,
    ) {
        if let Some(patch) = &mut self.spine {
            patch.draw(renderer, to_canvas, clip, style);
        }
        
    }
}

pub struct FrameSizes {
    margin: f32,
    spine_thickness: f32,
    tick_length: f32,
    tick_label_gap: f32,
    tick_text_height: f32,
    label_title_gap: f32,
}

impl FrameSizes {
    fn new() -> Self {
        Self {
            margin: 20.,
            spine_thickness: 4.,
            tick_length: 10.,
            tick_label_gap: 4.,
            tick_text_height: 28.,
            label_title_gap: 10.,
        }
    }
}

//
// Bottom frame
//

pub struct BottomFrame {
    pos: Bounds<Canvas>,
    extent: Bounds<Canvas>,

    sizes: FrameSizes,

    spine: Option<DisplayPatch>,

    axis: Option<Axis>,
    ticks: Vec<Box<dyn ArtistTrait<Canvas>>>,
    style_major: Style,
    grid_major: Vec<Box<dyn ArtistTrait<Canvas>>>,
    style_minor: Style,
    grid_minor: Vec<Box<dyn ArtistTrait<Canvas>>>,

    label: Text,
}

impl BottomFrame {
    pub fn new() -> Self {
        let mut style_major = Style::new();
        style_major.linewidth(1.);
        style_major.color(0xbfbfbf);
        let mut style_minor = Style::new();
        style_minor.linewidth(1.);
        style_minor.color(0x404040);

        let frame = Self {
            extent: Bounds::zero(),
            pos: Bounds::none(),
            sizes: FrameSizes::new(),
            spine: Some(DisplayPatch::new(Line::new(Point(0., 0.), Point(1., 0.)))),
            axis: Some(Axis::new()),
            ticks: Vec::new(),
            grid_major: Vec::new(),
            style_major,
            grid_minor: Vec::new(),
            style_minor,

            label: Text::new(),
        };

        frame
    }

    pub fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.pos = pos.clone();

        if let Some(spine) = &mut self.spine {
            spine.set_pos(Bounds::new(
                Point(pos.xmin(), pos.ymax() - 1.),
                Point(pos.xmax(), pos.ymax()),
            ))
        }

        //self.label.set_pos(Bounds::new(
        //    Point(pos.xmin(), pos.ymin() + self.sizes.margin),
        //    Point(pos.xmax(), pos.ymin() + self.sizes.margin + self.label.height()),
        //));

        self.label.set_pos(Bounds::new(
            Point(pos.xmin(), pos.ymin()),
            Point(pos.xmax(), pos.ymin() + self.label.height()),
        ));
    }

    pub fn calculate_axis(&mut self, data: &DataBox) {
        if let Some(axis) = &mut self.axis {
            let pos = &self.pos;
            let data_pos = data.get_pos();

            self.ticks = Vec::new();
            self.grid_major = Vec::new();

            let x0 = data_pos.xmin();

            let sizes = &self.sizes;

            for (xv, x) in axis.x_ticks(data) {
                if 0. <= x && x <= data_pos.width() {
                    // grid
                    let grid = PathPatch::new(Path::new(vec![
                        PathCode::MoveTo(Point(x + x0, data_pos.ymin())),
                        PathCode::LineTo(Point(x + x0, data_pos.ymax())),
                    ]));

                    self.grid_major.push(Box::new(grid));

                    let mut y = pos.ymax();

                    let tick = PathPatch::new(Path::new(vec![
                        PathCode::MoveTo(Point(x + x0, y - sizes.tick_length)),
                        PathCode::LineTo(Point(x + x0, y)),
                    ]));
                    y -= sizes.tick_length;

                    self.ticks.push(Box::new(tick));

                    let mut label = Text::new();
                    label.text(&Formatter::Plain.format(xv));
                    label.set_pos(Bounds::from((x + x0, y - sizes.tick_text_height)));
                    self.ticks.push(Box::new(label));
                }
            };
        }
    }

    fn label(&mut self, text: &str) -> &mut Text {
        self.label.text(text)
    }
}

impl ArtistTrait<Canvas> for BottomFrame {
    fn get_extent(&mut self) -> Bounds<Canvas> {
        self.extent.clone()
    }

    fn update_extent(&mut self, canvas: &Canvas) {
        self.label.update_extent(canvas);

        let sizes = &self.sizes;
        let mut height = sizes.margin;
        height += sizes.spine_thickness;
        height += sizes.tick_length;
        height += sizes.tick_label_gap;
        height += sizes.tick_text_height; // font size
        height += self.label.get_extent().height();
    
        // self.bounds = Bounds::from([0., height]);
    
        self.extent = Bounds::extent(self.label.get_extent().width(), height)
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn StyleOpt,
    ) {
        //let affine = Affine2d::eye().translate(self.pos.xmin(), self.pos.ymin());
        self.label.draw(renderer, to_canvas, clip, style);

        if let Some(patch) = &mut self.spine {
            patch.draw(renderer, to_canvas, clip, style);
        }
        
        for tick in &mut self.ticks {
            tick.draw(renderer, to_canvas, clip, style);
        }
        
        for grid in &mut self.grid_major {
            grid.draw(renderer, to_canvas, clip, &self.style_major);
        }

        for grid in &mut self.grid_minor {
            grid.draw(renderer, to_canvas, clip, &self.style_minor);
        }
    }
}

//
// Left frame
//

pub struct LeftFrame {
    bounds: Bounds<Canvas>,
    pos: Bounds<Canvas>,
    spine: Option<DisplayPatch>,

    axis: Option<Axis>,
    ticks: Vec<Box<dyn ArtistTrait<Canvas>>>,
    style_major: Style,
    grid_major: Vec<Box<dyn ArtistTrait<Canvas>>>,
    style_minor: Style,
    grid_minor: Vec<Box<dyn ArtistTrait<Canvas>>>,

    label: Text,
}

impl LeftFrame {
    pub fn new() -> Self {
        let mut style_major = Style::new();
        style_major.linewidth(1.0);
        style_major.color(0xbfbfbf);
        let mut style_minor = Style::new();
        style_minor.linewidth(1.);
        style_minor.color(0x404040);

        let mut label = Text::new();
        label.angle(PI / 2.);

        Self {
            bounds: Bounds::new(Point(0., 0.), Point(20., 0.)),
            pos: Bounds::none(),
            spine: Some(DisplayPatch::new(Line::new(Point(0., 0.), Point(0., 1.)))),
            axis: Some(Axis::new()),
            ticks: Vec::new(),
            grid_major: Vec::new(),
            style_major,
            grid_minor: Vec::new(),
            style_minor,

            label,
        }
    }

    pub fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.pos = pos.clone();

        if let Some(spine) = &mut self.spine {
            spine.set_pos(Bounds::new(
                Point(pos.xmax() - 1., pos.ymin()),
                Point(pos.xmax(), pos.ymax()),
            ))
        }

        self.label.set_pos(Bounds::new(
            Point(pos.xmin(), pos.ymid()),
            Point(pos.xmax() - 20., pos.ymid())
        ));
    }

    pub fn calculate_axis(&mut self, data: &DataBox) {
        if let Some(axis) = &mut self.axis {
            let pos = &self.pos;
            let data_pos = data.get_pos();

            self.ticks = Vec::new();
            self.grid_major = Vec::new();

            let y0 = data_pos.ymin();

            for (_yv, y) in axis.y_ticks(data) {
                if 0. <= y && y <= data_pos.height() {
                    // grid
                    let grid = PathPatch::new(Path::new(vec![
                        PathCode::MoveTo(Point(data_pos.xmin(), y + y0)),
                        PathCode::LineTo(Point(data_pos.xmax(), y + y0)),
                    ]));

                    self.grid_major.push(Box::new(grid));

                    let tick = PathPatch::new(Path::new(vec![
                        PathCode::MoveTo(Point(pos.xmax() - 10., y + y0)),
                        PathCode::LineTo(Point(pos.xmax(), y + y0)),
                    ]));

                    self.ticks.push(Box::new(tick));
                }
            };
        }
    }

    fn label(&mut self, text: &str) -> &mut Text {
        self.label.text(text)
    }
}

impl ArtistTrait<Canvas> for LeftFrame {
    fn get_extent(&mut self) -> Bounds<Canvas> {
        self.bounds.clone()
    }

    fn update_extent(&mut self, canvas: &Canvas) {
        let width = self.label.get_extent().width() + 20.;
        
        self.bounds = Bounds::new(Point(0., 0.), Point(width, 0.))
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn StyleOpt,
    ) {
        self.label.draw(renderer, to_canvas, clip, style);

        if let Some(patch) = &mut self.spine {
            patch.draw(renderer, to_canvas, clip, style);
        }
        
        for tick in &mut self.ticks {
            tick.draw(renderer, to_canvas, clip, style);
        }
        
        for grid in &mut self.grid_major {
            grid.draw(renderer, to_canvas, clip, &self.style_major);
        }

        for grid in &mut self.grid_minor {
            grid.draw(renderer, to_canvas, clip, &self.style_minor);
        }
    }
}

//
// Right frame
//

pub struct RightFrame {
    bounds: Bounds<Canvas>,
    pos: Bounds<Canvas>,
    spine: Option<DisplayPatch>,
}

impl RightFrame {
    pub fn new() -> Self {
        Self {
            bounds: Bounds::new(Point(0., 0.), Point(20., 0.)),
            pos: Bounds::none(),
            spine: Some(DisplayPatch::new(Line::new(Point(0., 0.), Point(0., 1.)))),
        }
    }

    pub fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.pos = pos.clone();

        if let Some(spine) = &mut self.spine {
            spine.set_pos(Bounds::new(
                Point(pos.xmin(), pos.ymin()),
                Point(pos.xmin() + 1., pos.ymax()),
            ))
        }
    }
}

impl ArtistTrait<Canvas> for RightFrame {
    fn get_extent(&mut self) -> Bounds<Canvas> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn StyleOpt,
    ) {
        if let Some(patch) = &mut self.spine {
            patch.draw(renderer, to_canvas, clip, style);
        }
    }
}
