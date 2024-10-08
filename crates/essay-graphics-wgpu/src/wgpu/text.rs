use bytemuck_derive::{Zeroable, Pod};
use essay_graphics_api::{Point, Color, Affine2d, HorizAlign, VertAlign};
use wgpu::util::DeviceExt;

use super::{text_texture::TextTexture, text_cache::{TextCache, FontId}};

pub struct TextRender {
    texture: TextTexture,
    text_cache: TextCache,

    vertex_stride: usize,
    vertex_vec: Vec<TextVertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_offset: usize,

    style_stride: usize,
    style_vec: Vec<GpuTextStyle>,
    style_buffer: wgpu::Buffer,
    style_offset: usize,

    text_items: Vec<TextItem>,

    pipeline: wgpu::RenderPipeline,
}

impl TextRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
        width: u32, 
        height: u32
    ) -> Self {
        let len = 2048;

        let mut vertex_vec = Vec::<TextVertex>::new();
        vertex_vec.resize(len, TextVertex { position: [0.0, 0.0], tex_coord: [0.0, 0.0] });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertex_vec.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let mut style_vec = Vec::<GpuTextStyle>::new();
        style_vec.resize(len, GpuTextStyle { 
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

        let texture = TextTexture::new(device, width, height);

        let text_shader = device.create_shader_module(wgpu::include_wgsl!("text.wgsl"));

        // let style_buffer = WgpuTextStyle::create_buffer(WgpuTextStyle::empty(), device);

        // let (style_layout, style_bind_group) = create_uniform_bind_group(device, &style_buffer);

        let pipeline = create_text_pipeline(
            device, 
            &text_shader,
            "vs_text",
            "fs_text",
            format,
            TextVertex::desc(),
            GpuTextStyle::desc(),
            // style_layout,
            &texture,
        );
    
        Self {
            texture: TextTexture::new(device, width, height),
            text_cache: TextCache::new(width, height),

            vertex_stride: std::mem::size_of::<TextVertex>(),
            vertex_vec,
            vertex_buffer,
            vertex_offset: 0,

            style_stride: std::mem::size_of::<GpuTextStyle>(),
            style_vec,
            style_buffer,
            style_offset: 0,

            text_items: Vec::new(),
            pipeline,
        }
    }

    pub fn clear(&mut self) {
        self.vertex_offset = 0;
        self.style_offset = 0;
    }

    ///
    /// load a font
    ///
    pub fn font(&mut self, font_name: &str) -> FontId {
        self.text_cache.font_id(font_name)
    }

    ///
    /// draw a text item
    /// 
    pub fn draw(
        &mut self, 
        text: &str, 
        font_id: FontId, 
        size: f32,
        pos: Point, 
        bounds: Point,
        color: Color,
        angle: f32,
        halign: HorizAlign,
        valign: VertAlign,
    ) {
        // let font = self.text_cache.font(font_name);
        // let font_id = self.text_cache.font_id(font_name);

        let x0 = pos.x();
        let y0 = pos.y();

        let start = self.vertex_offset;

        // TODO: proper spacing and kerning
        let text_size = (size + 0.5) as u16;

        let s = self.text_cache.glyph(font_id, text_size, ' ');
        let w_space = s.w + s.dx.max(0.);
        let w_inside = w_space * 0.3;

        // let w_inside = size * 0.07;
        let w_space = size * 0.4;
        
        let mut x = x0;
        let y = y0.round();
        for ch in text.chars() {
            let r = self.text_cache.glyph(font_id, text_size, ch);
            
            x = x.round();

            if r.is_none() || ch == ' ' {
                x += w_space;
                continue;
            }

            let y_ch = y + r.dy;// - r.h as f32;
            let x_ch = x; //  + r.dx;

            let w = r.w;
            let h = r.h;

            self.vertex(x_ch, y_ch, r.tx_min, r.ty_min);
            self.vertex(x_ch + w, y_ch, r.tx_max, r.ty_min);
            self.vertex(x_ch + w, y_ch + h, r.tx_max, r.ty_max);

            self.vertex(x_ch + w, y_ch + h, r.tx_max, r.ty_max);
            self.vertex(x_ch, y_ch + h, r.tx_min, r.ty_max);
            self.vertex(x_ch, y_ch, r.tx_min, r.ty_min);

            x += w + w_inside;
        }

        let dx = match halign {
            HorizAlign::Left => 0.,
            HorizAlign::Center => - 0.5 * (x - x0),
            HorizAlign::Right => - (x - x0),
        };

        let descent = 0.;

        let dy = match valign {
            VertAlign::Top => - size - descent,
            VertAlign::Center => - 0.5 * (size + descent),
            VertAlign::BaselineBottom => 0.,
            VertAlign::Bottom => - descent,
        };

        let end = self.vertex_offset;
        let affine = Affine2d::eye()
            .rotate_around(0.5 * (x0 + x), y0, angle)
            .translate(dx, dy)
            .scale(2. / bounds.x(), 2. / bounds.y())
            .translate(-1., -1.);

        
        self.text_items.push(TextItem {
            // style: GpuTextStyle::new(&affine, color.get_srgba()),
            start,
            end,
            index: self.style_offset,
        });
        self.style_vec[self.style_offset] = GpuTextStyle::new(&affine, color.to_rgba());
        self.style_offset += 1;
    }

    pub fn flush(
        &mut self, 
        queue: &wgpu::Queue, 
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        self.text_cache.flush(queue, &self.texture);

        if self.text_items.len() == 0 {
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

        for item in self.text_items.drain(..) {
            rpass.set_pipeline(&self.pipeline);

            let stride = self.vertex_stride;
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(
                (stride * item.start) as u64..(stride * item.end) as u64
            ));

            let stride = self.style_stride;
            rpass.set_vertex_buffer(1, self.style_buffer.slice(
                (stride * item.index) as u64..(stride * (item.index + 1)) as u64
            ));

            rpass.set_bind_group(0, self.texture.bind_group(), &[]);

            rpass.draw(
                0..(item.end - item.start) as u32,
                0..1,
            );

        }

        self.vertex_offset = 0;
    }

    fn vertex(&mut self, x: f32, y: f32, u: f32, v: f32) {
        // TODO: if_snap
        let x = x.round();
        let y = y.round();

        let vertex = TextVertex::new(x, y, u, v);

        self.vertex_vec[self.vertex_offset] = vertex;
        self.vertex_offset += 1;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TextVertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

impl TextVertex {
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2 ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }

    fn new(x: f32, y: f32, u: f32, v: f32) -> Self {
        Self {
            position: [x, y],
            tex_coord: [u, v],
        }
    }
}

pub struct TextItem {
    //style: GpuTextStyle,
    start: usize,
    end: usize,
    index: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct GpuTextStyle {
    affine_0: [f32; 4],
    affine_1: [f32; 4],
    color: [f32; 4],
}

impl GpuTextStyle {
    const ATTRS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![
            2 => Float32x4, 
            3 => Float32x4,
            4 => Float32x4
        ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRS,
        }
    }

    fn new(affine: &Affine2d, color: u32) -> Self {
        let mat = affine.mat();

        Self {
            affine_0: [mat[0], mat[1], 0., mat[2]],
            affine_1: [mat[3], mat[4], 0., mat[5]],
            color: [
                ((color >> 24) & 0xff) as f32 / 255.,
                ((color >> 16) & 0xff) as f32 / 255.,
                ((color >> 8) & 0xff) as f32 / 255.,
                ((color) & 0xff) as f32 / 255.,
            ],
        }
    }
}

fn create_text_pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    // pipeline_layout: &wgpu::PipelineLayout,
    vertex_entry: &str,
    fragment_entry: &str,
    format: wgpu::TextureFormat,
    vertex_layout: wgpu::VertexBufferLayout,
    style_layout: wgpu::VertexBufferLayout,
    // style_layout: wgpu::BindGroupLayout,
    texture: &TextTexture,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[
            texture.layout(),
            // &style_layout,
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
