use std::{mem, num::NonZero};

use bytemuck_derive::{Zeroable, Pod};
use essay_graphics_api::{Affine2d, Mesh2dColor};

use crate::{pipelines::pipeline_canvas::FlushItem, render::render::RenderWgpu};

pub(super) struct Mesh2dColorRender {
    vertex_buffer: wgpu::Buffer,
    vertex_offset: usize,
    vertex_len: usize,

    style_buffer: wgpu::Buffer,
    style_offset: usize,
    style_len: usize,

    pipeline: wgpu::RenderPipeline,
}

impl Mesh2dColorRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
    ) -> Self {
        let len = 2048;

        let mut vertex_vec = Vec::<Vertex>::new();
        vertex_vec.resize(len, Vertex::empty());

        let vertex_buffer = create_vertex_buffer(
            device,
            len * mem::size_of::<Vertex>(),
        );

        let style_buffer = create_vertex_buffer(
            device,
            len * mem::size_of::<Style>(),
        );

        let pipeline = create_pipeline(
            device, 
            format,
        );
    
        Self {
            vertex_buffer,
            vertex_offset: 0,
            vertex_len: len,

            style_buffer,
            style_offset: 0,
            style_len: len,

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

        if self.vertex_len < self.vertex_offset + mesh.as_slice().len()
            || self.style_len <= self.style_offset + 1 {
            todo!();
        }

        let vec: Vec<Vertex> = mesh.as_slice().iter().map(|src| {
            Vertex {
                position: [src.0[0], src.0[1]],
                color: src.1.to_rgba(),
            }
        }).collect();

        let len = vec.len();
        let v_start = self.vertex_offset;
        let v_end = v_start + len;
        self.vertex_offset += len;

        if let Some(mut view) = wgpu.queue.write_buffer_with(
                &mut self.vertex_buffer, 
                (v_start * mem::size_of::<Vertex>()) as u64,
                NonZero::new(((v_end - v_start) * mem::size_of::<Vertex>()) as u64).unwrap(),
        ) {
            view.copy_from_slice(
                bytemuck::cast_slice(vec.as_slice())
            );
        }

        let mut vec = Vec::<Style>::new();
        vec.push(Style::new(affine));

        let len = vec.len();
        let s_start = self.style_offset;
        let s_end = s_start + len;
        self.style_offset += len;

        if let Some(mut view) = wgpu.queue.write_buffer_with(
                &mut self.style_buffer, 
                (s_start * mem::size_of::<Style>()) as u64,
                NonZero::new(((s_end - s_start) * mem::size_of::<Style>()) as u64).unwrap(),
        ) {
            view.copy_from_slice(
                bytemuck::cast_slice(vec.as_slice())
            );
        }

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
    
        let stride = mem::size_of::<Vertex>();
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(
            (stride * item.v_start) as u64..(stride * item.v_end) as u64
        ));

        let stride = mem::size_of::<Style>();
        rpass.set_vertex_buffer(1, self.style_buffer.slice(
            (stride * item.s_start) as u64..(stride * item.s_end) as u64
        ));

        rpass.draw(
            0..(item.v_end - item.v_start) as u32,
            0..(item.s_end - item.s_start) as u32,
        )
    }

    pub fn clear(&mut self) {
        self.vertex_offset = 0;
        self.style_offset = 0;
    }
}

fn create_vertex_buffer(
    device: &wgpu::Device,
    size: usize,
) -> wgpu::Buffer {
    device.create_buffer(
        &wgpu::BufferDescriptor {
        label: None,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        size: size as u64,
        mapped_at_creation: false,
    })
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

    fn empty() -> Vertex {
        Self {
            position: [0., 0.],
            color: 0,
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
