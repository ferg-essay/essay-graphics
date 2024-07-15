use essay_graphics_api::{
    driver::Renderer, Bounds, Canvas, Affine2d,
};

use super::FrameId;

pub struct Frame {
    id: FrameId,
    
    pos: Bounds<Canvas>,

    _to_canvas: Affine2d,
}

impl Frame {
    pub(crate) fn new(id: FrameId) -> Self {
        Self {
            id,

            pos: Bounds::none(),

            _to_canvas: Affine2d::eye(),
        }
    }

    #[inline]
    pub fn id(&self) -> FrameId {
        self.id
    }

    pub(crate) fn pos(&self) -> &Bounds<Canvas> {
        &self.pos
    }

    pub(crate) fn update(&mut self, _canvas: &Canvas) {
    }

    ///
    /// Sets the device bounds and propagates to children
    /// 
    /// The position for a frame is the size of the data box. The frame,
    /// axes and titles are relative to the data box.
    /// 
    pub(crate) fn set_pos(&mut self, _pos: &Bounds<Canvas>) -> &mut Self {
        self
    }

    pub(crate) fn draw(&mut self, _renderer: &mut dyn Renderer) {
        /*
        let clip = Clip::from(&self.pos);

        let frame_to_canvas = ToCanvas::new(
            self.pos.clone(), 
            self.to_canvas.clone()
        );
        */

        // let to_canvas = ToCanvas::new(
        //    self.pos.clone(), 
        //     self.data.get_canvas_transform().clone()
        // );
    }
}
