use std::num::NonZero;

use bytemuck_derive::{Zeroable, Pod};
use essay_graphics_api::{path_style::MeshStyle, Affine2d, Color, Mesh2d, TextureId};
use wgpu::util::DeviceExt;

use crate::render::{render::RenderWgpu};
use super::{texture_store::TextureStore};

pub(super) struct Mesh2dRender {
    vertex_buffer: wgpu::Buffer,
    vertex_offset: usize,
    vertex_len: usize,

    style_buffer: wgpu::Buffer,
    style_offset: usize,
    _style_len: usize,
    style_stride: usize,
    style_vec: Vec<Style>,

    shape_items: Vec<Item>,

    pipeline: wgpu::RenderPipeline,
}

impl Mesh2dRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
    ) -> Self {
        let len = 2048;

        let vertex_buffer = create_vertex_buffer(device, len);

        let mut style_vec = Vec::<Style>::new();
        style_vec.resize(len, Style::empty());

        let style_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(style_vec.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let pipeline = create_shape2d_pipeline(
            device, 
            format,
        );
    
        Self {
            vertex_buffer,
            vertex_offset: 0,
            vertex_len: len,

            style_stride: std::mem::size_of::<Style>(),
            style_vec,
            style_buffer,
            style_offset: 0,
            _style_len: 0,

            shape_items: Vec::new(),
            pipeline,
        }
    }

    pub(super) fn draw(
        &mut self, 
        wgpu: &mut RenderWgpu,
        textures: &TextureStore,
        mesh: &Mesh2d,
        texture: TextureId,
        style: &[MeshStyle],
    ) {
        let len = mesh.vertices.len();

        if len == 0 || style.len() == 0 {
            return;
        }

        if self.vertex_len < self.vertex_offset + mesh.as_slice().len()
            || self.style_vec.len() <= self.style_offset + style.len() {
            self.flush(wgpu, textures);
            self.resize_buffers(wgpu.device, mesh);
        }

        self.start_shape(texture);

        let vec: Vec<Vertex> = mesh.as_slice().iter().map(|src| {
            Vertex {
                position: [src[0], src[1]],
                uv: [src[2], src[3]],
            }
        }).collect();

        let start_offset = self.vertex_offset * Vertex::size_of();
        let end_offset = (self.vertex_offset + len) * Vertex::size_of();
        if let Some(mut view) = wgpu.queue.write_buffer_with(
                &mut self.vertex_buffer, 
                start_offset as u64,
                NonZero::new((end_offset - start_offset) as u64).unwrap(),
        ) {
            view.copy_from_slice(
                bytemuck::cast_slice(vec.as_slice())
            );
        }

        self.vertex_offset += len;
        
        for MeshStyle { color, affine } in style {
            self.draw_style(*color, affine);
        }
    }

    fn resize_buffers(
        &mut self, 
        device: &wgpu::Device,
        mesh: &Mesh2d,
    ) {
        let mut size = self.vertex_len;

        while size < mesh.as_slice().len() {
            size += 2048;
        }

        self.vertex_len = size;

        self.vertex_buffer = create_vertex_buffer(device, size);

        // need to copy old data or force a redraw
        todo!("need to copy old data");

    }

    fn start_shape(&mut self, texture: TextureId) {
        let start = self.vertex_offset;

        self.shape_items.push(Item {
            v_start: start,
            v_end: usize::MAX,
            s_start: self.style_offset,
            s_end: usize::MAX,
            texture,
        });
    }

    fn draw_style(
        &mut self, 
        color: Color,
        affine: &Affine2d,
    ) {
        let end = self.vertex_offset;

        let len = self.shape_items.len();

        assert!(self.style_offset < self.style_vec.len());

        let item = &mut self.shape_items[len - 1];
        item.v_end = end;

        self.style_vec[self.style_offset] = Style::new(affine, color);
        self.style_offset += 1;

        item.s_end = self.style_offset;
    }

    pub(super) fn flush(
        &mut self, wgpu: 
        &mut RenderWgpu,
        textures: &TextureStore,
    ) {
        if self.shape_items.len() == 0 {
            return;
        }

        wgpu.write_buffer(
            &mut self.style_buffer, 
            bytemuck::cast_slice(&self.style_vec.as_slice()[0..self.style_offset])
        );

        wgpu.render_pass(|rpass| {
            rpass.set_pipeline(&self.pipeline);

            for item in self.shape_items.drain(..) {
                rpass.set_bind_group(0, textures.texture_bind_group(item.texture), &[]);
    
                if item.v_start < item.v_end && item.s_start < item.s_end {
                    let stride = Vertex::size_of();
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
        std::mem::size_of::<Vertex>()
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

#[derive(Debug)]
pub struct Item {
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

    pub fn empty() -> Style {
        Self {
            affine_0: [0., 0., 0., 0.],
            affine_1: [0., 0., 0., 0.],
            color: [0., 0., 0., 0.],
        }
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
