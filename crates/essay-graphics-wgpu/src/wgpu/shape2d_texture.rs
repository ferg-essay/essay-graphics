use std::{collections::HashMap, ops::{Index, IndexMut}};

use bytemuck_derive::{Zeroable, Pod};
use essay_graphics_api::{Affine2d, Color, Hatch, Point, TextureId};
use wgpu::util::DeviceExt;

use super::{canvas::line_normal, texture_store::TextureCache};

pub struct Shape2dTextureRender {
    vertex_stride: usize,
    vertex_vec: Vec<Vertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_offset: usize,

    style_stride: usize,
    style_vec: Vec<Style>,
    style_buffer: wgpu::Buffer,
    style_offset: usize,

    shape_items: Vec<Item>,

    is_stale: bool,

    texture_cache: TextureCache,
    hatch_map: HashMap<Hatch, TextureId>,
    
    pipeline: wgpu::RenderPipeline,
}

impl Shape2dTextureRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Self {
        let len = 2048;

        let mut vertex_vec = Vec::<Vertex>::new();
        vertex_vec.resize(len, Vertex { 
            position: [0.0, 0.0],
            texture_uv: [0., 0.],
         });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertex_vec.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let mut style_vec = Vec::<Style>::new();
        style_vec.resize(len, Style { 
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

        let mut textures = TextureCache::new();
        let hatch_map = init_hatch(device, queue, &mut textures);

        let pipeline = create_shape2d_pipeline(
            device, 
            format,
            &textures,
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
            // style_bind_group,

            is_stale: false,

            shape_items: Vec::new(),
            texture_cache: textures,
            hatch_map: hatch_map,
            pipeline,
        }
    }

    pub fn hatch_texture(&self, hatch: Hatch) -> TextureId {
        *self.hatch_map.get(&hatch).unwrap()
    }

    fn texture_bind_map(&self, id: TextureId) -> &wgpu::BindGroup {
        self.texture_cache.texture_bind_group(id)
    }

    pub fn clear(&mut self) {
        self.shape_items.drain(..);
        self.vertex_offset = 0;
        self.style_offset = 0;
    }

    pub fn start_shape(&mut self, texture: TextureId, clip: Option<[f32; 4]>) {
        let start = self.vertex_offset;

        self.shape_items.push(Item {
            v_start: start,
            v_end: usize::MAX,
            s_start: self.style_offset,
            s_end: usize::MAX,
            texture,
            clip,
        });
    }

    pub(crate) fn _draw_line(
        &mut self, 
        b0: &Point,
        b1: &Point,
        lw2: f32,
    ) {
        let (nx, ny) = line_normal(*b0, *b1, lw2);

        self._vertex(b0.x() - nx, b0.y() + ny);
        self._vertex(b0.x() + nx, b0.y() - ny);
        self._vertex(b1.x() + nx, b1.y() - ny);

        self._vertex(b1.x() + nx, b1.y() - ny);
        self._vertex(b1.x() - nx, b1.y() + ny);
        self._vertex(b0.x() - nx, b0.y() + ny);
    }

    pub(crate) fn draw_triangle(
        &mut self, 
        p0: &Point,
        p1: &Point,
        p2: &Point
    ) {
        //self.vertex(p0.x(), p0.y());
        //self.vertex(p1.x(), p1.y());
        //self.vertex(p2.x(), p2.y());
        //println!("Tri {:?} {:?} {:?}", p0, p1, p2);
        //self.vertex_uv(p0.x(), p0.y(), 0., 0.);
        //self.vertex_uv(p1.x(), p1.y(), 0., 10.);
        //self.vertex_uv(p2.x(), p2.y(), 10., 0.);

        let f = 1. / 64.;

        self.vertex_uv(p0.x(), p0.y(), p0.x() * f, p0.y() * f);
        self.vertex_uv(p1.x(), p1.y(), p1.x() * f, p1.y() * f);
        self.vertex_uv(p2.x(), p2.y(), p2.x() * f, p2.y() * f);
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
            self.style_vec.resize(self.style_vec.len() + 2048, Style::empty());
        }

        let item = &mut self.shape_items[len - 1];
        item.v_end = end;

        self.style_vec[self.style_offset] = Style::new(affine, color);
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

        // self.texture_cache.flush(queue);

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

        {
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

        let items : Vec<Item> = self.shape_items.drain(..).collect();
        for item in items {
            if item.v_start < item.v_end && item.s_start < item.s_end {
                if let Some([x, y, w, h]) = item.clip {
                    rpass.set_viewport(x, y, w, h, f32::MIN, f32::MAX);
                }

                rpass.set_bind_group(0, self.texture_bind_map(item.texture), &[]);
                
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
        }

        self.vertex_offset = 0;
    }

    fn _vertex(&mut self, x: f32, y: f32) {
        //let x = x.round();
        //let y = y.round();

        let vertex = Vertex { 
            position: [x, y],
            texture_uv: [x, y],
        };

        let len = self.vertex_vec.len();
        let offset = self.vertex_offset;

        if offset == len {
            self.is_stale = true;
            self.vertex_vec.resize(len + 2048, Vertex::empty());
        }


        self.vertex_vec[self.vertex_offset] = vertex;
        self.vertex_offset += 1;
    }

    fn vertex_uv(&mut self, x: f32, y: f32, u: f32, v: f32) {
        //let x = x.round();
        //let y = y.round();

        let vertex = Vertex { 
            position: [x, y],
            texture_uv: [u, v],
        };

        let len = self.vertex_vec.len();
        let offset = self.vertex_offset;

        if offset == len {
            self.is_stale = true;
            self.vertex_vec.resize(len + 2048, Vertex::empty());
        }


        self.vertex_vec[self.vertex_offset] = vertex;
        self.vertex_offset += 1;
    }
}

pub struct Item {
    v_start: usize,
    v_end: usize,

    s_start: usize,
    s_end: usize,

    clip: Option<[f32; 4]>,
    texture: TextureId,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    texture_uv: [f32; 2],
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

    fn empty() -> Vertex {
        Self {
            position: [0., 0.],
            texture_uv: [0., 0.],
        }
    }
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

fn init_hatch(device: &wgpu::Device, queue: &wgpu::Queue, textures: &mut TextureCache) -> HashMap<Hatch, TextureId> {
    let mut hatch_map = HashMap::new();

    hatch_map.insert(
        Hatch::Vertical, 
        hatch_vertical(device, queue, textures)
    );
    hatch_map.insert(
        Hatch::Horizontal, 
        hatch_horizontal(device, queue, textures)
    );

    hatch_map
}

fn hatch_vertical(
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    textures: &mut TextureCache
) -> TextureId {
    let mut builder = HatchBuilder::new(64, 64);

    for j in 0..64 {
        builder[(0, j)] = 255;
        builder[(1, j)] = 255;
        builder[(2, j)] = 255;
        builder[(3, j)] = 255;

        builder[(32, j)] = 255;
        builder[(33, j)] = 255;
        builder[(34, j)] = 255;
        builder[(35, j)] = 255;
    }

    builder.add_to(device, queue, textures)

}

fn hatch_horizontal(
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    textures: &mut TextureCache
) -> TextureId {
    let mut builder = HatchBuilder::new(64, 64);

    for j in 0..64 {
        builder[(j, 0)] = 255;
        builder[(j, 1)] = 255;
        builder[(j, 2)] = 255;
        builder[(j, 3)] = 255;

        builder[(j, 32)] = 255;
        builder[(j, 33)] = 255;
        builder[(j, 34)] = 255;
        builder[(j, 35)] = 255;
    }

    //for j in 17..32 {
    //    builder[(j, 0)] = 255;
    //}

    builder.add_to(device, queue, textures)
}



struct HatchBuilder {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl HatchBuilder {
    fn new(width: usize, height: usize) -> Self {
        let mut vec = Vec::new();

        vec.resize((width * height) as usize, 0);

        Self {
            data: vec,
            width,
            height
        }
    }

    fn add_to(
        self, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        textures: &mut TextureCache
    ) -> TextureId {
        textures.add_r_u8(
            device, 
            queue, 
            self.width as u32, 
            self.height as u32, 
            self.as_slice()
        )
    }

    fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    fn _set(&mut self, x: usize, y: usize, v: u8) -> &mut Self {
        self.data[x + y * self.width] = v;

        self
    }
}

impl Index<(usize, usize)> for HatchBuilder {
    type Output = u8;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 + index.1 * self.width]
    }
}

impl IndexMut<(usize, usize)> for HatchBuilder {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 + index.1 * self.width]
    }
}

fn create_shape2d_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    texture: &TextureCache,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("shape2d_texture.wgsl"));

    let vertex_entry = "vs_shape_tex";
    let fragment_entry = "fs_shape_tex";

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
        label: Some("texture_bind_group_layout"),
    })
}
