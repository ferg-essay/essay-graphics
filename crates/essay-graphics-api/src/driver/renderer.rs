use essay_tensor::Tensor;

use crate::{
    Bounds, Canvas, Clip, FontStyle, FontTypeId, ImageId, Path, PathOpt, Point, TextStyle, TextureId
};

pub trait Renderer {
    ///
    /// Returns the boundary of the canvas, usually in pixels or points.
    ///
    fn get_canvas(&self) -> &Canvas;

    fn to_px(&self, size: f32) -> f32 {
        size
    }

    fn draw_path(
        &mut self, 
        path: &Path<Canvas>, 
        style: &dyn PathOpt, 
        clip: &Clip,
    ) -> Result<(), RenderErr>;

    fn draw_markers(
        &mut self, 
        marker: &Path<Canvas>, 
        xy: &Tensor,
        scale: &Tensor,
        color: &Tensor<u32>,
        style: &dyn PathOpt, 
        clip: &Clip,
    ) -> Result<(), RenderErr>;

    fn font(
        &mut self,
        font_style: &FontStyle
    ) -> Result<FontTypeId, RenderErr>;

    fn draw_text(
        &mut self, 
        xy: Point, // location in Canvas coordinates
        text: &str,
        angle: f32,
        style: &dyn PathOpt, 
        text_style: &TextStyle,
        clip: &Clip,
    ) -> Result<(), RenderErr>;

    fn draw_triangles(
        &mut self,
        vertices: Tensor<f32>,  // Nx2 x,y in canvas coordinates
        colors: Tensor<u32>,    // N in rgba
        triangles: Tensor<u32>, // Mx3 vertex indices
        clip: &Clip,
    ) -> Result<(), RenderErr>;

    fn draw_image(
        &mut self,
        bounds: &Bounds<Canvas>,
        colors: &Tensor<u8>,  // [rows, cols, 4]
        clip: &Clip
    ) -> Result<(), RenderErr>;

    fn create_image(
        &mut self,
        colors: &Tensor<u8>, // [rows, cols, 4]
    ) -> ImageId;

    fn create_texture(
        &mut self,
        image: &Tensor<u8>, // [rows, cols, 4]
    ) -> TextureId;

    fn draw_image_ref(
        &mut self,
        bounds: &Bounds<Canvas>,
        image: ImageId,
        clip: &Clip
    ) -> Result<(), RenderErr>;

    fn flush(
        &mut self,
        clip: &Clip
    );

    fn request_redraw(
        &mut self,
        bounds: &Bounds<Canvas>
    );
}

#[derive(Debug)]
pub enum RenderErr {
    NotImplemented,
}
