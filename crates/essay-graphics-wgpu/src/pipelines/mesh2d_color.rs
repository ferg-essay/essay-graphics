use bytemuck_derive::{Zeroable, Pod};
use essay_graphics_api::{Affine2d, Mesh2dColor};

use crate::{pipelines::{buffer::VertexBuffer, pipeline_canvas::FlushItem}, render::render::RenderWgpu};

pub(super) struct Mesh2dColorRender {
    vertex: VertexBuffer<Vertex>,
    style: VertexBuffer<Style>,

    pipeline: wgpu::RenderPipeline,
}

impl Mesh2dColorRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
    ) -> Self {
        let len = 2048;

        let pipeline = create_pipeline(
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
        mesh: &Mesh2dColor,
        affine: &Affine2d,
    ) -> FlushItem {
        let len = mesh.vertices.len();

        if len == 0 {
            return FlushItem::None;
        }

        if self.vertex.expand(wgpu, len) {
            return FlushItem::Redraw;
        }

        if self.style.expand(wgpu, 1) {
            return FlushItem::Redraw;
        }

        let vec: Vec<Vertex> = mesh.as_slice().iter().map(|src| {
            Vertex {
                position: [src.0[0], src.0[1]],
                color: src.1.to_rgba(),
            }
        }).collect();

        let (v_start, v_end) = self.vertex.write(wgpu, &vec);

        let mut vec = Vec::<Style>::new();
        vec.push(Style::new(affine));

        let (s_start, s_end) = self.style.write(wgpu, &vec);

        FlushItem::Mesh2dColor(Mesh2dColorItem {
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
        item: Mesh2dColorItem,
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
pub struct Vertex {
    position: [f32; 2],
    color: u32,
}

impl Vertex {
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Uint32 ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }
}

#[derive(Debug)]
pub struct Mesh2dColorItem {
    v_start: usize,
    v_end: usize,

    s_start: usize,
    s_end: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Style {
    affine_0: [f32; 4],
    affine_1: [f32; 4],
}

impl Style {
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![
            2 => Float32x4, 
            3 => Float32x4,
        ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Style>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRS,
        }
    }

    fn new(affine: &Affine2d) -> Self {
        let mat = affine.mat();

        Self {
            affine_0: [mat[0], mat[1], 0., mat[2]],
            affine_1: [mat[3], mat[4], 0., mat[5]],
        }
    }
}

fn create_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("mesh2d_color.wgsl"));

    let vertex_entry = "vs_shape";
    let fragment_entry = "fs_shape";

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
