use core::fmt;
use std::{any::Any, mem, sync::Arc};

use bytemuck_derive::{Zeroable, Pod};
use essay_graphics_api::{path_style::MeshStyle, renderer::{Mesh2dBuffer, Result}, Affine2d, Color, Mesh2d, TextureId};

use crate::{pipelines::{buffer::{create_vertex_buffer_init, VertexBuffer}, pipeline_canvas::FlushItem}, render::render::RenderWgpu};
use super::{texture_store::TextureStore};

pub(super) struct Mesh2dRender {
    vertex: VertexBuffer<Vertex>,
    style: VertexBuffer<Style>,

    pipeline: wgpu::RenderPipeline,
}

impl Mesh2dRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
    ) -> Self {
        let len = 2048;

        let pipeline = create_shape2d_pipeline(
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
        mesh: &Mesh2d,
        texture: TextureId,
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

        let vec = Vertex::from_mesh(mesh);

        //let (v_start, v_end) = self.vertex.write(wgpu, &vec);
        let (v_start, v_end) = self.vertex.stage(wgpu, &vec);

        let data: Vec<Style> = style.iter().map(|src| {
            Style::new(&src.affine, src.color)
        }).collect();

        //let (s_start, s_end) = self.style.write(wgpu, &data);
        let (s_start, s_end) = self.style.stage(wgpu, &data);

        FlushItem::Mesh2d(Mesh2dFlush {
            v_start,
            v_end,

            s_start,
            s_end,

            texture,
        })
    }
    
    pub(crate) fn create_buffer(
        &self, 
        wgpu: &mut RenderWgpu, 
        mesh: &Mesh2d
    ) -> Result<Mesh2dBuffer> {
        let vertices = Vertex::from_mesh(mesh);

        let buffer = create_vertex_buffer_init(wgpu, &vertices);

        Ok(Mesh2dBuffer::new(Mesh2dBufferItem {
            buffer,
            n_vertex: vertices.len() as u32,
        }))
    }

    pub(super) fn draw_buffer(
        &mut self, 
        wgpu: &mut RenderWgpu,
        buffer: &Mesh2dBuffer,
        texture: TextureId,
        style: &[MeshStyle],
    ) -> FlushItem {
        if self.style.expand(wgpu, style.len()) {
            return FlushItem::Redraw;
        }

        let data: Vec<Style> = style.iter().map(|src| {
            Style::new(&src.affine, src.color)
        }).collect();

        //let (s_start, s_end) = self.style.write(wgpu, &data);
        let (s_start, s_end) = self.style.stage(wgpu, &data);

        FlushItem::Mesh2dBuffer(Mesh2dBufferFlush {
            vertices: buffer.clone(),

            s_start,
            s_end,

            texture,
        })
    }

    pub(super) fn write_stage(&mut self, wgpu: &mut RenderWgpu) {
        self.vertex.write_stage(wgpu);
        self.style.write_stage(wgpu);
    }

    pub(super) fn flush_item(
        &mut self, 
        rpass: &mut wgpu::RenderPass,
        textures: &TextureStore,
        item: Mesh2dFlush,
    ) {
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, textures.texture_bind_group(item.texture), &[]);
    
        rpass.set_vertex_buffer(0, self.vertex.buffer_slice(item.v_start, item.v_end));

        rpass.set_vertex_buffer(1, self.style.buffer_slice(item.s_start, item.s_end));
        rpass.draw(
            0..(item.v_end - item.v_start) as u32,
            0..(item.s_end - item.s_start) as u32,
        )
    }

    pub(super) fn flush_buffer_item(
        &mut self, 
        rpass: &mut wgpu::RenderPass,
        textures: &TextureStore,
        item: Mesh2dBufferFlush,
    ) {
        let buffer_item = item.vertices.0.downcast_ref::<Mesh2dBufferItem>().unwrap();
    
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, textures.texture_bind_group(item.texture), &[]);

        rpass.set_vertex_buffer(0, buffer_item.buffer.slice(..));

        rpass.set_vertex_buffer(1, self.style.buffer_slice(item.s_start, item.s_end));

        rpass.draw(
            0..buffer_item.n_vertex as u32,
            0..(item.s_end - item.s_start) as u32,
        )


    }

    pub fn clear(&mut self) {
        self.vertex.clear();
        self.style.clear();
    }
}

struct Mesh2dBufferItem {
    buffer: wgpu::Buffer,
    n_vertex: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
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

    fn from_mesh(mesh: &Mesh2d) -> Vec<Self> {
        mesh.as_slice().iter().map(|src| {
            Vertex {
                position: [src[0], src[1]],
                uv: [src[2], src[3]],
            }
        }).collect()
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct Style {
    affine_0: [f32; 4],
    affine_1: [f32; 4],
    color: u32,
}

impl Style {
    const ATTRS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![
            2 => Float32x4, 
            3 => Float32x4,
            4 => Uint32,
        ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Style>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRS,
        }
    }

    fn new(affine: &Affine2d, color: Color) -> Self {
        let mat = affine.mat();

        Self {
            affine_0: [mat[0], mat[1], 0., mat[2]],
            affine_1: [mat[3], mat[4], 0., mat[5]],
            color: color.to_lrgb_u32(),
        }
    }
}

#[derive(Debug)]
pub struct Mesh2dFlush {
    v_start: usize,
    v_end: usize,

    s_start: usize,
    s_end: usize,

    texture: TextureId,
}
impl Mesh2dFlush {
    pub(crate) fn merge(&mut self, next: &Mesh2dFlush) -> bool {
        if self.texture == next.texture
        && self.v_end == next.v_start
        && self.s_end == next.s_start {
            self.v_end = next.v_end;
            self.s_end = next.s_end;

            true
        } else {
            false
        } 
    }
}

pub struct Mesh2dBufferFlush {
    vertices: Mesh2dBuffer,

    s_start: usize,
    s_end: usize,

    texture: TextureId,
}

impl fmt::Debug for Mesh2dBufferFlush {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mesh2dBufferFlush")
            .field("s_start", &self.s_start)
            .field("s_end", &self.s_end)
            .field("texture", &self.texture)
            .finish()
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
