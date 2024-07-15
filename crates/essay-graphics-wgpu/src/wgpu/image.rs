use bytemuck_derive::{Pod, Zeroable};
use essay_graphics_api::{Bounds, Canvas, Affine2d, ImageId};
use essay_tensor::Tensor;
use wgpu::util::DeviceExt;


pub struct ImageRender {
    textures: Vec<RgbaTexture>,

    vertex_stride: usize,
    vertex_vec: Vec<ImageVertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_offset: usize,

    style_stride: usize,
    style_vec: Vec<ImageStyle>,
    style_buffer: wgpu::Buffer,
    style_offset: usize,

    texture_items: Vec<TextureItem>,

    image_id: usize,
    image_items: Vec<ImageItem>,

    pipeline: wgpu::RenderPipeline,
}

impl ImageRender {
    pub(crate) fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
    ) -> Self {
        let len = 2048;

        let mut vertex_vec = Vec::<ImageVertex>::new();
        vertex_vec.resize(len, ImageVertex { position: [0.0, 0.0], tex_coord: [0.0, 0.0] });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertex_vec.as_slice()),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let mut style_vec = Vec::<ImageStyle>::new();
        style_vec.resize(len, ImageStyle { 
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

        // let texture = RgbaTexture::new(device, width, height);

        //let text_shader = device.create_shader_module(wgpu::include_wgsl!("image.wgsl"));

        // let style_buffer = WgpuTextStyle::create_buffer(WgpuTextStyle::empty(), device);

        // let (style_layout, style_bind_group) = create_uniform_bind_group(device, &style_buffer);

        let pipeline = create_image_pipeline(
            device, 
            format,
        );
    
        Self {
            textures: Vec::new(), // ImageTexture::new(device, width, height),

            vertex_stride: std::mem::size_of::<ImageVertex>(),
            vertex_vec,
            vertex_buffer,
            vertex_offset: 0,

            style_stride: std::mem::size_of::<ImageStyle>(),
            style_vec,
            style_buffer,
            style_offset: 0,

            texture_items: Vec::new(),

            image_items: Vec::new(),
            image_id: 1,

            pipeline,
        }
    }

    pub fn clear(&mut self) {
        self.vertex_offset = 0;
        self.style_offset = 0;

         // TODO: check that this properly deallocates
        self.textures.retain(|texture| texture.image_id.is_live());
        //for texture in &mut self.textures {
        //    texture.is_stale = false;
        //}
    }

    pub(crate) fn create_image(&mut self, device: &wgpu::Device, colors: &Tensor<u8>) -> ImageId {
        let image_id = self.image_id;
        self.image_id += 1;

        let image = ImageId::new(image_id);

        let texture = RgbaTexture::new(
            device, 
            image.clone(),
            colors.dim(1) as u32, 
            colors.dim(0) as u32,
        );

        let tex_index = self.textures.len();
        self.textures.push(texture);
       
        self.texture_items.push(TextureItem {
            tex_index,
            image: colors.clone(),
        });

        image
    }

    pub(crate) fn _draw(
        &mut self, 
        _device: &wgpu::Device,
        _pos: &Bounds<Canvas>,
        _image: &Tensor<u8>,
        _affine: &Affine2d,
    ) {
        todo!();
        /*
        let start = self.vertex_offset;

        let x0 = pos.x0();
        let y0 = pos.y0();
        let x1 = pos.x1();
        let y1 = pos.y1();

        //let (x0, y0) = (50., 50.);
        //let (x1, y1) = (100., 100.);

        let (tx0, ty0) = (0., 0.);
        //let (tx1, ty1) = (image.rows() as f32, image.cols() as f32);
        let (tx1, ty1) = (1., 1.);

        self.vertex(x0, y0, tx0, ty0);
        self.vertex(x0, y1, tx0, ty1);
        self.vertex(x1, y1, tx1, ty1);

        self.vertex(x1, y1, tx1, ty1);
        self.vertex(x1, y0, tx1, ty0);
        self.vertex(x0, y0, tx0, ty0);

        let end = self.vertex_offset;

        let image_id = self.image_id;
        self.image_id += 1;

        let texture = RgbaTexture::new(
            device, 
            image,
            image.dim(1) as u32, 
            image.dim(0) as u32,
        );

        let tex_index = self.textures.len();
        self.textures.push(texture);
       
        self.image_items.push(ImageItem {
            // style: GpuTextStyle::new(&affine, color.get_srgba()),
            start,
            end,
            index: self.style_offset,
            tex_index,
            image: Some(image.clone()),
        });

        self.style_vec[self.style_offset] = ImageStyle::new(&affine, 0xff);
        self.style_offset += 1;
        */
    }

    pub fn draw_image(
        &mut self, 
        _device: &wgpu::Device,
        pos: &Bounds<Canvas>,
        image: &ImageId,
        affine: &Affine2d,
    ) {
        let start = self.vertex_offset;

        let x0 = pos.x0();
        let y0 = pos.y0();
        let x1 = pos.x1();
        let y1 = pos.y1();

        //let (x0, y0) = (50., 50.);
        //let (x1, y1) = (100., 100.);

        let (tx0, ty0) = (0., 0.);
        //let (tx1, ty1) = (image.rows() as f32, image.cols() as f32);
        let (tx1, ty1) = (1., 1.);

        self.vertex(x0, y0, tx0, ty0);
        self.vertex(x0, y1, tx0, ty1);
        self.vertex(x1, y1, tx1, ty1);

        self.vertex(x1, y1, tx1, ty1);
        self.vertex(x1, y0, tx1, ty0);
        self.vertex(x0, y0, tx0, ty0);

        let end = self.vertex_offset;

        //let image_id = self.image_id;
        //self.image_id += 1;

        let tex_index = self.textures.iter()
            .position(|tex| &tex.image_id == image)
            .unwrap();
       
        self.image_items.push(ImageItem {
            // style: GpuTextStyle::new(&affine, color.get_srgba()),
            start,
            end,
            index: self.style_offset,
            tex_index,
            image: None,
        });

        self.style_vec[self.style_offset] = ImageStyle::new(&affine, 0xff);
        self.style_offset += 1;
    }

    pub fn flush(
        &mut self, 
        queue: &wgpu::Queue, 
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        //self.text_cache.flush(queue, &self.texture);

        if self.image_items.len() == 0 {
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

        for item in self.texture_items.drain(..) {
            write_rgba_texture(
                queue, 
                &self.textures[item.tex_index], 
                &item.image, 
                item.image.dim(1) as u32, 
                item.image.dim(0) as u32
            );
        }

        for item in self.image_items.drain(..) {
            rpass.set_pipeline(&self.pipeline);

            let stride = self.vertex_stride;
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(
                (stride * item.start) as u64..(stride * item.end) as u64
            ));

            let stride = self.style_stride;
            rpass.set_vertex_buffer(1, self.style_buffer.slice(
                (stride * item.index) as u64..(stride * (item.index + 1)) as u64
            ));

            if let Some(_image) = item.image {
                todo!();
                //write_rgba_texture(queue, &self.textures[item.tex_index], &image, 
                //    image.dim(1) as u32, image.dim(0) as u32
                //);
            }

            rpass.set_bind_group(0, &self.textures[item.tex_index].bind_group, &[]);

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

        let vertex = ImageVertex::new(x, y, u, v);

        self.vertex_vec[self.vertex_offset] = vertex;
        self.vertex_offset += 1;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ImageVertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

impl ImageVertex {
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2 ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ImageVertex>() as wgpu::BufferAddress,
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

pub struct ImageItem {
    start: usize,
    end: usize,
    index: usize,

    tex_index: usize,
    image: Option<Tensor<u8>>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ImageStyle {
    affine_0: [f32; 4],
    affine_1: [f32; 4],
    color: [f32; 4],
}

impl ImageStyle {
    const ATTRS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![
            2 => Float32x4, 
            3 => Float32x4,
            4 => Float32x4
        ];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ImageVertex>() as wgpu::BufferAddress,
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

pub struct TextureItem {
    tex_index: usize,
    image: Tensor<u8>,
}

pub struct RgbaTexture {
    texture: wgpu::Texture,
    bind_group: wgpu::BindGroup,
    image_id: ImageId,
    is_stale: bool,
}

impl RgbaTexture {
    pub fn new(device: &wgpu::Device, image_id: ImageId, width: u32, height: u32) -> Self {
        let texture = create_rgba_texture(device, width, height);
        let layout = create_bind_group_layout(device);
        let bind_group = create_texture_bind_group(device, &layout, &texture);
        //let image_id = Image::new(id);

        Self {
            texture,
            bind_group,
            image_id,
            is_stale: true,
        }
    }
}

fn write_rgba_texture(
    queue: &wgpu::Queue, 
    texture: &RgbaTexture,
    data: &[u8], 
    width: u32, 
    height: u32) {

    if ! texture.is_stale {
        return;
    }

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture.texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        data,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height),
        },
        texture_size(width, height),
    );
}

fn create_rgba_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    device.create_texture(
        &wgpu::TextureDescriptor {
            size: texture_size(width, height),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            //format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING 
                | wgpu::TextureUsages::COPY_DST,
            label: Some("rgba_texture"),
            view_formats: &[],
        }
    )
}

fn texture_size(width: u32, height: u32) -> wgpu::Extent3d {
    wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    }
}

fn create_texture_bind_group(
    device: &wgpu::Device, 
    layout: &wgpu::BindGroupLayout,
    texture: &wgpu::Texture
) -> wgpu::BindGroup {
    let image_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    /*
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        .. Default::default()
    });
    */
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
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
                    resource: wgpu::BindingResource::TextureView(&image_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                }
            ],
            label: Some("image_bind_group")
        }
    )
}

fn create_image_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("image.wgsl"));

    let vertex_entry = "vs_image";
    let fragment_entry = "fs_image";

    let vertex_layout = ImageVertex::desc();
    let style_layout = ImageStyle::desc();

    let texture_layout = create_bind_group_layout(device);

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[
            &texture_layout,
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

fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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
        label: Some("bind_group_layout"),
    })
}
