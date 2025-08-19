use bytemuck_derive::{Zeroable, Pod};
use essay_graphics_api::{renderer::Pos, Color, Point, Size, TextureId};
use wgpu::util::DeviceExt;

use crate::{pipelines::{buffer::{VertexBuffer}, pipeline_canvas::FlushItem}, render::render::RenderWgpu};
use super::{texture_store::TextureStore};

pub(super) struct ShapeRectRender {
    unit_vertex: wgpu::Buffer,

    viewport: wgpu::Buffer,
    viewport_bind_group: wgpu::BindGroup,

    style: VertexBuffer<Style>,

    pipeline: wgpu::RenderPipeline,
}

impl ShapeRectRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
    ) -> Self {
        let len = 2048;

        let pipeline = create_shape_rect_pipeline(
            device, 
            format,
        );

        let unit_vertex = [
            Vertex { pos: [-1., -1.], uv: [0., 0.] },
            Vertex { pos: [1., -1.], uv: [1., 0.] },
            Vertex { pos: [1., 1.], uv: [1., 1.] },

            Vertex { pos: [-1., -1.], uv: [0., 0.] },
            Vertex { pos: [1., 1.], uv: [1., 1.] },
            Vertex { pos: [-1., 1.], uv: [0., 1.] },
        ];
    
        let unit_vertex = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(unit_vertex.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let viewport = Viewport { size: [1., 1.] };
        let viewport_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Viewport Buffer"),
                contents: bytemuck::cast_slice(&[viewport]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        Self {
            unit_vertex,

            viewport_bind_group: viewport_bind_group(device, &viewport_buffer),
            viewport: viewport_buffer,

            style: VertexBuffer::new(device, len),

            pipeline,
        }
    }

    pub(super) fn resize(&mut self, wgpu: &mut RenderWgpu, pos: Pos) {
        wgpu.write_buffer(
            &mut self.viewport,
            bytemuck::cast_slice(&[Viewport::new(pos)])
        );
    }

    pub(super) fn draw(
        &mut self, 
        wgpu: &mut RenderWgpu,
        pos: Point,
        size: Size,
        r1: f32,
        texture: TextureId,
        color: Color,
    ) -> FlushItem {
        let styles = [Style {
            pos: [pos.x(), pos.y()],
            size: [size.width(), size.height()],
            r: r1,
            color: color.to_lrgb_u32(),
        }];

        if self.style.expand(wgpu, 1) {
            return FlushItem::Redraw;
        }

        let (s_start, s_end) = self.style.write(wgpu, &styles);

        FlushItem::ShapeRect(ShapeRectFlush {
            s_start,
            s_end,

            texture,
        })
    }

    pub(super) fn flush_item(
        &mut self, 
        rpass: &mut wgpu::RenderPass,
        textures: &TextureStore,
        item: ShapeRectFlush,
    ) {
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, textures.texture_bind_group(item.texture), &[]);
        rpass.set_bind_group(1, &self.viewport_bind_group, &[]);
    
        rpass.set_vertex_buffer(0, self.unit_vertex.slice(..));

        rpass.set_vertex_buffer(1, self.style.buffer_slice(item.s_start, item.s_end));

        rpass.draw(
            0..6,
            0..(item.s_end - item.s_start) as u32,
        )
    }

    pub fn clear(&mut self) {
        self.style.clear();
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pos: [f32; 2],
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
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Viewport {
    size: [f32; 2],
}

impl Viewport {
    fn new(pos: Pos) -> Self {
        Self {
            size: [pos.width(), pos.height()],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Style {
    pos: [f32; 2],
    size: [f32; 2],
    r: f32,
    color: u32,
}

impl Style {
    const ATTRS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![
            2 => Float32x2, 
            3 => Float32x2,
            4 => Float32,
            5 => Uint32,
        ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Style>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRS,
        }
    }
}

#[derive(Debug)]
pub struct ShapeRectFlush {
    s_start: usize,
    s_end: usize,

    texture: TextureId,
}

fn create_shape_rect_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("shape_rect.wgsl"));

    let vertex_entry = "vs_shape";
    let fragment_entry = "fs_shape";

    let vertex_layout = Vertex::desc();
    let style_layout = Style::desc();

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[
            &texture_bind_group_layout(device),
            &viewport_bind_group_layout(device),
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

fn viewport_bind_group(
    device: &wgpu::Device,
    viewport_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &viewport_bind_group_layout(device),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: viewport_buffer.as_entire_binding(),
            }
        ],
        label: Some("viewport bind group"),
    })
}

fn viewport_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }
        ],
        label: Some("viewport bind group layout"),
    })
}
