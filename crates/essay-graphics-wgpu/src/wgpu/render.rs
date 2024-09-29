use std::mem;

use essay_graphics_api::{
    form::{Form, FormId, Matrix4, Shape, ShapeId}, renderer::{Canvas, Drawable, RenderErr, Renderer, Result}, Affine2d, Bounds, FontStyle, FontTypeId, ImageId, Path, PathOpt, Point, TextStyle, TextureId
};
use essay_tensor::Tensor;

use super::canvas::PlotCanvas;

pub struct PlotRenderer<'a> {
    canvas: &'a mut PlotCanvas,
    device: &'a wgpu::Device,
    queue: Option<&'a wgpu::Queue>,
    view: Option<&'a wgpu::TextureView>,

    pos: Bounds<Canvas>,
}

impl<'a> PlotRenderer<'a> {
    pub(crate) fn new(
        canvas: &'a mut PlotCanvas,
        device: &'a wgpu::Device,
        queue: Option<&'a wgpu::Queue>,
        view: Option<&'a wgpu::TextureView>,
    ) -> Self {
        let pos = canvas.bounds().clone();

        Self {
            device,
            canvas,
            queue,
            view,
            pos,
        }
    }

    fn flush_inner(&mut self) {
        if let Some(queue) = self.queue {
            if let Some(view) = self.view {
                let mut encoder =
                   self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                //let scissor = self.canvas.to_scissor(clip);
                let scissor = self.get_scissor();

                self.canvas.image_render.flush(queue, view, &mut encoder);
                self.canvas.triangle_render.flush(self.device, queue, view, &mut encoder, scissor);
                // TODO: order issues with bezier and shape2d
                self.canvas.shape2d_render.flush(self.device, queue, view, &mut encoder, scissor);
                self.canvas.bezier_render.flush(self.device, queue, view, &mut encoder, scissor);
                self.canvas.shape2d_texture_render.flush(self.device, queue, view, &mut encoder, scissor);
                self.canvas.text_render.flush(queue, view, &mut encoder);

                self.canvas.form3d_render.flush(
                    self.device, 
                    queue, 
                    view, 
                    &mut encoder, 
                    &self.canvas.texture_store, 
                    scissor
                );
                self.canvas.shape2d_tex2_render.flush(
                    self.device, 
                    queue, 
                    view, 
                    &mut encoder, 
                    &self.canvas.texture_store, 
                    scissor
                );
                
                queue.submit(Some(encoder.finish()));
            }
        }
    }

    fn get_scissor(&self) -> Option<(u32, u32, u32, u32)> {
        let pos = &self.pos;

        /*
        Some((
            pos.xmin() as u32, 
            (self.canvas.bounds().height() - pos.ymin()) as u32, 
            (pos.width() - 1.) as u32, 
            (pos.height() - 1.) as u32
        ))
        */
        Some((
            pos.xmin() as u32, 
            (self.canvas.bounds().ymax() - pos.ymax()) as u32, 
            // pos.ymin() as u32, 
            (pos.width()) as u32, 
            (pos.height()) as u32
        ))
    }
}

impl<'a> Renderer for PlotRenderer<'a> {
    fn extent(&self) -> &Bounds<Canvas> {
        self.canvas.bounds()
    }

    fn pos(&self) -> &Bounds<Canvas> {
        &self.pos
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
    ) -> Result<(), RenderErr> {
        self.canvas.draw_path(path, style)
    }

    fn draw_markers(
        &mut self, 
        marker: &Path<Canvas>, 
        xy: &Tensor,
        scale: &Tensor,
        color: &Tensor<u32>,
        style: &dyn PathOpt, 
    ) -> Result<(), RenderErr> {
        self.canvas.draw_markers(marker, xy, scale, color, style)
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
    ) -> Result<(), RenderErr> {
        self.canvas.draw_text(xy, text, angle, style, text_style)
    }

    fn draw_triangles(
        &mut self,
        vertices: Tensor<f32>,  // Nx2 x,y in canvas coordinates
        colors: Tensor<u32>,    // N in rgba
        triangles: Tensor<u32>, // Mx3 vertex indices
    ) -> Result<(), RenderErr> {
        self.canvas.draw_triangles(vertices, colors, triangles)
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
    ) -> Result<(), RenderErr> {
        self.canvas.draw_form(form, camera)
    }

    fn create_shape(
        &mut self,
        shape: &Shape,
    ) -> ShapeId {
        self.canvas.create_shape(shape)
    }

    fn draw_shape(
        &mut self,
        shape: ShapeId,
        camera: &Affine2d,
    ) -> Result<(), RenderErr> {
        self.canvas.draw_shape(shape, camera)
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
    ) -> Result<(), RenderErr> {
        let image = self.canvas.create_image(self.device, colors);

        self.canvas.draw_image_ref(self.device, bounds, image)
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
    ) -> Result<(), RenderErr> {
        self.canvas.draw_image_ref(self.device, bounds, image)
    }

    fn flush(
        &mut self,
    ) {
        self.flush_inner();
    }

    fn draw_with(
        &mut self, 
        pos: &Bounds<Canvas>, 
        drawable: &mut dyn Drawable
    ) -> Result<()> {
        let push = Push::new(self, pos);

        drawable.draw(push.ptr)?;

        //push.ptr.flush_inner(&push.clip);
        push.ptr.flush_inner();

        Ok(())
    }
}

struct Push<'a, 'b> {
    ptr: &'a mut PlotRenderer<'b>,

    pos: Bounds<Canvas>,
}

impl<'a, 'b> Push<'a, 'b> {
    fn new(renderer: &'a mut PlotRenderer<'b>, pos: &Bounds<Canvas>) -> Self {
        let mut push = Self {
            ptr: renderer,
            pos: pos.clone(),
        };

        mem::swap(&mut push.pos, &mut push.ptr.pos);

        push
    }
} 

impl Drop for Push<'_, '_> {
    fn drop(&mut self) {
        mem::swap(&mut self.pos, &mut self.ptr.pos);
    }
}

impl Drop for PlotRenderer<'_> {
    fn drop(&mut self) {
        self.flush_inner();
    }
}
