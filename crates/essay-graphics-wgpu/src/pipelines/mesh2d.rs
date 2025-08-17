use std::num::NonZero;

use bytemuck_derive::{Zeroable, Pod};
use essay_graphics_api::{path_style::MeshStyle, Affine2d, Color, Mesh2d, TextureId};

use crate::{pipelines::pipeline_canvas::FlushItem, render::render::RenderWgpu};
use super::{texture_store::TextureStore};

pub(super) struct Mesh2dRender {
    vertex_buffer: wgpu::Buffer,
    vertex_offset: usize,
    vertex_len: usize,

    style_buffer: wgpu::Buffer,
    style_offset: usize,
    style_len: usize,

    pipeline: wgpu::RenderPipeline,
}

impl Mesh2dRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
    ) -> Self {
        let len = 2048;

        let vertex_buffer = create_vertex_buffer(device, len);
        let style_buffer = create_style_buffer(device, len);

        let pipeline = create_shape2d_pipeline(
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
        mesh: &Mesh2d,
        texture: TextureId,
        style: &[MeshStyle],
    ) -> FlushItem {
        let len = mesh.vertices.len();

        if len == 0 || style.len() == 0 {
            return FlushItem::None;
        }

        if self.vertex_len < self.vertex_offset + mesh.as_slice().len()
            || self.style_len <= self.style_offset + style.len() {
            todo!();
            // self.flush(wgpu, textures);
            // self.resize_buffers(wgpu.device, mesh);
        }

        let vec: Vec<Vertex> = mesh.as_slice().iter().map(|src| {
            Vertex {
                position: [src[0], src[1]],
                uv: [src[2], src[3]],
            }
        }).collect();

        let len = vec.len();
        let v_start = self.vertex_offset;
        let v_end = v_start + len;
        self.vertex_offset += len;

        if let Some(mut view) = wgpu.queue.write_buffer_with(
                &mut self.vertex_buffer, 
                (v_start * Vertex::size_of()) as u64,
                NonZero::new(((v_end - v_start) * Vertex::size_of()) as u64).unwrap(),
        ) {
            view.copy_from_slice(
                bytemuck::cast_slice(vec.as_slice())
            );
        }

        let vec: Vec<Style> = style.iter().map(|src| {
            Style::new(&src.affine, src.color)
        }).collect();

        let len = vec.len();
        let s_start = self.style_offset;
        let s_end = s_start + len;
        self.style_offset += len;

        if let Some(mut view) = wgpu.queue.write_buffer_with(
                &mut self.style_buffer, 
                (s_start * Style::size_of()) as u64,
                NonZero::new(((s_end - s_start) * Style::size_of()) as u64).unwrap(),
        ) {
            view.copy_from_slice(
                bytemuck::cast_slice(vec.as_slice())
            );
        }

        FlushItem::Mesh2d(Mesh2dFlush {
            v_start,
            v_end,

            s_start,
            s_end,

            texture,
        })
    }

    pub(super) fn flush_item(
        &mut self, 
        rpass: &mut wgpu::RenderPass,
        textures: &TextureStore,
        item: Mesh2dFlush,
    ) {
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, textures.texture_bind_group(item.texture), &[]);
    
        let stride = Vertex::size_of();
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(
            (stride * item.v_start) as u64..(stride * item.v_end) as u64
        ));

        let stride = Style::size_of();
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

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
}

impl Vertex {
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2 ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }

    pub(crate) fn size_of() -> usize {
        std::mem::size_of::<Self>()
    }
}

fn create_vertex_buffer(
    device: &wgpu::Device,
    len: usize,
) -> wgpu::Buffer {
    let size = (Vertex::size_of() * len) as u64;

    device.create_buffer(
        &wgpu::BufferDescriptor {
        label: None,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        size,
        mapped_at_creation: false,
    })
}

fn create_style_buffer(
    device: &wgpu::Device,
    len: usize,
) -> wgpu::Buffer {
    let size = (Style::size_of() * len) as u64;

    device.create_buffer(
        &wgpu::BufferDescriptor {
        label: None,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        size,
        mapped_at_creation: false,
    })
}

#[derive(Debug)]
pub struct Mesh2dFlush {
    v_start: usize,
    v_end: usize,

    s_start: usize,
    s_end: usize,

    texture: TextureId,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Style {
    affine_0: [f32; 4],
    affine_1: [f32; 4],
    color: [f32; 4],
}

impl Style {
    const ATTRS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![
            2 => Float32x4, 
            3 => Float32x4,
            4 => Float32x4
        ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Style>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRS,
        }
    }

    pub(crate) fn size_of() -> usize {
        std::mem::size_of::<Self>()
    }

    fn new(affine: &Affine2d, color: Color) -> Self {
        let mat = affine.mat();

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

fn create_shape2d_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("mesh2d.wgsl"));

    let vertex_entry = "vs_shape";
    let fragment_entry = "fs_shape";

    let vertex_layout = Vertex::desc();
    let style_layout = Style::desc();

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[
            &texture_bind_group_layout(device),
        //    &camera_bind_group_layout(device),
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

fn texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: Some("texture bind_group layout"),
    })
}
