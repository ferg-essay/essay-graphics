use std::{mem, num::NonZero};

use essay_graphics_api::{
    form::{Form, FormId, Matrix4}, input::Input, path_style::MeshStyle, 
    renderer::{self, Canvas, RenderErr, Renderer, Result}, BezierMesh2d, Bounds, 
    FontStyle, FontTypeId, Mesh2d, Mesh2dColor, Path, PathOpt, Point, Size, 
    TextStyle, TextureId
};
use essay_tensor::tensor::Tensor;
use wgpu::util::StagingBelt;

use super::canvas::PlotCanvas;

pub(super) struct RenderWgpu<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub view: &'a wgpu::TextureView,

    pub encoder: Option<wgpu::CommandEncoder>,
    pub staging: StagingBelt,

    pub scissor: Option<(u32, u32, u32, u32)>,
    pub state: State,
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

    pub fn init_encoder(&mut self) -> &mut wgpu::CommandEncoder {
        if self.encoder.is_none() {
            self.encoder = Some(
                self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
            );

            self.staging.recall();

            self.init();
        };

        self.encoder.as_mut().unwrap()
    }

    pub fn write_buffer(&mut self, target: &wgpu::Buffer, data: &[u8]) {
        let len = NonZero::new(data.len() as u64).unwrap();

        self.init_encoder();

        if let Some(encoder) = &mut self.encoder {
            self.staging.write_buffer(
                encoder,
                target,
                0,
                len,
                self.device,
            ).copy_from_slice(data)
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
                rpass.set_scissor_rect(scissor.0, scissor.1, scissor.2, scissor.3);
            }
    
            (draw)(&mut rpass);
        }
    }

    fn clear_screen(&mut self, view: &wgpu::TextureView) {
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
            self.staging.finish();
            self.queue.submit(Some(encoder.finish()));
        }
    }

    fn close(&mut self) {
        self.flush();
    }
}

pub(super) fn wgpu_rpass<'a: 'b, 'b, R>(
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    view: &'a wgpu::TextureView,
    staging: StagingBelt,
    draw: impl FnOnce(&mut RenderWgpu<'a>) -> renderer::Result<R> + 'b
) -> (renderer::Result<R>, StagingBelt) {
    let mut wgpu = RenderWgpu {
        device,
        queue,
        view,
        scissor: None,
        encoder: None,
        state: State::PreInit,
        staging,
    };

    let result = (draw)(&mut wgpu);

    wgpu.close();

    (result, wgpu.staging)
}

pub(crate) fn render_draw<'a, R>(
    canvas: &'a mut PlotCanvas,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    view: Option<&'a wgpu::TextureView>,
    is_flush: bool,
    draw: impl FnOnce(&mut dyn Renderer) -> Result<R> + 'a
) -> Result<R> {
    let staging = canvas.take_staging();

    let (result, staging) = render_draw_inner(canvas, device, queue, view, staging, is_flush, draw);

    canvas.replace_staging(staging);

    // canvas.input_mut().update_after_draw();

    result
}

 pub(super) fn render_draw_inner<'a, R>(
        canvas: &'a mut PlotCanvas,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        view: Option<&'a wgpu::TextureView>,
        staging: StagingBelt,
        is_flush: bool,
        draw: impl FnOnce(&mut dyn Renderer) -> Result<R> + 'a
    ) -> (Result<R>, StagingBelt) {
    if let Some(view) = view {
        wgpu_rpass(device, queue, view, staging,
            |wgpu: &mut RenderWgpu<'a>| {
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
            pos,
            wgpu: None,
        };

        let result = (draw)(&mut ui);

        if is_flush {
            ui.flush();
        }

        (result, staging)
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
    fn flush_inner(&mut self) {
        if let Some(wgpu) = self.wgpu.as_mut() {
            self.canvas.flush(wgpu);

            wgpu.flush();
        }
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

    fn update_scissor(&mut self) {
        let scissor = self.get_scissor();

        if let Some(wgpu) = &mut self.wgpu {
            (*wgpu).scissor = scissor;
        }
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
        if let Some(wgpu) = self.wgpu.as_mut() {
            self.canvas.draw_path(wgpu, path, style)?;
        }

        Ok(())
    }
    
    fn draw_bezier_mesh(
        &mut self,
        mesh: &BezierMesh2d,
        texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<()> {
        if let Some(wgpu) = self.wgpu.as_mut() {
            self.canvas.draw_bezier_mesh(wgpu, mesh, texture, style)?;
        }

        Ok(())
    }
    
    fn draw_mesh2d(
        &mut self,
        mesh: &Mesh2d,
        texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<()> {
        if let Some(wgpu) = self.wgpu.as_mut() {
            self.canvas.draw_mesh2d(wgpu, mesh, texture, style)?;
        }

        Ok(())
    }
    
    fn draw_mesh2d_color(
        &mut self,
        mesh: &Mesh2dColor,
    ) -> Result<()> {
        if let Some(wgpu) = self.wgpu.as_mut() {
            self.canvas.draw_mesh2d_color(wgpu, mesh)?;
        }

        Ok(())
    }

    fn draw_markers(
        &mut self, 
        marker: &Path<Canvas>, 
        path_style: &dyn PathOpt, 
        marker_style: &[MeshStyle],
    ) -> Result<(), RenderErr> {
        if let Some(wgpu) = self.wgpu.as_mut() {
            self.canvas.draw_markers(wgpu, marker, path_style, marker_style)?;
        }

        Ok(())
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

    fn request_redraw(
        &mut self,
        _bounds: Bounds<Canvas>
    ) {
        self.canvas.request_redraw(true)
    }

    fn create_texture_rgba8(
        &mut self,
        colors: &Tensor<u8>, // [rows, cols, 4]
    ) -> TextureId {
        self.canvas.create_texture_rgba8(self.device, self.queue.unwrap(), colors)
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

        // renderer._get_scissor();

        mem::swap(&mut push.pos, &mut push.ptr.pos);
        push.ptr.update_scissor();

        push
    }

    fn new_clip(renderer: &'a mut PlotRenderer<'b, 'c>, pos: Bounds<Canvas>) -> Self {
        let mut push = Self {
            ptr: renderer,
            pos,
        };

        mem::swap(&mut push.pos, &mut push.ptr.pos);
        push.ptr.update_scissor();

        push
    }
} 

impl Drop for Push<'_, '_, '_> {
    fn drop(&mut self) {
        mem::swap(&mut self.pos, &mut self.ptr.pos);
        self.ptr.update_scissor();
    }
}
/*
impl Drop for PlotRenderer<'_, '_> {
    fn drop(&mut self) {
        self.flush_inner();
    }
}
    */
