use essay_graphics_api::{
    form::{Form, FormId, Matrix4}, input::Input, path_style::MeshStyle, renderer::{Canvas, RenderErr, Renderer, Result}, BezierMesh2d, Bounds, FontStyle, FontTypeId, Mesh2d, Mesh2dColor, Path, PathOpt, Point, Size, TextStyle, TextureId
};
use essay_tensor::{tensor::Tensor};

pub struct NullRenderer<'a>(pub &'a mut dyn Renderer);

impl Renderer for NullRenderer<'_> {
    fn pos(&self) -> Bounds<Canvas> {
        self.0.pos()
    }

    fn extent(&self) -> Bounds<Canvas> {
        self.0.extent()
    }

    fn scale_factor(&self) -> f32 {
        self.0.scale_factor()
    }

    fn text_size(
        &mut self,
        text: &str,
        text_style: &TextStyle
    ) -> Size {
        self.0.text_size(text, text_style)
    }

    fn draw_path(
        &mut self, 
        _path: &Path<Canvas>, 
        _style: &dyn PathOpt, 
    ) -> Result<()> {
        Ok(())
    }

    fn draw_markers(
        &mut self, 
        _marker: &Path<Canvas>, 
        _path_style: &dyn PathOpt,
        _marker_style: &[MeshStyle],
    ) -> Result<()> {
        Ok(())
    }

    fn draw_mesh2d(
        &mut self,
        _mesh: &Mesh2d,
        _texture: TextureId,
        _style: &[MeshStyle],
    ) -> Result<()> {
        Ok(())
    }

    fn draw_bezier_mesh(
        &mut self,
        _mesh: &BezierMesh2d,
        _texture: TextureId,
        _style: &[MeshStyle],
    ) -> Result<()> {
        Ok(())    
    }

    fn draw_mesh2d_color(
        &mut self,
        _mesh: &Mesh2dColor,
    ) -> Result<()> {
        Ok(())
    }

    fn font(
        &mut self,
        font_style: &FontStyle
    ) -> Result<FontTypeId, RenderErr> {
        self.0.font(font_style)
    }

    fn draw_text(
        &mut self, 
        _xy: Point, // location in Canvas coordinates
        _text: &str,
        _angle: f32,
        _style: &dyn PathOpt, 
        _text_style: &TextStyle,
    ) -> Result<()> {
        Ok(())
    }

    fn create_texture_rgba8(
        &mut self,
        texture: &Tensor<u8>, // [rows, cols, 4]
    ) -> TextureId {
        self.0.create_texture_rgba8(texture)
    }

    fn create_form(
        &mut self,
        form: &Form,
    ) -> FormId {
        self.0.create_form(form)
    }

    fn draw_form(
        &mut self,
        form: FormId,
        camera: &Matrix4,
    ) -> Result<()> {
        Ok(())
    }

    fn flush(
        &mut self,
    ) {
    }

    fn draw_with<'a>(
        &mut self, 
        pos: Bounds<Canvas>, 
        draw: Box<dyn FnOnce(&mut dyn Renderer) -> Result<()> + 'a>,
    ) -> Result<()> {
        self.0.draw_with(pos, Box::new(|ui| {
            draw(&mut NullRenderer(ui))
        }))
    }

    fn draw_with_clip<'a>(
        &mut self, 
        pos: Bounds<Canvas>, 
        draw: Box<dyn FnOnce(&mut dyn Renderer) -> Result<()> + 'a>,
    ) -> Result<()> {
        self.0.draw_with_clip(pos, Box::new(|ui| {
            draw(&mut NullRenderer(ui))
        }))
    }

    fn input(&self) -> &Input {
        self.0.input()
    }

    fn request_redraw(
        &mut self,
        bounds: Bounds<Canvas>
    ) {
        self.0.request_redraw(bounds)
    }
}