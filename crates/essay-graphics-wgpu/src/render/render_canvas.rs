use essay_graphics_api::{
    input::Input, renderer::{Canvas, GraphicsContext, Pos, RenderErr}, Affine2d, Bounds, FontStyle, FontTypeId, Point, Size, TextureId};

use crate::{pipelines::PipelineCanvas, render::{context::WgpuGraphicsContext, render::RenderWgpu, text_cache::{FontId, TextCache}}};


pub struct RenderCanvas {
    bounds: Bounds<Canvas>,
    scale_factor: f32,
    input: Input,

    pub pipeline: PipelineCanvas,

    pub text_cache: TextCache,
    pub font_context: WgpuGraphicsContext,

    pub to_gpu: Affine2d,

    cache_pos: Pos,
    is_request_redraw: bool,
}

impl RenderCanvas {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
        scale_factor: f32,
    ) -> Self {
        // let hatch_map = init_hatch(device, queue, &mut texture_store);

        let mut pipeline = PipelineCanvas::new(device, queue, format, width, height);

        let text_cache = TextCache::new(device, pipeline.textures_mut(), 512, 512);

        let mut canvas = Self {
            bounds: Bounds::unit(),

            pipeline,

            text_cache,
            font_context: WgpuGraphicsContext::new(),

            cache_pos: Pos::unit(),
            input: Input::default(),
            scale_factor: 4. / 3. * scale_factor,

            to_gpu: Affine2d::eye(),

            is_request_redraw: false,
        };

        canvas.input.size = Size(width as f32, height as f32);
        // canvas.resize(&device);


        canvas
    }

    pub fn set_scale_factor(&mut self, scale_factor: f32) {
        self.scale_factor = 4. / 3. * scale_factor;
    }

    pub fn request_redraw(&mut self, is_redraw: bool) {
        self.is_request_redraw = is_redraw;
    }

    pub fn resize(&mut self, pos: Pos) -> bool {
        if self.cache_pos == pos || pos.width() == 0. {
            return false;
        }

        self.cache_pos = pos;
        self.request_redraw(true);
        self.bounds = Bounds::from(pos);

        let pos_gpu = Bounds::<Canvas>::new_flat(
            Point(-1., 1.),
            Point(1., -1.)
        );

        self.to_gpu = self.bounds.affine_to(&pos_gpu);

        //if self.input.size.width() > 0. {
        //    self.form3d_render.resize(device, self.cache_size.width() as u32, self.cache_size.height() as u32);
        //}

        true
    }

    pub fn pos(&self) -> Pos {
        self.bounds
    }

    pub fn font_texture_id(&self, _font: FontId, _size: f32) -> TextureId {
        self.text_cache.texture_id()
    }

    pub fn font(
        &mut self,
        style: &FontStyle,
    ) -> Result<FontTypeId, RenderErr> {
        if let Some(family) = style.get_family() {
            let font_id = self.text_cache.font_id(family);

            Ok(FontTypeId(font_id.0)) // i()))
        } else {
            Err(RenderErr::NotImplemented)            
        }
    }

    pub fn flush(
        &mut self,
        wgpu: &RenderWgpu,
    ) {
        self.text_cache.flush(wgpu.queue, self.pipeline.textures_mut());
    }
}
