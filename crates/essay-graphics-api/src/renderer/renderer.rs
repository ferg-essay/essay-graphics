use essay_tensor::Tensor;

use crate::{
    form::{Form, FormId, Matrix4, Shape, ShapeId}, Affine2d, Bounds, FontStyle, FontTypeId, ImageId, Path, PathOpt, Point, TextStyle, TextureId
};

use super::{Canvas, Drawable};

pub trait Renderer {
    ///
    /// Returns the boundary of the full canvas, usually in pixels or points.
    ///
    fn extent(&self) -> &Bounds<Canvas>;

    ///
    /// Returns the position of the current view.
    ///
    fn pos(&self) -> &Bounds<Canvas>;

    fn scale_factor(&self) -> f32;

    fn to_px(&self, size: f32) -> f32 {
        size * self.scale_factor()
    }

    fn draw_path(
        &mut self, 
        path: &Path<Canvas>, 
        style: &dyn PathOpt, 
    ) -> Result<()>;

    fn draw_markers(
        &mut self, 
        marker: &Path<Canvas>, 
        xy: &Tensor,
        scale: &Tensor,
        color: &Tensor<u32>,
        style: &dyn PathOpt, 
    ) -> Result<()>;

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
    ) -> Result<()>;

    fn draw_triangles(
        &mut self,
        vertices: Tensor<f32>,  // Nx2 x,y in canvas coordinates
        colors: Tensor<u32>,    // N in rgba
        triangles: Tensor<u32>, // Mx3 vertex indices
    ) -> Result<()>;

    fn draw_image(
        &mut self,
        bounds: &Bounds<Canvas>,
        colors: &Tensor<u8>,  // [rows, cols, 4]
    ) -> Result<()>;

    fn create_image(
        &mut self,
        colors: &Tensor<u8>, // [rows, cols, 4]
    ) -> ImageId;

    fn create_texture_r8(
        &mut self,
        image: &Tensor<u8>, // [rows, cols, 4]
    ) -> TextureId;

    fn create_texture_rgba8(
        &mut self,
        texture: &Tensor<u8>, // [rows, cols, 4]
    ) -> TextureId;

    fn draw_image_ref(
        &mut self,
        bounds: &Bounds<Canvas>,
        image: ImageId,
    ) -> Result<()>;

    fn create_form(
        &mut self,
        form: &Form,
    ) -> FormId;

    fn draw_form(
        &mut self,
        form: FormId,
        camera: &Matrix4,
    ) -> Result<()>;

    fn create_shape(
        &mut self,
        form: &Shape,
    ) -> ShapeId;

    fn draw_shape(
        &mut self,
        form: ShapeId,
        camera: &Affine2d,
    ) -> Result<()>;

    fn flush(
        &mut self,
    );

    fn draw_with(
        &mut self, 
        pos: &Bounds<Canvas>, 
        drawable: 
        &mut dyn Drawable
    ) -> Result<()>;

    fn request_redraw(
        &mut self,
        bounds: &Bounds<Canvas>
    );
}

pub type Result<T, E=RenderErr> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum RenderErr {
    NotImplemented,
}
