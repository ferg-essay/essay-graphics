use essay_graphics_api::{
    form::{Form, FormId, Matrix4}, 
    path_style::MeshStyle, 
    renderer::{self, Canvas, RenderErr, Result}, 
    Affine2d, BezierMesh2d, Bounds, Color, Mesh2d, Mesh2dColor, 
    Shapes, Size, TextureId
};
use essay_tensor::tensor::Tensor;

use crate::{
    pipelines::{bezier_mesh::BezierFlush, mesh2d::Mesh2dFlush, mesh2d_color::Mesh2dColorItem, 
        shape_rect::{ShapeRectFlush, ShapeRectRender}
    }, 
    render::render::RenderWgpu
};
use super::{
    bezier_mesh::BezierMeshRender, form3d::Form3dRender,
    mesh2d::Mesh2dRender, mesh2d_color::Mesh2dColorRender, 
    texture_store::TextureStore,
};

pub(crate) struct PipelineCanvas {
    texture_store: TextureStore,

    mesh2d_render: Mesh2dRender,
    bezier_mesh_render: BezierMeshRender,
    mesh2d_color_render: Mesh2dColorRender,
    form3d_render: Form3dRender,

    shape_rect_render: ShapeRectRender,

    pos: Bounds<Canvas>,

    flush_items: Vec<FlushItem>,

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

        // let shape_rect_render = ShapeRectRender::new(device, format);

        let texture_store = TextureStore::new(device, queue);

        // let hatch_map = init_hatch(device, queue, &mut texture_store);

        let canvas = Self {
            texture_store,

            mesh2d_render,
            bezier_mesh_render,
            mesh2d_color_render,
            form3d_render,

            shape_rect_render: ShapeRectRender::new(device, format),

            pos: Bounds::from(Size::new(width as f32, height as f32)),

            flush_items: Vec::new(),

            is_request_redraw: false,
        };

        //canvas.resize(&device, width as f32, height as f32);

        canvas
    }

    pub fn request_redraw(&mut self, is_redraw: bool) {
        self.is_request_redraw = is_redraw;
    }
    
    pub fn clear(&mut self) {
        self.flush_items.clear();
        self.mesh2d_render.clear();
    }

    pub fn textures_mut(&mut self) -> &mut TextureStore {
        &mut self.texture_store
    }

    pub fn resize(&mut self, wgpu: &mut RenderWgpu, width: f32, height: f32) {
        if width > 0. {
            self.pos = Bounds::from([width, height]);
            self.form3d_render.resize(wgpu.device, width as u32, height as u32);

            self.shape_rect_render.resize(wgpu, self.pos);
        }
    }

    pub(crate) fn draw_bezier_mesh(
        &mut self, 
        wgpu: &mut RenderWgpu,
        mesh: &BezierMesh2d, 
        _texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<(), RenderErr> {
        let item = self.bezier_mesh_render.draw(wgpu, mesh, style);

        self.push_flush(item)
    }

    pub(crate) fn draw_mesh2d(
        &mut self, 
        wgpu: &mut RenderWgpu,
        mesh: &Mesh2d,
        texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<(), RenderErr> {
        let item = self.mesh2d_render.draw(wgpu, mesh, texture, style);

        self.push_flush(item)
    }

    pub(crate) fn draw_mesh2d_color(
        &mut self, 
        wgpu: &mut RenderWgpu,
        mesh: &Mesh2dColor,
        affine: &Affine2d,
    ) -> Result<(), RenderErr> {
        let item = self.mesh2d_color_render.draw(
            wgpu, 
            mesh, 
            affine,
        );

        self.push_flush(item)
    }

    pub(crate) fn draw_shape(
        &mut self, 
        wgpu: &mut RenderWgpu,
        shape: &Shapes,
        texture: TextureId,
    ) -> Result<(), RenderErr> {
        let item = match shape {
            Shapes::None => { FlushItem::None },
            Shapes::Rectangle(pos, size, r1, color) => {
                self.shape_rect_render.draw(wgpu, *pos, *size, *r1, texture, *color)
            }
        };

        self.push_flush(item)
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

    fn push_flush(&mut self, item: FlushItem) -> renderer::Result<()> {
        match item {
            FlushItem::None => {}
            _ => {
                self.flush_items.push(item);
            }
        }

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
        //self.bezier_mesh_render.flush(wgpu);
        //self.mesh2d_render.flush(wgpu, &self.texture_store);
        //self.mesh2d_color_render.flush(wgpu);
        self.form3d_render.flush(wgpu, &self.texture_store);

        wgpu.render_pass(|rpass| {
            rpass.set_viewport(
                self.pos.x0(), 
                self.pos.y0(), 
                self.pos.width(), 
                self.pos.height(),
                0.,
                1.
            );

            for item in self.flush_items.drain(..) {
                match item {
                    FlushItem::None => {},
                    FlushItem::Redraw => panic!("Redraw should not allow flush()"),
                    FlushItem::Mesh2d(item) => {
                        self.mesh2d_render.flush_item(
                            rpass, 
                            &self.texture_store, 
                            item
                        );
                    },
                    FlushItem::Bezier(item) => {
                        self.bezier_mesh_render.flush_item(
                            rpass, 
                            item
                        );
                    },
                    FlushItem::Mesh2dColor(item) => {
                        self.mesh2d_color_render.flush_item(
                            rpass, 
                            item
                        );
                    },
                    FlushItem::ShapeRect(item) => {
                        self.shape_rect_render.flush_item(
                            rpass, 
                            &self.texture_store,
                            item,
                        );
                    },
                }
            }
        });

        self.mesh2d_render.clear();
        self.bezier_mesh_render.clear();
        self.mesh2d_color_render.clear();

        self.shape_rect_render.clear();
     }
}

#[derive(Debug)]
pub enum FlushItem {
    None,
    Redraw, // force skipping of flush and redraw
    Mesh2d(Mesh2dFlush),
    Bezier(BezierFlush),
    Mesh2dColor(Mesh2dColorItem),

    ShapeRect(ShapeRectFlush),
}

