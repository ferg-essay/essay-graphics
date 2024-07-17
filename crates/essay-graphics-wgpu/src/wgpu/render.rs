use essay_graphics_api::{
    driver::{RenderErr, Renderer}, Bounds, Canvas, Clip, FontStyle, FontTypeId, ImageId, Path, PathOpt, Point, TextStyle, TextureId
};
use essay_tensor::Tensor;

use super::canvas::PlotCanvas;

pub struct PlotRenderer<'a> {
    figure: &'a mut PlotCanvas,
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
            figure,
            queue,
            view,
        }
    }

    pub fn flush_inner(&mut self, clip: &Clip) {
        if let Some(queue) = self.queue {
            if let Some(view) = self.view {
                let mut encoder =
                   self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                let scissor = self.figure.to_scissor(clip);

                self.figure.image_render.flush(queue, view, &mut encoder);
                self.figure.triangle_render.flush(self.device, queue, view, &mut encoder, clip);
                // TODO: order issues with bezier and shape2d
                self.figure.shape2d_render.flush(self.device, queue, view, &mut encoder, scissor);
                self.figure.bezier_render.flush(self.device, queue, view, &mut encoder, scissor);
                self.figure.shape2d_texture_render.flush(self.device, queue, view, &mut encoder, scissor);
                self.figure.text_render.flush(queue, view, &mut encoder);

                self.figure.triangle3d_render.flush(self.device, queue, view, &mut encoder, clip);
        
                queue.submit(Some(encoder.finish()));
            }
        }
    }

}

impl Renderer for PlotRenderer<'_> {
    fn get_canvas(&self) -> &Canvas {
        self.figure.get_canvas()
    }

    fn to_px(&self, size: f32) -> f32 {
        self.figure.to_px(size)
    }

    fn draw_path(
        &mut self, 
        path: &Path<Canvas>, 
        style: &dyn PathOpt, 
        clip: &Clip,
    ) -> Result<(), RenderErr> {
        self.figure.draw_path(path, style, clip)
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
        self.figure.draw_markers(marker, xy, scale, color, style, clip)
    }

    fn font(
        &mut self,
        style: &FontStyle
    ) -> Result<FontTypeId, RenderErr> {
        self.figure.font(style)
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
        self.figure.draw_text(xy, text, angle, style, text_style, clip)
    }

    fn draw_triangles(
        &mut self,
        vertices: Tensor<f32>,  // Nx2 x,y in canvas coordinates
        colors: Tensor<u32>,    // N in rgba
        triangles: Tensor<u32>, // Mx3 vertex indices
        clip: &Clip,
    ) -> Result<(), RenderErr> {
        self.figure.draw_triangles(vertices, colors, triangles, clip)
    }

    fn draw_3d(
        &mut self,
        vertices: Tensor<f32>,  // Nx3 x,y in canvas coordinates
        colors: Tensor<u32>,    // N in rgba
        triangles: Tensor<u32>, // Mx3 vertex indices
        clip: &Clip,
    ) -> Result<(), RenderErr> {
        self.figure.draw_3d(vertices, colors, triangles, clip)
    }

    fn request_redraw(
        &mut self,
        bounds: &Bounds<Canvas>
    ) {
        self.figure.request_redraw(bounds)
    }

    fn draw_image(
        &mut self,
        bounds: &Bounds<Canvas>,
        colors: &Tensor<u8>,
        clip: &Clip
    ) -> Result<(), RenderErr> {
        let image = self.figure.create_image(self.device, colors);

        self.figure.draw_image_ref(self.device, bounds, image, clip)
    }

    fn create_image(
        &mut self,
        colors: &Tensor<u8>, // [rows, cols, 4]
    ) -> ImageId {
        self.figure.create_image(self.device, colors)
    }

    fn create_texture(
        &mut self,
        colors: &Tensor<u8>, // [rows, cols, 4]
    ) -> TextureId {
        self.figure.create_texture(colors)
    }

    fn draw_image_ref(
        &mut self,
        bounds: &Bounds<Canvas>,
        image: ImageId,
        clip: &Clip
    ) -> Result<(), RenderErr> {
        self.figure.draw_image_ref(self.device, bounds, image, clip)
    }

    fn flush(
        &mut self,
        clip: &Clip
    ) {
        self.flush_inner(clip);
    }
}
