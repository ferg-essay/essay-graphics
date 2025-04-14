use bytemuck_derive::{Pod, Zeroable};
use essay_graphics_api::{path_style::MeshStyle, Affine2d, BezierMesh2d, Color};
use wgpu::util::DeviceExt;

use super::render::RenderWgpu;

pub struct BezierMeshRender {
    vertex_stride: usize,
    vertex_vec: Vec<BezierVertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_offset: usize,

    style_stride: usize,
    style_vec: Vec<BezierStyle>,
    style_buffer: wgpu::Buffer,
    style_offset: usize,

    shape_items: Vec<BezierItem>,

    pipeline: wgpu::RenderPipeline,
}

impl BezierMeshRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
    ) -> Self {
        let len = 2048;

        let mut vertex_vec = Vec::<BezierVertex>::new();
        vertex_vec.resize(len, BezierVertex::empty());

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertex_vec.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let mut style_vec = Vec::<BezierStyle>::new();
        style_vec.resize(len, BezierStyle::empty());

        let style_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(style_vec.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let pipeline = create_bezier_pipeline(
            device, 
            format,
        );
    
        Self {
            vertex_stride: std::mem::size_of::<BezierVertex>(),
            vertex_vec,
            vertex_buffer,
            vertex_offset: 0,

            style_stride: std::mem::size_of::<BezierStyle>(),
            style_vec,
            style_buffer,
            style_offset: 0,

            shape_items: Vec::new(),
            pipeline,
        }
    }

    pub(super) fn draw(
        &mut self, 
        wgpu: &mut RenderWgpu,
        mesh: &BezierMesh2d, 
        style: &[MeshStyle],
    ) {
        let mesh_vertices = mesh.as_slice();

        if mesh_vertices.len() == 0 || style.len() == 0{
            return;
        }

        self.start_shape();

        let len = self.vertex_vec.len();
        let offset = self.vertex_offset;

        if len < offset + mesh_vertices.len() 
            || self.style_vec.len() + self.style_offset < style.len() {
            self.flush(wgpu);
        }

        assert!(offset + mesh_vertices.len() <= len);

        for (dst, src) in self.vertex_vec.iter_mut().skip(offset).zip(mesh_vertices) {
            dst.position = [src[0], src[1]];
            dst.uv = [src[2], src[3]];
            dst.buv_ab = [src[4], src[5],src[6], src[7]];
        }

        self.vertex_offset += mesh_vertices.len();

        for MeshStyle { color, affine } in style {
            self.draw_style(*color, affine);
        }
    }

    fn start_shape(&mut self) {
        let start = self.vertex_offset;

        self.shape_items.push(BezierItem {
            v_start: start,
            v_end: start,
            s_start: self.style_offset,
            s_end: self.style_offset,
        });
    }

    fn draw_style(
        &mut self, 
        color: Color,
        affine: &Affine2d,
    ) {
        let end = self.vertex_offset;
        let len = self.shape_items.len();

        // todo: flush if overflow
        assert!(self.style_offset < self.style_vec.len());

        let item = &mut self.shape_items[len - 1];
        item.v_end = end;

        self.style_vec[self.style_offset] = BezierStyle::new(affine, color);
        self.style_offset += 1;

        item.s_end = self.style_offset;
    }

    pub(super) fn flush(
        &mut self, 
        wgpu: &mut RenderWgpu,
    ) {
        if self.shape_items.len() == 0 {
            return;
        }

        wgpu.write_buffer(
            &mut self.vertex_buffer, 
            bytemuck::cast_slice(&self.vertex_vec.as_slice()[0..self.vertex_offset])
        );

        wgpu.write_buffer(
            &self.style_buffer,
            bytemuck::cast_slice(&self.style_vec.as_slice()[0..self.style_offset])
        );

        wgpu.render_pass(|rpass| {
            rpass.set_pipeline(&self.pipeline);

            for item in self.shape_items.drain(..) {
                if item.v_start < item.v_end && item.s_start < item.s_end {
                    let stride = self.vertex_stride;
                    rpass.set_vertex_buffer(0, self.vertex_buffer.slice(
                        (stride * item.v_start) as u64..(stride * item.v_end) as u64
                    ));

                    let stride = self.style_stride;
                    rpass.set_vertex_buffer(1, self.style_buffer.slice(
                        (stride * item.s_start) as u64..(stride * item.s_end) as u64
                    ));

                    rpass.draw(
                        0..(item.v_end - item.v_start) as u32,
                        0..(item.s_end - item.s_start) as u32,
                    );
                }
            }
        });

        self.clear();
    }

    fn clear(&mut self) {
        self.shape_items.drain(..);
        self.vertex_offset = 0;
        self.style_offset = 0;
    }
}

struct BezierItem {
    v_start: usize,
    v_end: usize,

    s_start: usize,
    s_end: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct BezierVertex {
    position: [f32; 2],
    uv: [f32; 2],
    buv_ab: [f32; 4],
}

impl BezierVertex {
    const ATTRS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4 ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<BezierVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }

    fn empty() -> BezierVertex {
        Self {
            position: [0.0, 0.0],
            uv: [0., 0.],
            buv_ab: [0., 0., 0., 0.],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct BezierStyle {
    affine_0: [f32; 4],
    affine_1: [f32; 4],
    color: [f32; 4],
}

impl BezierStyle {
    const ATTRS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![
            3 => Float32x4, 
            4 => Float32x4,
            5 => Float32x4
        ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<BezierStyle>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRS,
        }
    }

    fn empty() -> Self {
        Self {
            affine_0: [0., 0., 0., 0.],
            affine_1: [0., 0., 0., 0.],
            color: [0., 0., 0., 0.],
        }
    }

    fn new(camera: &Affine2d, color: Color) -> Self {
        let mat = camera.mat();

        Self {
            affine_0: [mat[0], mat[1], 0., mat[2]],
            affine_1: [mat[3], mat[4], 0., mat[5]],
            color: [
                Color::srgb_to_lrgb(color.red()),
                Color::srgb_to_lrgb(color.green()),
                Color::srgb_to_lrgb(color.blue()),
                color.alpha(),
            ],
        }
    }
}

fn create_bezier_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("bezier_mesh.wgsl"));

    let vertex_entry = "vs_bezier";
    let fragment_entry = "fs_bezier";

    let vertex_layout = BezierVertex::desc();
    let style_layout = BezierStyle::desc();

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[
        ],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: vertex_entry,
            buffers: &[
                vertex_layout,
                style_layout,
            ],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: fragment_entry,
            targets: &[
                Some(wgpu::ColorTargetState {
                    format,

                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add
                        },

                        alpha: wgpu::BlendComponent::OVER
                    }),

                    write_mask: wgpu::ColorWrites::ALL,
                })
            ],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    })
}
