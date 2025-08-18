use bytemuck_derive::{Pod, Zeroable};
use essay_graphics_api::{path_style::MeshStyle, Affine2d, BezierMesh2d, Color};

use crate::{pipelines::{buffer::VertexBuffer, pipeline_canvas::FlushItem}, render::render::RenderWgpu};

pub struct BezierMeshRender {
    vertex: VertexBuffer<Vertex>,
    style: VertexBuffer<Style>,

    pipeline: wgpu::RenderPipeline,
}

impl BezierMeshRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
    ) -> Self {
        let len = 2048;

        let pipeline = create_bezier_pipeline(
            device, 
            format,
        );
    
        Self {
            vertex: VertexBuffer::new(device, len),
            style: VertexBuffer::new(device, len),

            pipeline,
        }
    }

    pub(super) fn draw(
        &mut self, 
        wgpu: &mut RenderWgpu,
        mesh: &BezierMesh2d, 
        style: &[MeshStyle],
    ) -> FlushItem {
        let len = mesh.vertices.len();

        if len == 0 || style.len() == 0 {
            return FlushItem::None;
        }

        if self.vertex.expand(wgpu, mesh.as_slice().len()) {
            return FlushItem::Redraw;
        }

        if self.style.expand(wgpu, style.len()) {
            return FlushItem::Redraw;
        }

        let vec: Vec<Vertex> = mesh.as_slice().iter().map(|src| {
            Vertex {
                position: [src[0], src[1]],
                uv: [src[2], src[3]],
                buv_ab: [src[4], src[5], src[6], src[7]],
            }
        }).collect();

        let (v_start, v_end) = self.vertex.write(wgpu, &vec);

        let vec: Vec<Style> = style.iter().map(|src| {
            Style::new(&src.affine, src.color)
        }).collect();

        let (s_start, s_end) = self.style.write(wgpu, &vec);

        FlushItem::Bezier(BezierFlush {
            v_start,
            v_end,

            s_start,
            s_end,

            // texture,
        })
    }

    pub(super) fn flush_item(
        &mut self, 
        rpass: &mut wgpu::RenderPass,
        item: BezierFlush,
    ) {
        rpass.set_pipeline(&self.pipeline);
    
        rpass.set_vertex_buffer(0, self.vertex.buffer_slice(item.v_start, item.v_end));
        rpass.set_vertex_buffer(1, self.style.buffer_slice(item.s_start, item.s_end));

        rpass.draw(
            0..(item.v_end - item.v_start) as u32,
            0..(item.s_end - item.s_start) as u32,
        )
    }

    pub fn clear(&mut self) {
        self.vertex.clear();
        self.style.clear();
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
    buv_ab: [f32; 4],
}

impl Vertex {
    const ATTRS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4 ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Style {
    affine_0: [f32; 4],
    affine_1: [f32; 4],
    color: [f32; 4],
}

impl Style {
    const ATTRS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![
            3 => Float32x4, 
            4 => Float32x4,
            5 => Float32x4
        ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Style>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRS,
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

#[derive(Debug)]
pub struct BezierFlush {
    v_start: usize,
    v_end: usize,

    s_start: usize,
    s_end: usize,

    // texture: TextureId,
}

fn create_bezier_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("bezier_mesh.wgsl"));

    let vertex_entry = "vs_bezier";
    let fragment_entry = "fs_bezier";

    let vertex_layout = Vertex::desc();
    let style_layout = Style::desc();

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
            entry_point: Some(vertex_entry),
            buffers: &[
                vertex_layout,
                style_layout,
            ],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some(fragment_entry),
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
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: Default::default(),
    })
}
