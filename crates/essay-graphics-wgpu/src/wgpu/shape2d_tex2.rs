use bytemuck_derive::{Pod, Zeroable};
use essay_graphics_api::{form::{Form, FormId, Matrix4, Shape, ShapeId}, Affine2d, TextureId};
use essay_tensor::Tensor;
use wgpu::util::DeviceExt;

use super::texture_store::TextureCache;

pub struct Shape2dTex2Render {
    vertex_stride: usize,
    vertex_vec: Vec<Vertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_offset: usize,

    texture_cache: TextureCache,
    depth_buffer: DepthBuffer,

    camera: CameraUniform,
    camera_buffer: wgpu::Buffer,

    form_items: Vec<FormItem>,
    draw_items: Vec<DrawItem>,

    pipeline: wgpu::RenderPipeline,
    camera_bind_group: wgpu::BindGroup,

    is_stale: bool,
    is_buffer_stale: bool,
}

impl Shape2dTex2Render {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
    ) -> Self {
        let len = 2048;

        let mut vertex_vec = Vec::<Vertex>::new();
        vertex_vec.resize(len, Vertex { 
            position: [0., 0.], 
            tex_uv: [0., 0.],
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

        let camera = CameraUniform::new();
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let depth_buffer = DepthBuffer::new(device, width, height);

        let pipeline = form3d_pipeline(
            device, 
            format,
        );
    
        Self {
            vertex_stride: std::mem::size_of::<Vertex>(),
            vertex_vec,
            vertex_buffer,
            vertex_offset: 0,

            form_items: Vec::new(),
            draw_items: Vec::new(),

            texture_cache: TextureCache::new(),
            depth_buffer,

            camera,
            camera_bind_group: camera_bind_group(device, &camera_buffer),
            camera_buffer,

            pipeline,

            is_stale: false,
            is_buffer_stale: false,
        }
    }

    pub(crate) fn resize(
        &mut self,
        device: &wgpu::Device, 
        width: u32,
        height: u32,
    ) {
        self.depth_buffer.resize(device, width, height);
    }

    pub fn clear(&mut self) {
        self.draw_items.drain(..);
    }

    pub fn create_texture_rgba8(
        &mut self, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        image: &Tensor<u8>
    ) -> TextureId {
        assert!(image.rank() == 3, "texture rank must be 3 shape={:?}", image.shape().as_slice());
        assert!(image.cols() == 4, "texture cols 4 shape={:?}", image.shape().as_slice());

        self.texture_cache.add_rgba_u8(
            device, 
            queue, 
            image.dim(1) as u32, 
            image.dim(0) as u32, 
            image.as_slice()
        )
    }

    pub fn create_shape(&mut self, shape: &Shape) -> ShapeId {
        let id = ShapeId(self.form_items.len());

        let mut item = FormItem {
            v_start: self.vertex_offset,
            v_end: usize::MAX,
            texture: shape.get_texture(),
        };

        for xy in shape.vertices().iter() {
            self.draw_vertex(xy.vertex().clone(), xy.tex_uv().clone());
        }
        
        item.v_end = self.vertex_offset;

        self.form_items.push(item);
        self.is_stale = true;

        id
    }

    fn draw_vertex(&mut self, pos: [f32; 2], tex_uv: [f32; 2]) {
        let vertex = Vertex { 
            position: pos,
            tex_uv,
        };

        let len = self.vertex_vec.len();
        if len <= self.vertex_offset {
            self.vertex_vec.resize(len + 2048, Vertex::empty());
            self.is_buffer_stale = true;
        }
        
        self.vertex_vec[self.vertex_offset] = vertex;
        self.vertex_offset += 1;
    }

    pub fn draw_shape(&mut self, shape: ShapeId) {
        self.draw_items.push(DrawItem::new(shape));
    }

    pub fn camera(
        &mut self, 
        camera: &Affine2d,
    ) {
        self.camera.set(camera);
    }

    pub fn flush(
        &mut self, 
        device: &wgpu::Device,
        queue: &wgpu::Queue, 
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        clip: Option<(u32, u32, u32, u32)>
    ) {
        if self.draw_items.len() == 0 {
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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_buffer.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        if self.is_buffer_stale {
            self.is_buffer_stale = false;
 
            self.vertex_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(self.vertex_vec.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }
            );
        }

        if self.is_stale {
            self.is_stale = false;

            queue.write_buffer(
                &mut self.vertex_buffer, 
                0,
                bytemuck::cast_slice(self.vertex_vec.as_slice())
            );
        }

        queue.write_buffer(
            &mut self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera])
        );

        rpass.set_pipeline(&self.pipeline);

        // rpass.set_stencil_ref
        rpass.set_bind_group(1, &self.camera_bind_group, &[]);

        if let Some((x, y, w, h)) = clip {
            rpass.set_scissor_rect(x, y, w, h);
        }

        for draw_item in self.draw_items.drain(..) {
            let item = &self.form_items[draw_item.id.0];

            /*
            if let Clip::Bounds(p0, p1) = draw_item.clip {
                rpass.set_scissor_rect(p0.0 as u32, p0.1 as u32, (p1.0 - p0.0) as u32, (p1.1 - p0.1) as u32);
            } else {
                // rpass.set_scissor_rect(0, u32::MAX, 0, u32::MAX);
            }
            */
    
            rpass.set_bind_group(0, self.texture_cache.texture_bind_group(item.texture), &[]);

            if item.v_start < item.v_end {
                let stride = self.vertex_stride;
                rpass.set_vertex_buffer(0, self.vertex_buffer.slice(
                    (stride * item.v_start) as u64..(stride * item.v_end) as u64
                ));

                rpass.draw(
                    0..(item.v_end - item.v_start) as u32,
                    0..1,
                );
            }
        }
    }
}

struct FormItem {
    v_start: usize,
    v_end: usize,

    texture: TextureId,
}

struct DrawItem {
    id: ShapeId,
}

impl DrawItem {
    fn new(id: ShapeId) -> Self {
        Self {
            id,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    tex_uv: [f32; 2],
}

impl Vertex {
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2 ];

    fn empty() -> Self {
        Self {
            position: [0., 0.],
            tex_uv: [0., 0.],
        }
    }

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
struct CameraUniform {
    a0: [f32; 3],
    a1: [f32; 3],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            a0: [1., 0., 0.],
            a1: [0., 1., 0.],
        }
    }

    fn set(&mut self, affine: &Affine2d) {
        let mat = affine.mat();
        self.a0 = [mat[0], mat[1], mat[2]];
        self.a1 = [mat[3], mat[4], mat[5]];
    }
}

//
// WGPU pipeline definition
//

fn form3d_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("shape2d_tex2.wgsl"));

    let vertex_entry = "vs_shape2d";
    let fragment_entry = "fs_shape2d";

    let vertex_layout = Vertex::desc();

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[
            &texture_bind_group_layout(device),
            &camera_bind_group_layout(device),
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
        // TODO: remove depth
        depth_stencil: Some(wgpu::DepthStencilState {
            format: DepthBuffer::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    })
}

fn camera_bind_group(
    device: &wgpu::Device,
    camera_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &camera_bind_group_layout(device),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }
        ],
        label: Some("camera bind group"),
    })
}

fn camera_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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
        label: Some("camera bind group layout"),
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

struct DepthBuffer {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
}

impl DepthBuffer {
    const DEPTH_FORMAT : wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    fn new(
        device: &wgpu::Device, 
        width: u32,
        height: u32,
    ) -> Self {
        let texture = depth_texture(device, width, height);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture,
            view,
        }
    }

    fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.texture = depth_texture(device, width, height);
        self.view = self.texture.create_view(&wgpu::TextureViewDescriptor::default());
    }
}

fn depth_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    let size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let desc = wgpu::TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DepthBuffer::DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    };

    device.create_texture(&desc)
}

fn _depth_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(
        &wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.,
            lod_max_clamp: 100.,
            ..Default::default()
        }
    )
}
