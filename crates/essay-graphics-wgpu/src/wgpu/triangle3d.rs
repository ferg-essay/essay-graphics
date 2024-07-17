use bytemuck_derive::{Pod, Zeroable};
use essay_graphics_api::{Affine2d, Clip};
use wgpu::util::DeviceExt;

pub struct Triangle3dRender {
    vertex_stride: usize,
    vertex_vec: Vec<Triangle3dVertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_offset: usize,

    index_stride: usize,
    index_vec: Vec<u32>,
    index_buffer: wgpu::Buffer,
    index_offset: usize,

    style_stride: usize,
    style_vec: Vec<Triangle3dStyle>,
    style_buffer: wgpu::Buffer,
    style_offset: usize,

    mesh_items: Vec<GridMesh2dItem>,

    pipeline: wgpu::RenderPipeline,

    is_stale: bool,
}

impl Triangle3dRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
    ) -> Self {
        let len = 2048;

        let mut vertex_vec = Vec::<Triangle3dVertex>::new();
        vertex_vec.resize(len, Triangle3dVertex { 
            position: [0., 0., 0.], 
            color: 0 
        });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertex_vec.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let mut index_vec = Vec::<u32>::new();
        index_vec.resize(len, 0);

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(index_vec.as_slice()),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let mut style_vec = Vec::<Triangle3dStyle>::new();
        style_vec.resize(len, Triangle3dStyle { 
            affine_0: [0.0, 0.0, 0.0, 0.0], 
            affine_1: [0.0, 0.0, 0.0, 0.0], 
        });

        let style_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(style_vec.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let pipeline = create_triangle3d_pipeline(
            device, 
            format,
        );
    
        Self {
            vertex_stride: std::mem::size_of::<Triangle3dVertex>(),
            vertex_vec,
            vertex_buffer,
            vertex_offset: 0,

            index_stride: std::mem::size_of::<u32>(),
            index_vec,
            index_buffer,
            index_offset: 0,

            style_stride: std::mem::size_of::<Triangle3dStyle>(),
            style_vec,
            style_buffer,
            style_offset: 0,
            // style_bind_group,

            mesh_items: Vec::new(),
            pipeline,

            is_stale: false,
        }
    }

    pub fn clear(&mut self) {
        self.vertex_offset = 0;
        self.index_offset = 0;
        self.style_offset = 0;
        self.mesh_items.drain(..);
        self.is_stale = false;
    }

    pub fn start_triangles(&mut self) {
        self.mesh_items.push(GridMesh2dItem {
            v_start: self.vertex_offset,
            v_end: usize::MAX,
            i_start: self.index_offset,
            i_end: usize::MAX,
            s_start: self.style_offset,
            s_end: usize::MAX,
        });
    }

    pub fn draw_vertex(&mut self, x: f32, y: f32, z: f32, rgba: u32) {
        let vertex = Triangle3dVertex { 
            position: [x, y, z],
            color: rgba,
        };

        let len = self.vertex_vec.len();
        if len <= self.vertex_offset {
            self.vertex_vec.resize(len + 2048, Triangle3dVertex::empty());
            self.is_stale = true;
        }
        
        self.vertex_vec[self.vertex_offset] = vertex;
        self.vertex_offset += 1;
    }

    pub fn draw_triangle(&mut self, v0: u32, v1: u32, v2: u32) {
        let item = &self.mesh_items[self.mesh_items.len() - 1];

        let v_start = item.v_start;
        let offset = self.index_offset;

        let len = self.index_vec.len();
        if len <= self.index_offset + 2 {
            self.index_vec.resize(len + 2048, 0);
            self.is_stale = true;
        }

        assert!((v_start + v0 as usize) < self.vertex_offset);
        self.index_vec[offset] = v0;
        assert!((v_start + v1 as usize) < self.vertex_offset);
        self.index_vec[offset + 1] = v1;
        assert!((v_start + v2 as usize) < self.vertex_offset);
        self.index_vec[offset + 2] = v2;

        self.index_offset += 3;
    }

    pub fn draw_style(
        &mut self, 
        affine: &Affine2d,
    ) {
        let v_end = self.vertex_offset;
        let i_end = self.index_offset;

        let len = self.mesh_items.len();
        let item = &mut self.mesh_items[len - 1];

        item.v_end = v_end;
        item.i_end = i_end;

        self.style_vec[self.style_offset] = Triangle3dStyle::new(affine);
        self.style_offset += 1;

        item.s_end = self.style_offset;
    }

    pub fn flush(
        &mut self, 
        device: &wgpu::Device,
        queue: &wgpu::Queue, 
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        clip: &Clip,
    ) {
        if self.mesh_items.len() == 0 {
            return;
        }

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        if self.is_stale {
            self.is_stale = false;
 
            self.vertex_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(self.vertex_vec.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }
            );
    
            self.index_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(self.index_vec.as_slice()),
                    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                }
            );
    
            self.style_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(self.style_vec.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }
            );
        }

        queue.write_buffer(
            &mut self.vertex_buffer, 
            0,
            bytemuck::cast_slice(self.vertex_vec.as_slice())
        );

        queue.write_buffer(
            &mut self.index_buffer, 
            0,
            bytemuck::cast_slice(self.index_vec.as_slice())
        );

        queue.write_buffer(
            &mut self.style_buffer, 
            0,
            bytemuck::cast_slice(self.style_vec.as_slice())
        );

        rpass.set_pipeline(&self.pipeline);

        if let Clip::Bounds(p0, p1) = clip {
            rpass.set_scissor_rect(p0.0 as u32, p0.1 as u32, (p1.0 - p0.0) as u32, (p1.1 - p0.1) as u32);
        }

        for item in self.mesh_items.drain(..) {
            if item.v_start < item.v_end && item.i_start < item.i_end {
                let stride = self.vertex_stride;
                rpass.set_vertex_buffer(0, self.vertex_buffer.slice(
                    (stride * item.v_start) as u64..(stride * item.v_end) as u64
                ));
                let stride = self.style_stride;
                rpass.set_vertex_buffer(1, self.style_buffer.slice(
                    (stride * item.s_start) as u64..(stride * item.s_end) as u64
                ));
                let stride = self.index_stride;
                rpass.set_index_buffer(self.index_buffer.slice(
                    (stride * item.i_start) as u64..(stride * item.i_end) as u64
                ), wgpu::IndexFormat::Uint32
                );

                rpass.draw_indexed(
                    0..(item.i_end - item.i_start) as u32,
                    0,
                    0..(item.s_end - item.s_start) as u32,
                );
            }
        }
    }
}

pub struct GridMesh2dItem {
    v_start: usize,
    v_end: usize,

    i_start: usize,
    i_end: usize,

    s_start: usize,
    s_end: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Triangle3dVertex {
    position: [f32; 3],
    color: u32,
}

impl Triangle3dVertex {
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Uint32 ];

    fn empty() -> Self {
        Self {
            position: [0., 0., 0.],
            color: 0,
        }
    }

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Triangle3dVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }

}
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Triangle3dStyle {
    affine_0: [f32; 4],
    affine_1: [f32; 4],
}

impl Triangle3dStyle {
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![
            2 => Float32x4, 
            3 => Float32x4,
        ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Triangle3dStyle>() as wgpu::BufferAddress,
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

fn create_triangle3d_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("triangle3d.wgsl"));

    let vertex_entry = "vs_triangle";
    let fragment_entry = "fs_triangle";

    let vertex_layout = Triangle3dVertex::desc();
    let style_layout = Triangle3dStyle::desc();

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
