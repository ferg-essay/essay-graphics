use essay_graphics_api::TextureId;


pub struct TextureCache {
    texture_items: Vec<TextureItem>,
}

impl TextureCache {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let mut textures = Self {
            texture_items: Vec::new(),
        };

        let (width, height) = (64, 4);

        let mut data = Vec::<u8>::new();
        data.resize(width * height * 4, 0xff);
        let id = textures.add_rgba_u8(device, queue, width as u32, height as u32, data.as_slice());

        assert_eq!(id, TextureId::default());

        textures
    }

    pub fn add_rgba_u8(
        &mut self, 
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32, 
        height: u32, 
        data: &[u8]
    ) -> TextureId {
        assert!(width * height * 4 == data.len() as u32);
        
        let id = TextureId::new(self.texture_items.len());
        
        let is_repeat = true;
        let address_mode = if is_repeat {
            wgpu::AddressMode::Repeat
        } else {
            wgpu::AddressMode::ClampToEdge
        };

        let mut item = TextureItem::new(
            device, 
            wgpu::TextureFormat::Rgba8Unorm,
            address_mode,
            width, 
            height
        );

        item.write(queue, width * 4, width, height, data);

        self.texture_items.push(item);

        id
    }
    
    pub(crate) fn texture_bind_group(&self, id: TextureId) -> &wgpu::BindGroup {
        &self.texture_items[id.index()].bind_group
    }
}

struct TextureItem {
    texture: wgpu::Texture,
    _format: wgpu::TextureFormat,
    _layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl TextureItem {
    fn new(
        device: &wgpu::Device, 
        format: wgpu::TextureFormat,
        address_mode: wgpu::AddressMode,
        width: u32, 
        height: u32
    ) -> Self {
        let texture = create_texture(device, format, width, height);
        let layout = create_bind_group_layout(device);
        let bind_group = create_bind_group(device, &layout, &texture, address_mode);

        Self {
            texture,
            _format: format,
            _layout: layout,
            bind_group,
        }
    }

    fn write(&mut self, queue: &wgpu::Queue, bytes_per_row: u32, width: u32, height: u32, data: &[u8]) {
        write_texture(
            queue, 
            &self.texture, 
            data,
            bytes_per_row,
            width,
            height,
        );
    }
}

fn create_texture(
    device: &wgpu::Device, 
    format: wgpu::TextureFormat, 
    width: u32, 
    height: u32
) -> wgpu::Texture {
    // wgpu::TextureFormat::Rgba8Unorm,
    //let format;
    device.create_texture(
        &wgpu::TextureDescriptor {
            size: texture_size(width, height),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING 
                | wgpu::TextureUsages::COPY_DST,
            label: None,
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
        label: Some("texture_bind_group_layout"),
    })
}

fn create_bind_group(
    device: &wgpu::Device, 
    layout: &wgpu::BindGroupLayout,
    texture: &wgpu::Texture,
    address_mode: wgpu::AddressMode,
) -> wgpu::BindGroup {
    let text_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    // wgpu::AddressMode::ClampToEdge
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: address_mode,
        address_mode_v: address_mode,
        //address_mode_u: wgpu::AddressMode::ClampToEdge,
        //address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        //mag_filter: wgpu::FilterMode::Linear,
        // use nearest for magnification for sharp edges
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
                    resource: wgpu::BindingResource::TextureView(&text_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                }
            ],
            label: Some("text_bind_group")
        }
    )
}

fn write_texture(
    queue: &wgpu::Queue, 
    texture: &wgpu::Texture, 
    data: &[u8], 
    bytes_per_row: u32,
    width: u32, 
    height: u32) {
    //assert!(width % 256 == 0);

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &data,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(bytes_per_row),
            rows_per_image: Some(height),
        },
        texture_size(width, height),
    );
}
