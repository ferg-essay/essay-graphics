use std::mem;

use essay_graphics_api::{
    form::{Form, FormId, Matrix4, Shape, ShapeId}, 
    input::Input,
    renderer::{self, Canvas, RenderErr, Renderer, Result}, 
    Affine2d, Bounds, FontStyle, FontTypeId, ImageId, Path, PathOpt, 
    Point, Size, TextStyle, TextureId
};
use essay_tensor::tensor::Tensor;

use super::canvas::PlotCanvas;

pub(super) struct RenderWgpu<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub view: &'a wgpu::TextureView,

    pub encoder: Option<wgpu::CommandEncoder>,

    pub scissor: Option<(u32, u32, u32, u32)>,
    pub state: State,
    pub commands: Vec<wgpu::CommandBuffer>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    PreInit,
    Initialized
}

impl<'a> RenderWgpu<'a> {
    pub fn init(&mut self) {
        if self.state == State::PreInit {
            self.state = State::Initialized;

            self.clear_screen(self.view);
        }
    }

    pub fn init_encoder(&mut self) {
        if self.encoder.is_none() {
            self.encoder = Some(
                self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
            );

            self.init();
        }

    }

    pub fn render_pass<'b>(
        &'b mut self,
        draw: impl FnOnce(&mut wgpu::RenderPass<'b>) + 'b
    ) {
        self.init();

        self.init_encoder();
        if let Some(encoder) = &mut self.encoder {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: self.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
        });

        if let Some(scissor) = self.scissor {
            println!("Scissor {:?}", scissor);
            rpass.set_scissor_rect(scissor.0, scissor.1, scissor.2, scissor.3);
        }
    
        (draw)(&mut rpass);
        }

        // self.queue.submit(self.encoder..finish());
    }

    fn clear_screen(&mut self, view: &wgpu::TextureView) {
        //if let Some(encoder) = self.get_encoder() {
        self.get_encoder().begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                }
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
    //    }
    }

    fn get_encoder(&mut self) -> &mut wgpu::CommandEncoder {
        self.init_encoder();

        self.encoder.as_mut().unwrap()
    }

    fn flush(&mut self) {
        if let Some(encoder) = self.encoder.take() {
            self.queue.submit(Some(encoder.finish()));
            // self.commands.push(encoder.finish());
            // self.queue.submit(Some(encoder.finish()));
        }

        // self.encoder = Some(encoder);
    
    }

    fn close(&mut self) {
        self.flush();
        /*
        if let Some(encoder) = self.encoder.take() {
            self.commands.push(encoder.finish());
        }

        self.queue.submit(self.commands.drain(..));
        */
    }
}

pub(super) fn wgpu_rpass<'a: 'b, 'b, R>(
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    view: &'a wgpu::TextureView,
    // scissor: Option<(u32, u32, u32, u32)>,
    draw: impl FnOnce(&mut RenderWgpu<'a>) -> renderer::Result<R> + 'b
) -> renderer::Result<R> {
    //let encoder =
    //    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    
    let mut wgpu = RenderWgpu {
        device,
        queue,
        view,
        scissor: None,
        encoder: None,
        state: State::PreInit,
        commands: Vec::new(),
    };

    let result = (draw)(&mut wgpu);

    //if let Some(encoder) = wgpu.encoder.take() {
    //    wgpu.commands.push(encoder.finish());
    //}

    //if wgpu.state != State::PreInit {
        //println!("SubMit");
    //}

    //if wgpu.commands.len() > 0 {
    //    wgpu.queue.submit(wgpu.commands.drain(..));
    //}

    wgpu.close();

    result
}

pub(crate) fn render_draw<'a, R>(
    canvas: &'a mut PlotCanvas,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    view: Option<&'a wgpu::TextureView>,
    is_flush: bool,
    draw: impl FnOnce(&mut dyn Renderer) -> Result<R> + 'a
) -> Result<R> {
    let result = render_draw_inner(canvas, device, queue, view, is_flush, draw);

    canvas.input_mut().update_after_draw();

    result
}

 pub(super) fn render_draw_inner<'a, R>(
        canvas: &'a mut PlotCanvas,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        view: Option<&'a wgpu::TextureView>,
        is_flush: bool,
        draw: impl FnOnce(&mut dyn Renderer) -> Result<R> + 'a
    ) -> Result<R> {
    if let Some(view) = view {
        wgpu_rpass(device, queue, view, |wgpu: &mut RenderWgpu<'a>| {
            let pos = canvas.bounds().clone();

            let mut ui = PlotRenderer {
                canvas,
                device,
                queue: Some(queue),
                // view: Some(view),
                pos,
                wgpu: Some(wgpu),
            };

            let result = (draw)(&mut ui);

            if is_flush {
                ui.flush();
            }

            result
        })
    } else {
        let pos = canvas.bounds();

        let mut ui = PlotRenderer {
            device,
            canvas,
            queue: Some(queue),
            // view: None,
            pos,
            wgpu: None,
        };

        let result = (draw)(&mut ui);

        if is_flush {
            ui.flush();
        }

        result
    }
}


pub struct PlotRenderer<'a, 'b> {
    canvas: &'a mut PlotCanvas,
    device: &'a wgpu::Device,
    queue: Option<&'a wgpu::Queue>,
    // view: Option<&'a wgpu::TextureView>,

    wgpu: Option<&'b mut RenderWgpu<'a>>,

    pos: Bounds<Canvas>,
}

impl<'a, 'b> PlotRenderer<'a, 'b> {
    /*
    pub(crate) fn _new(
        canvas: &'a mut PlotCanvas,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        view: Option<&'a wgpu::TextureView>,
    ) -> Self {
        let pos = canvas.bounds().clone();

        Self {
            device,
            canvas,
            queue: Some(queue),
            view,
            pos,
        }
    }
    */
    fn flush_inner(&mut self) {
        if let Some(wgpu) = self.wgpu.as_mut() {
            self.canvas.image_render.flush(wgpu);
            self.canvas.triangle_render.flush(wgpu);
            self.canvas.shape2d_render.flush(wgpu);
            // TODO: order issues with bezier and shape2d
            self.canvas.bezier_render.flush(wgpu);
            self.canvas.shape2d_texture_render.flush(wgpu);
            self.canvas.text_render.flush(wgpu);
            self.canvas.form3d_render.flush(
                wgpu,
                &self.canvas.texture_store, 
            );
            self.canvas.shape2d_tex2_render.flush(
                wgpu,
                &self.canvas.texture_store, 
            );

            wgpu.flush();
        }
        /*
        if let Some(queue) = self.queue {
            if let Some(texture) = self.view {
                // self.get_scissor()
                wgpu_rpass(self.device, queue, texture, |wgpu| {
                    self.canvas.image_render.flush(wgpu);
                    self.canvas.triangle_render.flush(wgpu);
                    self.canvas.shape2d_render.flush(wgpu);
                    // TODO: order issues with bezier and shape2d
                    self.canvas.bezier_render.flush(wgpu);
                    self.canvas.shape2d_texture_render.flush(wgpu);
                    self.canvas.text_render.flush(wgpu);
                    self.canvas.form3d_render.flush(
                        wgpu,
                        &self.canvas.texture_store, 
                    );
                    self.canvas.shape2d_tex2_render.flush(
                        wgpu,
                        &self.canvas.texture_store, 
                    );
    
                    Ok(())
                }).unwrap();
            }
        }
        */
    }

    fn get_scissor(&self) -> Option<(u32, u32, u32, u32)> {
        let pos = &self.pos;

        Some((
            pos.xmin() as u32, 
            (self.canvas.bounds().ymax() - pos.ymax()) as u32, 
            // pos.ymin() as u32, 
            (pos.width()) as u32, 
            (pos.height()) as u32
        ))
    }
}

impl<'a, 'b> Renderer for PlotRenderer<'a, 'b> {
    fn extent(&self) -> Bounds<Canvas> {
        self.canvas.bounds()
    }

    fn pos(&self) -> Bounds<Canvas> {
        self.pos
    }

    fn scale_factor(&self) -> f32 {
        self.canvas.scale_factor()
    }

    fn to_px(&self, size: f32) -> f32 {
        self.canvas.to_px(size)
    }

    fn input(&self) -> &Input {
        &self.canvas.input()
    }

    fn draw_path(
        &mut self, 
        path: &Path<Canvas>, 
        style: &dyn PathOpt, 
    ) -> Result<(), RenderErr> {
        self.canvas.draw_path(path, style)?;
        
        Ok(())

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

    fn text_size(
        &mut self, 
        text: &str,
        text_style: &TextStyle,
    ) -> Size {
        self.canvas.text_size(text, text_style)
    }

    fn draw_triangles(
        &mut self,
        vertices: &Tensor<f32>,  // Nx2 x,y in canvas coordinates
        colors: &Tensor<u32>,    // N in rgba
        triangles: &Tensor<u32>, // Mx3 vertex indices
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
        _bounds: Bounds<Canvas>
    ) {
        self.canvas.request_redraw(true)
    }

    fn draw_image(
        &mut self,
        bounds: Bounds<Canvas>,
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
        bounds: Bounds<Canvas>,
        image: ImageId,
    ) -> Result<(), RenderErr> {
        self.canvas.draw_image_ref(self.device, bounds, image)
    }

    fn flush(
        &mut self,
    ) {
        self.flush_inner();
    }

    fn draw_with<'c>(
        &mut self, 
        pos: Bounds<Canvas>, 
        f: Box<dyn FnOnce(&mut dyn Renderer) -> Result<()> + 'c>
    ) -> Result<()> {
        let push = Push::new(self, pos);

        (f)(push.ptr)?;

        Ok(())
    }

    fn draw_with_clip<'c>(
        &mut self, 
        pos: Bounds<Canvas>, 
        f: Box<dyn FnOnce(&mut dyn Renderer) -> Result<()> + 'c>
    ) -> Result<()> {
        self.flush();

        let push = Push::new_clip(self, pos);

        (f)(push.ptr)?;

        //push.ptr.flush_inner(&push.clip);
        push.ptr.flush_inner();

        Ok(())
    }
}

struct Push<'a, 'b, 'c> {
    ptr: &'a mut PlotRenderer<'b, 'c>,

    pos: Bounds<Canvas>,
}

impl<'a, 'b, 'c> Push<'a, 'b, 'c> {
    fn new(renderer: &'a mut PlotRenderer<'b, 'c>, pos: Bounds<Canvas>) -> Self {
        let mut push = Self {
            ptr: renderer,
            pos,
        };

        mem::swap(&mut push.pos, &mut push.ptr.pos);

        push
    }

    fn new_clip(renderer: &'a mut PlotRenderer<'b, 'c>, pos: Bounds<Canvas>) -> Self {
        let mut push = Self {
            ptr: renderer,
            pos,
        };

        mem::swap(&mut push.pos, &mut push.ptr.pos);

        push
    }
} 

impl Drop for Push<'_, '_, '_> {
    fn drop(&mut self) {
        mem::swap(&mut self.pos, &mut self.ptr.pos);
    }
}
/*
impl Drop for PlotRenderer<'_, '_> {
    fn drop(&mut self) {
        self.flush_inner();
    }
}
    */
