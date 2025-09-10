use std::{any::Any, ops, sync::Arc};

use essay_tensor::tensor::Tensor;

use crate::{
    form::{Form, FormId, Matrix4}, input::Input, mesh2d::{BezierMesh2d, Mesh2d}, path_style::MeshStyle, Bounds, FontStyle, FontTypeId, Mesh2dColor, Path, PathOpt, Point, Shapes, Size, TextStyle, TextureId
};

use super::{Canvas, RenderErr, Result};

pub struct Painter<'a>(pub &'a mut dyn Renderer);

impl<'a> ops::Deref for Painter<'a> {
    type Target = &'a mut dyn Renderer;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ops::DerefMut for Painter<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait Renderer {
    ///
    /// Returns the position of the current view.
    ///
    fn pos(&self) -> Bounds<Canvas>;

    ///
    /// Returns the boundary of the full canvas, usually in pixels or points.
    ///
    fn extent(&self) -> Bounds<Canvas>;

    fn scale_factor(&self) -> f32;

    fn to_px(&self, size: f32) -> f32 {
        size * self.scale_factor()
    }

    fn text_size(
        &mut self,
        text: &str,
        text_style: &TextStyle
    ) -> Size;

    //
    // drawing primitives
    //

    fn draw_path(
        &mut self, 
        path: &Path<Canvas>, 
        style: &dyn PathOpt, 
    ) -> Result<()>;

    fn draw_markers(
        &mut self, 
        marker: &Path<Canvas>, 
        path_style: &dyn PathOpt,
        marker_style: &[MeshStyle],
    ) -> Result<()>;

    fn draw_mesh2d(
        &mut self,
        mesh: &Mesh2d,
        texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<()>;

    fn create_mesh2d_buffer(
        &mut self,
        mesh: &Mesh2d,
    ) -> Result<Mesh2dBuffer>;

    fn draw_mesh2d_buffer(
        &mut self,
        mesh: &Mesh2dBuffer,
        texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<()>;

    fn draw_bezier_mesh(
        &mut self,
        mesh: &BezierMesh2d,
        texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<()>;

    fn draw_mesh2d_color(
        &mut self,
        mesh: &Mesh2dColor,
    ) -> Result<()>;

    fn draw_shape(
        &mut self, 
        shape: &Shapes,
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

    fn create_texture_rgba8(
        &mut self,
        texture: &Tensor<u8>, // [rows, cols, 4]
    ) -> TextureId;

    fn create_form(
        &mut self,
        form: &Form,
    ) -> FormId;

    fn draw_form(
        &mut self,
        form: FormId,
        camera: &Matrix4,
    ) -> Result<()>;

    fn flush(
        &mut self,
    );

    fn draw_with<'a>(
        &mut self, 
        pos: Bounds<Canvas>, 
        draw: Box<dyn FnOnce(&mut dyn Renderer) -> Result<()> + 'a>,
    ) -> Result<()>;

    fn draw_with_clip<'a>(
        &mut self, 
        pos: Bounds<Canvas>, 
        draw: Box<dyn FnOnce(&mut dyn Renderer) -> Result<()> + 'a>,
    ) -> Result<()>;

    fn input(&self) -> &Input;

    fn request_redraw(
        &mut self,
        bounds: Bounds<Canvas>
    );
}

#[derive(Clone)]
pub struct Mesh2dBuffer(pub Arc<Box<dyn Any + Send + Sync>>);

impl Mesh2dBuffer {
    pub fn new(value: impl Any + Send + Sync) -> Mesh2dBuffer {
        Self(Arc::new(Box::new(value)))
    }
}
