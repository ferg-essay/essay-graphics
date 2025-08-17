use essay_graphics_api::{
    form::{Form, FormId, Matrix4}, 
    path_style::MeshStyle, 
    renderer::{RenderErr, Result}, 
    Affine2d, BezierMesh2d, 
    Mesh2d, Mesh2dColor, 
    TextureId,
};
use essay_tensor::tensor::Tensor;

use crate::render::{render::{RenderWgpu}};
use super::{
    bezier_mesh::BezierMeshRender, form3d::Form3dRender,
    mesh2d::Mesh2dRender, mesh2d_color::Mesh2dColorRender, 
    texture_store::TextureStore,
};

pub(crate) struct PipelineCanvas {
    mesh2d_render: Mesh2dRender,
    bezier_mesh_render: BezierMeshRender,

    mesh2d_color_render: Mesh2dColorRender,

    form3d_render: Form3dRender,

    texture_store: TextureStore,

    is_request_redraw: bool,
}

impl PipelineCanvas {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
    ) -> Self {
        let mesh2d_render = Mesh2dRender::new(device, format);
        let bezier_mesh_render = BezierMeshRender::new(device, format);

        let mesh2d_color_render = Mesh2dColorRender::new(device, format);

        let form3d_render = Form3dRender::new(device, format, width, height);

        let texture_store = TextureStore::new(device, queue);

        // let hatch_map = init_hatch(device, queue, &mut texture_store);

        let mut canvas = Self {
            mesh2d_render,
            bezier_mesh_render,
            mesh2d_color_render,
            form3d_render,

            texture_store,

            is_request_redraw: false,
        };

        canvas.resize(&device);

        canvas
    }

    pub fn request_redraw(&mut self, is_redraw: bool) {
        self.is_request_redraw = is_redraw;
    }
    
    pub fn clear(&mut self) {
    }

    pub fn textures_mut(&mut self) -> &mut TextureStore {
        &mut self.texture_store
    }

    pub fn resize(&mut self, device: &wgpu::Device) -> bool {
        /*
        if self.cache_size == self.input.size || self.input.size.width() == 0. {
            return false;
        }
        */

        /*
        if self.input.size.width() > 0. {
            self.form3d_render.resize(device, self.cache_size.width() as u32, self.cache_size.height() as u32);
        }
        */

        true
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
        self.form3d_render.flush(wgpu, &self.texture_store);
     }
}
