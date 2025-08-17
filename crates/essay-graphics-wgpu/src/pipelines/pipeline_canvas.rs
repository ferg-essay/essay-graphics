use std::collections::HashMap;

use essay_graphics_api::{
    form::{Form, FormId, Matrix4}, 
    input::Input, path_style::MeshStyle, 
    renderer::{Canvas, RenderErr, Result}, 
    Affine2d, BezierMesh2d, Bounds, Clip, Color, FontStyle, FontTypeId, 
    Hatch, HorizAlign, Mesh2d, Mesh2dColor, 
    PathOpt, Point, Size, TextStyle, TextureId, VertAlign
};
use essay_tensor::tensor::Tensor;
use wgpu::util::StagingBelt;

use crate::render::{hatch::init_hatch, render::{RenderWgpu}};
use super::{
    bezier_mesh::BezierMeshRender, form3d::Form3dRender,
    mesh2d::Mesh2dRender, mesh2d_color::Mesh2dColorRender, 
    text::TextRender, text_cache::FontId, 
    texture_store::TextureCache,
};

pub struct PipelineCanvas {
    bounds: Bounds<Canvas>,
    scale_factor: f32,
    input: Input,

    mesh2d_render: Mesh2dRender,
    bezier_mesh_render: BezierMeshRender,

    mesh2d_color_render: Mesh2dColorRender,

    text_render: TextRender,

    form3d_render: Form3dRender,

    texture_store: TextureCache,
    hatch_map: HashMap<Hatch, TextureId>,

    staging: Option<StagingBelt>,

    font_id_default: FontId,

    // to_gpu: Affine2d,

    cache_size: Size,
    is_request_redraw: bool,
}

impl PipelineCanvas {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
        scale_factor: f32,
    ) -> Self {
        let mesh2d_render = Mesh2dRender::new(device, format);
        let bezier_mesh_render = BezierMeshRender::new(device, format);

        let mesh2d_color_render = Mesh2dColorRender::new(device, format);

        let mut text_render = TextRender::new(device, format, 512, 512);

        let font_id_default = text_render.font("default");

        let form3d_render = Form3dRender::new(device, format, width, height);

        let staging = StagingBelt::new(2048 * 128);

        let mut texture_store = TextureCache::new(device, queue);

        let hatch_map = init_hatch(device, queue, &mut texture_store);

        let mut canvas = Self {
            bounds: Bounds::from([width as f32, height as f32]),

            cache_size: Size::default(),
            input: Input::default(),
            scale_factor: 4. / 3. * scale_factor,

            mesh2d_render,
            bezier_mesh_render,
            mesh2d_color_render,
            form3d_render,
            text_render,

            font_id_default,
            texture_store,
            hatch_map,

            staging: Some(staging),
            // to_gpu: Affine2d::eye(),

            is_request_redraw: false,
        };

        // canvas.input.size = Size(width as f32, height as f32);
        canvas.resize(&device);

        canvas
    }

    pub fn request_redraw(&mut self, is_redraw: bool) {
        self.is_request_redraw = is_redraw;
    }
    
    pub fn clear(&mut self) {
    }

    pub fn resize(&mut self, device: &wgpu::Device) -> bool {
        if self.cache_size == self.input.size || self.input.size.width() == 0. {
            return false;
        }

        /*
        if self.input.size.width() > 0. {
            self.form3d_render.resize(device, self.cache_size.width() as u32, self.cache_size.height() as u32);
        }
        */

        true
    }

    pub fn set_viewport(&self, pass: &mut wgpu::RenderPass) {
        pass.set_viewport(
            self.bounds.xmin(), 
            self.bounds.ymin(), 
            self.bounds.xmax(), 
            self.bounds.ymax(),
            -1., 1.
        );
    }

    pub fn to_scissor(&self, clip: &Clip) -> Option<(u32, u32, u32, u32)> {
        match clip {
            Clip::None => None,
            Clip::Bounds(p0, p1) => {
                Some((
                    p0.0 as u32, 
                    p1.0 as u32,
                    (p1.0 - p0.0) as u32, 
                    (p1.1 - p1.0) as u32
                ))
            }
        }
    }

    ///
    /// Returns the boundary of the canvas in pixels
    ///
    pub fn bounds(&self) -> Bounds<Canvas> {
        self.bounds
    }

    #[inline]
    pub fn to_px(&self, size: f32) -> f32 {
        self.scale_factor * size
    }

    pub(crate) fn draw_bezier_mesh(
        &mut self, 
        wgpu: &mut RenderWgpu,
        mesh: &BezierMesh2d, 
        _texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<(), RenderErr> {
        self.bezier_mesh_render.draw(wgpu, mesh, style);

        Ok(())
    }

    pub(crate) fn draw_mesh2d(
        &mut self, 
        wgpu: &mut RenderWgpu,
        mesh: &Mesh2d,
        texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<(), RenderErr> {
        self.mesh2d_render.draw(
            wgpu, 
            &self.texture_store, 
            mesh, 
            texture,
            style,
        );

        Ok(())
    }

    pub(crate) fn draw_mesh2d_color(
        &mut self, 
        wgpu: &mut RenderWgpu,
        mesh: &Mesh2dColor,
        affine: &Affine2d,
    ) -> Result<(), RenderErr> {
        self.mesh2d_color_render.draw(
            wgpu, 
            mesh, 
            affine,
        );

        Ok(())
    }

    pub fn font(
        &mut self,
        style: &FontStyle,
    ) -> Result<FontTypeId, RenderErr> {
        if let Some(family) = style.get_family() {
            let font_id = self.text_render.font(family);

            Ok(FontTypeId(font_id.0)) // i()))
        } else {
            Err(RenderErr::NotImplemented)            
        }
    }

    pub fn draw_text(
        &mut self,
        xy: Point, // location in Canvas coordinates
        text: &str,
        angle: f32,
        style: &dyn PathOpt, 
        text_style: &TextStyle,
    ) -> Result<(), RenderErr> {
        if text.len() == 0 { // todo: more sophisticated validation
            return Ok(());
        }

        let color = match style.get_face_color() {
            Some(color) => color,
            None => Color(0x000000ff),
        };

        let size = match &text_style.get_size() {
            Some(size) => *size,
            None => 10.,
        };

        let size = self.to_px(size);

        let halign = match text_style.get_width_align() {
            Some(align) => align.clone(),
            None => HorizAlign::Center,
        };

        let valign = match text_style.get_height_align() {
            Some(align) => align.clone(),
            None => VertAlign::Bottom,
        };

        let font_id = match text_style.get_font() {
            Some(type_id) => FontId(type_id.0),
            None => self.font_id_default,
        };
        // let font_id = self.text_render.font("sans-serif");

        self.text_render.draw(
            text,
            font_id,
            size,
            xy, 
            Point(self.bounds.width(), self.bounds.height()),
            color,
            angle,
            halign,
            valign,
        );
 
        Ok(())
    }

    pub fn text_size(
        &mut self,
        text: &str,
        text_style: &TextStyle,
    ) -> Size {
        let size = text_style.get_size().map_or(10., |s| s);
        let size = self.to_px(size);

        let font_id = text_style.get_font().map_or(
            self.font_id_default,
            |id| FontId(id.0)
        );

        self.text_render.text_size(
            text,
            font_id,
            size
        )
    }

    pub fn create_form(
        &mut self,
        form: &Form,
    ) -> FormId {
        self.form3d_render.create_form(form)
    }

    pub fn draw_form(
        &mut self,
        form: FormId,
        camera: &Matrix4,
    ) -> Result<(), RenderErr> {
        self.form3d_render.camera(camera);
        self.form3d_render.draw_form(form);
        
        Ok(())
    }

    pub fn create_texture(&mut self, image: &Tensor<u8>) -> TextureId {
        assert!(image.rank() == 2, "colors rank must be 2 shape={:?}", image.shape().as_vec());

        //self.shape2d_render.add_texture(image.rows(), image.cols(), image.as_slice())
        todo!();
    }

    pub fn create_texture_rgba8(
        &mut self, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        image: &Tensor<u8>
    ) -> TextureId {
        assert!(image.rank() == 3, "texture requires rank 3 shape={:?}", image.shape().as_vec());
        assert!(image.cols() == 4, "texture requires 4 columns shape={:?}", image.shape().as_vec());
    
        self.texture_store.add_rgba_u8(
            device, 
            queue, 
            image.dim(1) as u32, 
            image.dim(0) as u32, 
            image.as_slice()
        )
    }

    pub(crate) fn flush(&mut self, wgpu: &mut RenderWgpu) {
        self.bezier_mesh_render.flush(wgpu);
        self.mesh2d_render.flush(wgpu, &self.texture_store);
        self.mesh2d_color_render.flush(wgpu);
        self.text_render.flush(wgpu);
        self.form3d_render.flush(wgpu, &self.texture_store);
     }
    
    pub(crate) fn take_staging(&mut self) -> wgpu::util::StagingBelt {
        self.staging.take().unwrap()
    }
    
    pub(crate) fn replace_staging(&mut self, staging: wgpu::util::StagingBelt) {
        assert!(self.staging.is_none());

        self.staging.replace(staging);
    }
}
