use crate::{artist::{Style, patch::{PatchTrait, self, DisplayPatch, Line}, ArtistTrait}, driver::{Canvas, Renderer}, frame::Affine2d, figure::GridSpec};

use super::{Display, Bounds, Point, databox::DataBox, Data};

pub struct Frame {
    pos: Bounds<Canvas>,

    to_canvas: Affine2d,
    style: Style,

    data: DataBox,

    bottom: SpineX,
    left: SpineY,
    top: SpineTop,
    right: SpineRight,

    artists: Vec<Box<dyn ArtistTrait<Data>>>,
}

impl Frame {
    pub(crate) fn new() -> Self {
        Self {
            pos: Bounds::none(),

            artists: Vec::new(),

            data: DataBox::new(),

            bottom: SpineX::new(),
            left: SpineY::new(),
            top: SpineTop::new(),
            right: SpineRight::new(),

            style: Style::default(),

            to_canvas: Affine2d::eye(),
        }
    }

    pub(crate) fn pos(&self) -> &Bounds<Canvas> {
        &self.pos
    }

    ///
    /// Sets the device bounds and propagates to children
    /// 
    pub(crate) fn set_pos(&mut self, pos: &Bounds<Canvas>) -> &mut Self {
        self.pos = pos.clone();

        let bottom = self.bottom.get_bounds();
        let left = self.left.get_bounds();
        let top = self.top.get_bounds();
        let right = self.right.get_bounds();

        let pos_data = Bounds::<Canvas>::new(
            Point(left.width(), bottom.height()), 
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

        self
    }

    pub(crate) fn data_mut(&mut self) -> &mut DataBox {
        &mut self.data
    }

    pub(crate) fn draw(&mut self, renderer: &mut impl Renderer) {
        self.data.draw(renderer, &self.to_canvas, &self.pos, &self.style);

        self.bottom.draw(renderer, &self.to_canvas, &self.pos, &self.style);
        self.left.draw(renderer, &self.to_canvas, &self.pos, &self.style);

        self.top.draw(renderer, &self.to_canvas, &self.pos, &self.style);
        self.right.draw(renderer, &self.to_canvas, &self.pos, &self.style);
    }
}

pub struct SpineTop {
    bounds: Bounds<Canvas>,
    pos: Bounds<Canvas>,
    spine: Option<DisplayPatch>,
}

impl SpineTop {
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

impl ArtistTrait<Canvas> for SpineTop {
    fn get_bounds(&mut self) -> Bounds<Canvas> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn crate::driver::Renderer,
        to_canvas: &super::Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn crate::artist::StyleOpt,
    ) {
        //let affine = Affine2d::eye().translate(self.pos.xmin(), self.pos.ymin());

        if let Some(patch) = &mut self.spine {
            patch.draw(renderer, to_canvas, clip, style);
        }
        
    }
}

pub struct SpineX {
    bounds: Bounds<Canvas>,
    pos: Bounds<Canvas>,
    spine: Option<DisplayPatch>,
}

impl SpineX {
    pub fn new() -> Self {
        Self {
            bounds: Bounds::new(Point(0., 0.), Point(0., 20.)),
            pos: Bounds::none(),
            spine: Some(DisplayPatch::new(Line::new(Point(0., 0.), Point(1., 0.)))),
        }
    }

    pub fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.pos = pos.clone();
        println!("Spine {:?}", pos);
        if let Some(spine) = &mut self.spine {
            spine.set_pos(Bounds::new(
                Point(pos.xmin(), pos.ymax() - 1.),
                Point(pos.xmax(), pos.ymax()),
            ))
        }
    }
}

impl ArtistTrait<Canvas> for SpineX {
    fn get_bounds(&mut self) -> Bounds<Canvas> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn crate::driver::Renderer,
        to_canvas: &super::Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn crate::artist::StyleOpt,
    ) {
        //let affine = Affine2d::eye().translate(self.pos.xmin(), self.pos.ymin());

        if let Some(patch) = &mut self.spine {
            patch.draw(renderer, to_canvas, clip, style);
        }
        
    }
}

pub struct SpineY {
    bounds: Bounds<Canvas>,
    pos: Bounds<Canvas>,
    spine: Option<DisplayPatch>,
}

impl SpineY {
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
            println!("SpinePos {:?}", pos);
            spine.set_pos(Bounds::new(
                Point(pos.xmax() - 1., pos.ymin()),
                Point(pos.xmax(), pos.ymax()),
            ))
        }
    }
}

impl ArtistTrait<Canvas> for SpineY {
    fn get_bounds(&mut self) -> Bounds<Canvas> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn crate::driver::Renderer,
        to_canvas: &super::Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn crate::artist::StyleOpt,
    ) {
        if let Some(patch) = &mut self.spine {
            patch.draw(renderer, to_canvas, clip, style);
        }
        
    }
}

pub struct SpineRight {
    bounds: Bounds<Canvas>,
    pos: Bounds<Canvas>,
    spine: Option<DisplayPatch>,
}

impl SpineRight {
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

impl ArtistTrait<Canvas> for SpineRight {
    fn get_bounds(&mut self) -> Bounds<Canvas> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn crate::driver::Renderer,
        to_canvas: &super::Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn crate::artist::StyleOpt,
    ) {
        if let Some(patch) = &mut self.spine {
            patch.draw(renderer, to_canvas, clip, style);
        }
        
    }
}

pub struct Spine {
    style: Style,
    patch: Box<dyn PatchTrait>,
}

impl Spine {
    pub fn new() -> Self {
        Self {
            style: Style::new(),
            patch: Box::new(patch::Line::new([0., 0.], [0., 0.])),
        }
    }
}