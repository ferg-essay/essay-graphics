use bytemuck_derive::{Zeroable, Pod};
use essay_graphics_api::{Affine2d, Color, Point};
use wgpu::util::DeviceExt;

use super::{render::line_normal, texture_store::TextureCache};

pub struct Shape2dRender {
    vertex_stride: usize,
    vertex_vec: Vec<Shape2dVertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_offset: usize,

    style_stride: usize,
    style_vec: Vec<Shape2dStyle>,
    style_buffer: wgpu::Buffer,
    style_offset: usize,

    shape_items: Vec<Shape2dItem>,

    is_stale: bool,

    texture_cache: TextureCache,
    pipeline: wgpu::RenderPipeline,
}

impl Shape2dRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
    ) -> Self {
        let len = 2048;

        let mut vertex_vec = Vec::<Shape2dVertex>::new();
        vertex_vec.resize(len, Shape2dVertex { position: [0.0, 0.0] });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertex_vec.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let mut style_vec = Vec::<Shape2dStyle>::new();
        style_vec.resize(len, Shape2dStyle { 
            affine_0: [0.0, 0.0, 0.0, 0.0], 
            affine_1: [0.0, 0.0, 0.0, 0.0], 
            color: [0.0, 0.0, 0.0, 0.0],
        });

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
            vertex_stride: std::mem::size_of::<Shape2dVertex>(),
            vertex_vec,
            vertex_buffer,
            vertex_offset: 0,

            style_stride: std::mem::size_of::<Shape2dStyle>(),
            style_vec,
            style_buffer,
            style_offset: 0,
            // style_bind_group,

            is_stale: false,

            shape_items: Vec::new(),
            texture_cache: TextureCache::new(device, 512, 512),
            pipeline,
        }
    }

    //pub fn add_texture(&mut self, width: usize, height: usize, data: &[u8]) -> TextureId {
    //    self.texture_cache.add(width, height, data)
    //}

    pub fn clear(&mut self) {
        self.shape_items.drain(..);
        self.vertex_offset = 0;
        self.style_offset = 0;
    }

    pub fn start_shape(&mut self, clip: Option<[f32; 4]>) {
        let start = self.vertex_offset;

        self.shape_items.push(Shape2dItem {
            v_start: start,
            v_end: usize::MAX,
            s_start: self.style_offset,
            s_end: usize::MAX,
            clip,
        });
    }

    pub(crate) fn draw_line(
        &mut self, 
        b0: &Point,
        b1: &Point,
        lw2: f32,
    ) {
        let (nx, ny) = line_normal(*b0, *b1, lw2);

        self.vertex(b0.x() - nx, b0.y() + ny);
        self.vertex(b0.x() + nx, b0.y() - ny);
        self.vertex(b1.x() + nx, b1.y() - ny);

        self.vertex(b1.x() + nx, b1.y() - ny);
        self.vertex(b1.x() - nx, b1.y() + ny);
        self.vertex(b0.x() - nx, b0.y() + ny);
    }

    pub(crate) fn draw_triangle(
        &mut self, 
        p0: &Point,
        p1: &Point,
        p2: &Point
    ) {
        self.vertex(p0.x(), p0.y());
        self.vertex(p1.x(), p1.y());
        self.vertex(p2.x(), p2.y());
    }

    pub fn draw_style(
        &mut self, 
        color: Color,
        affine: &Affine2d,
    ) {
        let end = self.vertex_offset;

        let len = self.shape_items.len();

        if self.style_offset == self.style_vec.len() {
            self.is_stale = true;
            self.style_vec.resize(self.style_vec.len() + 2048, Shape2dStyle::empty());
        }

        let item = &mut self.shape_items[len - 1];
        item.v_end = end;

        self.style_vec[self.style_offset] = Shape2dStyle::new(affine, color);
        self.style_offset += 1;

        item.s_end = self.style_offset;
    }

    pub fn flush(
        &mut self, 
        device: &wgpu::Device,
        queue: &wgpu::Queue, 
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        scissor: Option<(u32, u32, u32, u32)>,
    ) {
        if self.shape_items.len() == 0 {
            return;
        }

        self.texture_cache.flush(queue);

        if false {
            self.texture_cache.bind_group();
            self.texture_cache.layout();
        }

        if self.is_stale {
            self.is_stale = false;
 
            self.vertex_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(self.vertex_vec.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
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
            &mut self.style_buffer, 
            0,
            bytemuck::cast_slice(self.style_vec.as_slice())
        );

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

        rpass.set_pipeline(&self.pipeline);

        if let Some((x, y, w, h)) = scissor {
            rpass.set_scissor_rect(x, y, w, h);
        }

        for item in self.shape_items.drain(..) {
            if item.v_start < item.v_end && item.s_start < item.s_end {
                if let Some([x, y, w, h]) = item.clip {
                    rpass.set_viewport(x, y, w, h, f32::MIN, f32::MAX);
                }

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

        self.vertex_offset = 0;
    }

    fn vertex(&mut self, x: f32, y: f32) {
        //let x = x.round();
        //let y = y.round();

        let vertex = Shape2dVertex { position: [x, y] };

        let len = self.vertex_vec.len();
        let offset = self.vertex_offset;

        if offset == len {
            self.is_stale = true;
            self.vertex_vec.resize(len + 2048, Shape2dVertex::empty());
        }


        self.vertex_vec[self.vertex_offset] = vertex;
        self.vertex_offset += 1;
    }
}

pub struct Shape2dItem {
    v_start: usize,
    v_end: usize,

    s_start: usize,
    s_end: usize,

    clip: Option<[f32; 4]>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Shape2dVertex {
    position: [f32; 2],
}

impl Shape2dVertex {
    const ATTRS: [wgpu::VertexAttribute; 1] =
        wgpu::vertex_attr_array![0 => Float32x2 ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Shape2dVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }

    fn empty() -> Shape2dVertex {
        Self {
            position: [0., 0.],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Shape2dStyle {
    affine_0: [f32; 4],
    affine_1: [f32; 4],
    color: [f32; 4],
}

impl Shape2dStyle {
    const ATTRS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![
            1 => Float32x4, 
            2 => Float32x4,
            3 => Float32x4
        ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Shape2dStyle>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRS,
        }
    }

    pub fn empty() -> Shape2dStyle {
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
    let shader = device.create_shader_module(wgpu::include_wgsl!("shape2d.wgsl"));

    let vertex_entry = "vs_shape";
    let fragment_entry = "fs_shape";

    let vertex_layout = Shape2dVertex::desc();
    let style_layout = Shape2dStyle::desc();

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
