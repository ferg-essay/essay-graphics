use bytemuck_derive::{Pod, Zeroable};
use essay_graphics_api::{form::{Form, FormId, Matrix4}, Clip, Color, TextureId};
use essay_tensor::Tensor;
use wgpu::util::DeviceExt;

use super::texture_store::TextureCache;

pub struct Form3dRender {
    vertex_stride: usize,
    vertex_vec: Vec<Vertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_offset: usize,

    index_stride: usize,
    index_vec: Vec<u32>,
    index_buffer: wgpu::Buffer,
    index_offset: usize,

    style_stride: usize,
    style_vec: Vec<Style>,
    style_buffer: wgpu::Buffer,
    style_offset: usize,

    camera: CameraUniform,
    camera_buffer: wgpu::Buffer,

    form_items: Vec<FormItem>,
    draw_items: Vec<DrawItem>,

    pipeline: wgpu::RenderPipeline,
    camera_bind_group: wgpu::BindGroup,

    texture_cache: TextureCache,

    is_stale: bool,
    is_buffer_stale: bool,
}

impl Form3dRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
    ) -> Self {
        let len = 2048;

        let mut vertex_vec = Vec::<Vertex>::new();
        vertex_vec.resize(len, Vertex { 
            position: [0., 0., 0.], 
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

        let mut style_vec = Vec::<Style>::new();
        style_vec.resize(len, Style { 
            dummy: 0,
        });

        let style_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(style_vec.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
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

        let pipeline = form3d_pipeline(
            device, 
            format,
        );
    
        Self {
            vertex_stride: std::mem::size_of::<Vertex>(),
            vertex_vec,
            vertex_buffer,
            vertex_offset: 0,

            index_stride: std::mem::size_of::<u32>(),
            index_vec,
            index_buffer,
            index_offset: 0,

            style_stride: std::mem::size_of::<Style>(),
            style_vec,
            style_buffer,
            style_offset: 0,
            // style_bind_group,

            form_items: Vec::new(),
            draw_items: Vec::new(),

            texture_cache: TextureCache::new(),

            camera,
            camera_bind_group: camera_bind_group(device, &camera_buffer),
            camera_buffer,

            pipeline,

            is_stale: false,
            is_buffer_stale: false,
        }
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

    pub fn create_form(&mut self, form: &Form) -> FormId {
        let id = FormId(self.form_items.len());

        let mut item = FormItem {
            v_start: self.vertex_offset,
            v_end: usize::MAX,
            i_start: self.index_offset,
            i_end: usize::MAX,
            s_start: self.style_offset,
            s_end: usize::MAX,
            texture: form.get_texture(),
        };

        for xy in form.vertices().iter() {
            self.draw_vertex(xy.vertex().clone(), xy.tex_uv().clone());
        }

        for tri in form.triangles().iter() {
            self.draw_triangle(
                tri[0] as u32, 
                tri[1] as u32, 
                tri[2] as u32
            );
        }

        self.draw_style();
        
        item.v_end = self.vertex_offset;
        item.i_end = self.index_offset;
        item.s_end = self.style_offset;

        self.form_items.push(item);
        self.is_stale = true;

        id
    }

    fn draw_vertex(&mut self, pos: [f32; 3], tex_uv: [f32; 2]) {
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

    fn draw_triangle(&mut self, v0: u32, v1: u32, v2: u32) {
        // let item = &self.form_items[self.form_items.len() - 1];
        // let v_start = item.v_start;
        let offset = self.index_offset;

        let len = self.index_vec.len();
        if len <= self.index_offset + 2 {
            self.index_vec.resize(len + 2048, 0);
            self.is_buffer_stale = true;
        }

        // assert!((v_start + v0 as usize) < self.vertex_offset);
        assert!(v0 < self.vertex_offset as u32);
        self.index_vec[offset] = v0;
        // assert!((v_start + v1 as usize) < self.vertex_offset);
        assert!(v1 < self.vertex_offset as u32);
        self.index_vec[offset + 1] = v1;
        // assert!((v_start + v2 as usize) < self.vertex_offset);
        assert!(v2 < self.vertex_offset as u32);
        self.index_vec[offset + 2] = v2;

        self.index_offset += 3;
    }

    fn draw_style(
        &mut self, 
    ) {
        let len = self.style_vec.len();
        if len <= self.style_offset + 2 {
            self.style_vec.resize(len + 512, Style::new());
            self.is_buffer_stale = true;
        }

        self.style_vec[self.style_offset] = Style::new();
        self.style_offset += 1;
    }

    pub fn draw_form(&mut self, form: FormId) {
        self.draw_items.push(DrawItem::new(form));
    }

    pub fn camera(
        &mut self, 
        camera: &Matrix4,
    ) {
        self.camera.set(camera);
    }

    pub fn flush(
        &mut self, 
        device: &wgpu::Device,
        queue: &wgpu::Queue, 
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        clip: &Clip,
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
            depth_stencil_attachment: None,
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

        if self.is_stale {
            self.is_stale = false;

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

            /*
            queue.write_buffer(
                &mut self.style_buffer, 
                0,
                bytemuck::cast_slice(self.style_vec.as_slice())
            );
            */
        }

        queue.write_buffer(
            &mut self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera])
        );

        if let Clip::Bounds(p0, p1) = clip {
            rpass.set_scissor_rect(p0.0 as u32, p0.1 as u32, (p1.0 - p0.0) as u32, (p1.1 - p0.1) as u32);
        }

        rpass.set_pipeline(&self.pipeline);

        rpass.set_bind_group(1, &self.camera_bind_group, &[]);

        for draw_item in self.draw_items.drain(..) {
            let item = &self.form_items[draw_item.id.0];

            rpass.set_bind_group(0, self.texture_cache.texture_bind_group(item.texture), &[]);

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

struct FormItem {
    v_start: usize,
    v_end: usize,

    i_start: usize,
    i_end: usize,

    s_start: usize,
    s_end: usize,

    texture: TextureId,
}

struct DrawItem {
    id: FormId,
}

impl DrawItem {
    fn new(id: FormId) -> Self {
        Self {
            id
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_uv: [f32; 2],
}

impl Vertex {
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2 ];

    fn empty() -> Self {
        Self {
            position: [0., 0., 0.],
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
struct Style {
    dummy: u32
}

impl Style {
    const ATTRS: [wgpu::VertexAttribute; 1] =
        wgpu::vertex_attr_array![ 2 => Uint32
        ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Style>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRS,
        }
    }

    fn new() -> Self {
        Self {
            dummy: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CameraUniform {
    matrix: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            matrix: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    fn set(&mut self, mat: &Matrix4) {
        self.matrix = mat.into();
    }
}

//
// WGPU pipeline definition
//

fn form3d_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("form3d.wgsl"));

    let vertex_entry = "vs_form3d";
    let fragment_entry = "fs_form3d";

    let vertex_layout = Vertex::desc();
    let style_layout = Style::desc();

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

fn texture_bind_group(
    device: &wgpu::Device, 
    layout: &wgpu::BindGroupLayout,
    texture: &wgpu::Texture
) -> wgpu::BindGroup {
    let text_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    // wgpu::AddressMode::ClampToEdge
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        .. Default::default()
    });

    device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&text_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                }
            ],
            label: Some("draw3d texture bind group")
        }
    )
}
