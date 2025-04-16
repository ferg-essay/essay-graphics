use bytemuck_derive::{Zeroable, Pod};
use essay_graphics_api::{Affine2d, Mesh2dColor};
use wgpu::util::DeviceExt;

use super::render::RenderWgpu;

pub(super) struct Mesh2dColorRender {
    vertex_stride: usize,
    vertex_vec: Vec<Vertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_offset: usize,

    style_stride: usize,
    style_vec: Vec<Style>,
    style_buffer: wgpu::Buffer,
    style_offset: usize,

    shape_items: Vec<Item>,

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

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertex_vec.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let mut style_vec = Vec::<Style>::new();
        style_vec.resize(len, Style::empty());

        let style_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(style_vec.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let pipeline = create_pipeline(
            device, 
            format,
        );
    
        Self {
            vertex_stride: std::mem::size_of::<Vertex>(),
            vertex_vec,
            vertex_buffer,
            vertex_offset: 0,

            style_stride: std::mem::size_of::<Style>(),
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
        mesh: &Mesh2dColor,
        affine: &Affine2d,
    ) {
        let len = mesh.vertices.len();

        if len == 0 {
            return;
        }

        if self.vertex_vec.len() < self.vertex_offset + mesh.as_slice().len()
            || self.style_vec.len() <= self.style_offset + 1 {
            self.flush(wgpu);
        }

        if self.vertex_vec.len() < self.vertex_offset + mesh.as_slice().len()
            || self.style_vec.len() <= self.style_offset + 1 {
            self.resize_buffers(wgpu.device, mesh);
            // todo!("Can't yet resize buffers: mesh-size {}", mesh.as_slice().len());
        }

        let offset = self.vertex_offset;

        for (dst, src) in self.vertex_vec.iter_mut().skip(offset).zip(mesh.as_slice()) {
            dst.position[0] = src.0[0];
            dst.position[1] = src.0[1];

            //dst.color = Color::from(src.1.to_lrgb()).to_rgba();
            dst.color = src.1.to_rgba();
        }

        self.vertex_offset += len;

        self.shape_items.push(Item {
            v_start: offset,
            v_end: self.vertex_offset,
            s_start: self.style_offset,
            s_end: self.style_offset + 1,
        });


        self.style_vec[self.style_offset] = Style::new(affine);

        self.style_offset += 1;
    }

    fn resize_buffers(
        &mut self, 
        device: &wgpu::Device,
        mesh: &Mesh2dColor,
    ) {
        let mut size = self.vertex_vec.len();

        while size < mesh.as_slice().len() {
            size += 2048;
        }

        let mut vertex_vec = Vec::new();
        vertex_vec.resize(size, Vertex::empty());

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertex_vec.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        self.vertex_buffer = vertex_buffer;
        self.vertex_vec = vertex_vec;
    }

    pub(super) fn flush(
        &mut self, wgpu: 
        &mut RenderWgpu,
    ) {
        if self.shape_items.len() == 0 {
            return;
        }

        wgpu.write_buffer(
            &mut self.vertex_buffer, 
            bytemuck::cast_slice(&self.vertex_vec.as_slice()[0..self.vertex_offset])
        );

        wgpu.write_buffer(
            &mut self.style_buffer, 
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
pub struct Item {
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

    pub fn empty() -> Style {
        Self {
            affine_0: [0., 0., 0., 0.],
            affine_1: [0., 0., 0., 0.],
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
