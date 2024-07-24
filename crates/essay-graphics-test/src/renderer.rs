use essay_graphics_api::{
    form::{Form, FormId, Matrix4}, 
    renderer::{Canvas, Drawable, RenderErr, Renderer, RendererGuard}, 
    Bounds, Clip, FontStyle, FontTypeId, ImageId, Path, PathOpt, Point, TextStyle, TextureId
};
use essay_tensor::Tensor;

pub struct TestRenderer {
    bounds: Bounds<Canvas>,
    scale_factor: f32,

    vec: Vec<String>,
}

impl TestRenderer {
    pub fn new(bounds: impl Into<Bounds<Canvas>>) -> Self {
        Self {
            bounds: bounds.into(),
            scale_factor: 1.,
            vec: Vec::new(),
        }
    }

    pub fn set_bounds(&mut self, bounds: impl Into<Bounds<Canvas>>) {
        self.bounds = bounds.into();
    }

    pub fn set_scale_factor(&mut self, scale_factor: f32) {
        self.scale_factor = scale_factor;
    }

    fn push(&mut self, str: &str) -> &mut Self {
        self.vec.push(String::from(str));

        self
    }

    pub fn drain(&mut self) -> Vec<String> {
        self.vec.drain(..).collect()
    }
}

impl Renderer for TestRenderer {
    fn bounds(&self) -> &Bounds<Canvas> {
        &self.bounds
    }

    fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    fn draw_path(
        &mut self, 
        _path: &Path<Canvas>, 
        _style: &dyn PathOpt, 
        _clip: &Clip,
    ) -> Result<(), RenderErr> {
        todo!()
    }

    fn draw_markers(
        &mut self, 
        _marker: &Path<Canvas>, 
        _xy: &Tensor,
        _scale: &Tensor,
        _color: &Tensor<u32>,
        _style: &dyn PathOpt, 
        _clip: &Clip,
    ) -> Result<(), RenderErr> {
        todo!()
    }

    fn font(
        &mut self,
        _font_style: &FontStyle
    ) -> Result<FontTypeId, RenderErr> {
        todo!()
    }

    fn draw_text(
        &mut self, 
        _xy: Point, // location in Canvas coordinates
        _text: &str,
        _angle: f32,
        _style: &dyn PathOpt, 
        _text_style: &TextStyle,
        _clip: &Clip,
    ) -> Result<(), RenderErr> {
        todo!()
    }

    fn draw_triangles(
        &mut self,
        _vertices: Tensor<f32>,  // Nx2 x,y in canvas coordinates
        _colors: Tensor<u32>,    // N in rgba
        _triangles: Tensor<u32>, // Mx3 vertex indices
        _clip: &Clip,
    ) -> Result<(), RenderErr> {
        todo!()
    }

    fn draw_image(
        &mut self,
        _bounds: &Bounds<Canvas>,
        _colors: &Tensor<u8>,  // [rows, cols, 4]
        _clip: &Clip
    ) -> Result<(), RenderErr> {
        todo!()
    }

    fn create_image(
        &mut self,
        _colors: &Tensor<u8>, // [rows, cols, 4]
    ) -> ImageId {
        todo!()
    }

    fn create_texture_r8(
        &mut self,
        _image: &Tensor<u8>, // [rows, cols, 4]
    ) -> TextureId {
        todo!()
    }

    fn create_texture_rgba8(
        &mut self,
        _texture: &Tensor<u8>, // [rows, cols, 4]
    ) -> TextureId {
        todo!()
    }

    fn draw_image_ref(
        &mut self,
        _bounds: &Bounds<Canvas>,
        _image: ImageId,
        _clip: &Clip
    ) -> Result<(), RenderErr> {
        todo!()
    }

    fn create_form(
        &mut self,
        _form: &Form,
    ) -> FormId {
        todo!()
    }

    fn draw_form(
        &mut self,
        _form: FormId,
        _camera: &Matrix4,
        _clip: &Clip,
    ) -> Result<(), RenderErr> {
        todo!()
    }

    fn flush(
        &mut self,
        clip: &Clip
    ) {
        match clip {
            Clip::None => { self.push("flush"); },
            Clip::Bounds(_, _) => todo!(),
        }
    }

    fn request_redraw(
        &mut self,
        _bounds: &Bounds<Canvas>
    ) {
        todo!()
    }
    
    fn sub_render(&mut self, _pos: &Bounds<Canvas>, drawable: &mut dyn Drawable) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use essay_graphics_api::{renderer::{Canvas, Renderer}, Bounds, Clip};

    use super::TestRenderer;

    #[test]
    fn bounds() {
        let mut test = TestRenderer::new((1., 2., 30., 40.));

        assert_eq!(test.bounds(), &Bounds::<Canvas>::from((1., 2., 30., 40.)));
        assert_eq!(test.drain(), Vec::<String>::new());
    }

    #[test]
    fn flush() {
        let mut test = TestRenderer::new([1., 1.]);

        test.flush(&Clip::None);

        assert_eq!(test.drain(), &["flush"]);
    }
}