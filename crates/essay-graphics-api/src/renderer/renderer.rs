use std::ops::{Deref, DerefMut};

use essay_tensor::Tensor;

use crate::{
    form::{Form, FormId, Matrix4}, Bounds, Clip, FontStyle, FontTypeId, 
    ImageId, Path, PathOpt, Point, TextStyle, TextureId
};

use super::{Canvas, Drawable};

pub trait Renderer {
    ///
    /// Returns the boundary of the canvas, usually in pixels or points.
    ///
    fn bounds(&self) -> &Bounds<Canvas>;

    fn scale_factor(&self) -> f32;

    fn to_px(&self, size: f32) -> f32 {
        size * self.scale_factor()
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
        clip: &Clip
    ) -> Result<(), RenderErr>;

    fn create_form(
        &mut self,
        form: &Form,
    ) -> FormId;

    fn draw_form(
        &mut self,
        form: FormId,
        camera: &Matrix4,
        clip: &Clip,
    ) -> Result<(), RenderErr>;

    fn flush(
        &mut self,
        clip: &Clip
    );

    fn request_redraw(
        &mut self,
        bounds: &Bounds<Canvas>
    );

    fn sub_render(&mut self, pos: &Bounds<Canvas>, drawable: &mut dyn Drawable);
}

pub struct RendererGuard<'a> {
    ptr: &'a mut dyn Renderer,
}

impl<'a> RendererGuard<'a> {
    pub fn new(ptr: &'a mut dyn Renderer) -> Self {
        Self {
            ptr
        }
    }
}

impl Drop for RendererGuard<'_> {
    fn drop(&mut self) {
        println!("DropGuard");
    }
}

#[derive(Debug)]
pub enum RenderErr {
    NotImplemented,
}
