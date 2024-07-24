use essay_graphics_api::{
    form::{Form, FormId, Matrix4}, renderer::{Canvas, RenderErr, Renderer}, Bounds, Clip, FontStyle, FontTypeId, ImageId, Path, PathOpt, Point, TextStyle, TextureId
};
use essay_tensor::Tensor;

use super::canvas::PlotCanvas;

pub struct PlotRenderer<'a> {
    canvas: &'a mut PlotCanvas,
    device: &'a wgpu::Device,
    queue: Option<&'a wgpu::Queue>,
    view: Option<&'a wgpu::TextureView>,
}

impl<'a> PlotRenderer<'a> {
    pub fn new(
        figure: &'a mut PlotCanvas,
        device: &'a wgpu::Device,
        queue: Option<&'a wgpu::Queue>,
        view: Option<&'a wgpu::TextureView>,
    ) -> Self {
        Self {
            device,
            canvas: figure,
            queue,
            view,
        }
    }

    pub fn flush_inner(&mut self, clip: &Clip) {
        if let Some(queue) = self.queue {
            if let Some(view) = self.view {
                let mut encoder =
                   self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                let scissor = self.canvas.to_scissor(clip);

                self.canvas.image_render.flush(queue, view, &mut encoder);
                self.canvas.triangle_render.flush(self.device, queue, view, &mut encoder, clip);
                // TODO: order issues with bezier and shape2d
                self.canvas.shape2d_render.flush(self.device, queue, view, &mut encoder, scissor);
                self.canvas.bezier_render.flush(self.device, queue, view, &mut encoder, scissor);
                self.canvas.shape2d_texture_render.flush(self.device, queue, view, &mut encoder, scissor);
                self.canvas.text_render.flush(queue, view, &mut encoder);

                self.canvas.form3d_render.flush(self.device, queue, view, &mut encoder);
        
                queue.submit(Some(encoder.finish()));
            }
        }
    }

}

impl Renderer for PlotRenderer<'_> {
    fn bounds(&self) -> &Bounds<Canvas> {
        self.canvas.bounds()
    }

    fn scale_factor(&self) -> f32 {
        self.canvas.scale_factor()
    }

    fn to_px(&self, size: f32) -> f32 {
        self.canvas.to_px(size)
    }

    fn draw_path(
        &mut self, 
        path: &Path<Canvas>, 
        style: &dyn PathOpt, 
        clip: &Clip,
    ) -> Result<(), RenderErr> {
        self.canvas.draw_path(path, style, clip)
    }

    fn draw_markers(
        &mut self, 
        marker: &Path<Canvas>, 
        xy: &Tensor,
        scale: &Tensor,
        color: &Tensor<u32>,
        style: &dyn PathOpt, 
        clip: &Clip,
    ) -> Result<(), RenderErr> {
        self.canvas.draw_markers(marker, xy, scale, color, style, clip)
    }

    fn font(
        &mut self,
        style: &FontStyle
    ) -> Result<FontTypeId, RenderErr> {
        self.canvas.font(style)
    }

    fn draw_text(
        &mut self, 
        xy: Point, // location in Canvas coordinates
        text: &str,
        angle: f32,
        style: &dyn PathOpt, 
        text_style: &TextStyle,
        clip: &Clip,
    ) -> Result<(), RenderErr> {
        self.canvas.draw_text(xy, text, angle, style, text_style, clip)
    }

    fn draw_triangles(
        &mut self,
        vertices: Tensor<f32>,  // Nx2 x,y in canvas coordinates
        colors: Tensor<u32>,    // N in rgba
        triangles: Tensor<u32>, // Mx3 vertex indices
        clip: &Clip,
    ) -> Result<(), RenderErr> {
        self.canvas.draw_triangles(vertices, colors, triangles, clip)
    }

    fn create_form(
        &mut self,
        form: &Form,
    ) -> FormId {
        self.canvas.create_form(form)
    }

    fn draw_form(
        &mut self,
        form: FormId,
        camera: &Matrix4,
        clip: &Clip,
    ) -> Result<(), RenderErr> {
        self.canvas.draw_form(form, camera, clip)
    }

    fn request_redraw(
        &mut self,
        _bounds: &Bounds<Canvas>
    ) {
        self.canvas.request_redraw(true)
    }

    fn draw_image(
        &mut self,
        bounds: &Bounds<Canvas>,
        colors: &Tensor<u8>,
        clip: &Clip
    ) -> Result<(), RenderErr> {
        let image = self.canvas.create_image(self.device, colors);

        self.canvas.draw_image_ref(self.device, bounds, image, clip)
    }

    fn create_image(
        &mut self,
        colors: &Tensor<u8>, // [rows, cols, 4]
    ) -> ImageId {
        self.canvas.create_image(self.device, colors)
    }

    fn create_texture_r8(
        &mut self,
        colors: &Tensor<u8>, // [rows, cols, 4]
    ) -> TextureId {
        self.canvas.create_texture(colors)
    }

    fn create_texture_rgba8(
        &mut self,
        colors: &Tensor<u8>, // [rows, cols, 4]
    ) -> TextureId {
        self.canvas.create_texture_rgba8(self.device, self.queue.unwrap(), colors)
    }

    fn draw_image_ref(
        &mut self,
        bounds: &Bounds<Canvas>,
        image: ImageId,
        clip: &Clip
    ) -> Result<(), RenderErr> {
        self.canvas.draw_image_ref(self.device, bounds, image, clip)
    }

    fn flush(
        &mut self,
        clip: &Clip
    ) {
        self.flush_inner(clip);
    }
}
