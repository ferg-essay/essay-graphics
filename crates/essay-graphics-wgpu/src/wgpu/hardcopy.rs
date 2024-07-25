use std::{fs::File, io::BufWriter, ops::Deref};

use essay_graphics_api::renderer::{Drawable, Event};
use wgpu::BufferView;
use image::{ImageBuffer, Rgba};

use crate::{PlotCanvas, PlotRenderer};

pub struct WgpuHardcopy {
    device: wgpu::Device,
    queue: wgpu::Queue,

    canvas: PlotCanvas,

    texture: wgpu::Texture,
    // texture_format: wgpu::TextureFormat,
    texture_size: wgpu::Extent3d,
    bytes_per_row: u32,
    is_short_row: bool,
    surfaces: Vec<SurfaceItem>,
}

impl WgpuHardcopy {
    pub fn new(width: u32, height: u32) -> WgpuHardcopy {
        let (device, queue) = pollster::block_on(wgpu_device());

        let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;

        let texture_size = wgpu::Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: 1,
        };

        let u32_size = std::mem::size_of::<u32>() as u32;
        let bytes_per_row = u32_size * width;
        let is_short_row = bytes_per_row % 256 != 0;
        let bytes_per_row = bytes_per_row + (256 - bytes_per_row) % 256;

        let texture_desc = wgpu::TextureDescriptor {
            size: texture_size.clone(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: texture_format,
            usage: wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[texture_format],
            label: None,
        };
        
        let texture = device.create_texture(&texture_desc);

        let canvas = PlotCanvas::new(
            &device,
            &queue,
            texture_format,
            width,
            height
        );
    
        Self {
            device,
            queue,
            canvas,
            texture,
            // texture_format,
            texture_size,
            bytes_per_row,
            is_short_row,

            surfaces: Vec::new(),
        }
    }

    pub fn add_surface(&mut self) -> SurfaceId {
        /*
        let texture_desc = wgpu::TextureDescriptor {
            size: self.texture_size.clone(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.texture_format,
            usage: wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[self.texture_format],
            label: None,
        };
        
        let texture = self.device.create_texture(&texture_desc);
        */

        let id = SurfaceId(self.surfaces.len());


        let o_size = (self.bytes_per_row * self.texture_size.height) as wgpu::BufferAddress;

        let o_desc = wgpu::BufferDescriptor {
            size: o_size,
            usage: wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false,
        };

        let o_buffer = self.device.create_buffer(&o_desc);

        // let buffer_slice = o_buffer.slice(..);

        // let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        // buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        //     tx.send(result).unwrap();
        // });

        self.surfaces.push(SurfaceItem {
            // texture,
            buffer: o_buffer,
            // buffer_slice,
        });

        id
    }

    pub fn draw_and_read<R>(
        &mut self, 
        id: SurfaceId, 
        drawable: &mut dyn Drawable,
        fun: impl FnOnce(ImageBuffer::<Rgba<u8>, &[u8]>) -> R
    ) -> R {        
        self.draw(drawable);
        self.copy_into_buffer(id);
        self.read_into(id, fun)
    }

    pub fn draw(&mut self, drawable: &mut dyn Drawable) {
            /*
        let view = self.surfaces[id.0]
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        */
        let view = self.texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.clear_screen(&view);

        let pos = self.canvas.bounds().clone();
    
        let mut plot_renderer = PlotRenderer::new(
            &mut self.canvas, 
            &self.device, 
            Some(&self.queue), 
            Some(&view)
        );
    
        drawable.event(&mut plot_renderer, &Event::Resize(pos.clone()));
    
        drawable.draw(&mut plot_renderer).unwrap();
    }

    pub fn copy_into_buffer(
        &mut self, 
        id: SurfaceId, 
    ) {
        let o_buffer = &self.surfaces[id.0].buffer; // self.device.create_buffer(&o_desc);

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture, // surfaces[id.0].texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &o_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.bytes_per_row),
                    rows_per_image: Some(self.texture_size.height),
                }
            },
            self.texture_size,
        );
        
        self.queue.submit(Some(encoder.finish()));
    }

    pub fn read_into<R>(&mut self, id: SurfaceId, fun: impl FnOnce(ImageBuffer::<Rgba<u8>, &[u8]>) -> R) -> R {
        pollster::block_on(self.read_into_async(id, fun))
    }

    pub async fn read_into_async<R>(
        &mut self, 
        id: SurfaceId, 
        fun: impl FnOnce(ImageBuffer::<Rgba<u8>, &[u8]>) -> R
    ) -> R {
        let is_short_row = self.is_short_row;
        let bytes_per_row = self.bytes_per_row;
        let width = self.texture_size.width;
        let height = self.texture_size.height;

        let result = {
            let buffer = self.read_buffer_async(id).await;

            if is_short_row {
                let u32_size = std::mem::size_of::<u32>() as u32;
                let vec = short_buffer(
                    buffer.deref(), 
                    bytes_per_row as usize, 
                    (u32_size * width) as usize,
                    height as usize
                );

                fun(ImageBuffer::from_raw(width, height, vec.deref()).unwrap())
            } else {
                fun(ImageBuffer::from_raw(width, height, buffer.deref()).unwrap())
            }
        };

        self.surfaces[id.0].buffer.unmap();

        result
    }

    pub async fn read_buffer_async(&mut self, id: SurfaceId) -> BufferView {
        let o_buffer = &self.surfaces[id.0].buffer; // self.device.create_buffer(&o_desc);
        /*
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture, // surfaces[id.0].texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &o_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.bytes_per_row),
                    rows_per_image: Some(self.texture_size.height),
                }
            },
            self.texture_size,
        );
        
        self.queue.submit(Some(encoder.finish()));
        */

        {
            let buffer_slice = o_buffer.slice(..);

            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });

            self.device.poll(wgpu::Maintain::Wait);
            rx.receive().await.unwrap().unwrap();

            buffer_slice.get_mapped_range()
        }
    }

    pub fn save(
        &mut self, 
        _id: SurfaceId,
        _path: impl AsRef<std::path::Path>,
        _dpi: usize,
    ) {
        /*
        save_png(
            path, 
            self.texture_size.width, 
            self.texture_size.height, 
            dpi,
            &self.read_buffer(id),
        );
        */

        // pollster::block_on(self.extract_buffer(path, dpi));
    }

    /*
    async fn extract_buffer(
        &mut self,
        path: impl AsRef<std::path::Path>,
        dpi: usize
    ) {
        let u32_size = std::mem::size_of::<u32>() as u32;
        let o_size = (u32_size * self.texture_size.width * self.texture_size.height) as wgpu::BufferAddress;

        let o_desc = wgpu::BufferDescriptor {
            size: o_size,
            usage: wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false,
        };

        let o_buffer = self.device.create_buffer(&o_desc);

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.surfaces[0].texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &o_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(u32_size * self.texture_size.width),
                    rows_per_image: Some(self.texture_size.height),
                }
            },
            self.texture_size,
        );

        self.queue.submit(Some(encoder.finish()));

        {
            let buffer_slice = o_buffer.slice(..);

            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });

            self.device.poll(wgpu::Maintain::Wait);
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
                self.texture_size.width, 
                self.texture_size.height, 
                data
            ).unwrap();

            if true {
                _save_png(path, self.texture_size.width, self.texture_size.height, dpi, &buffer);
            } else {
                buffer.save(path).unwrap()
            }
        }
    }
    */

    fn clear_screen(&self, view: &wgpu::TextureView) {
        let mut encoder =
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
    }
}

fn short_buffer(buffer: &[u8], row_size: usize, width: usize, height: usize) -> Vec<u8> {
    let mut vec = Vec::<u8>::new();
    vec.resize(width * height, 0);

    let mut row = 0;
    for chunk in vec.chunks_mut(width) {
        chunk.copy_from_slice(&buffer[row..row + width]);

        row += row_size;
    }

    vec
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceId(usize);

struct SurfaceItem {
    // texture: wgpu::Texture,
    buffer: wgpu::Buffer,
}

async fn wgpu_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::default();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .expect("Failed to find adapter");

    adapter
        .request_device(&Default::default(), None)
        .await
        .expect("Failed to create device")
}

fn _save_png(
    path: impl AsRef<std::path::Path>, 
    width: u32, 
    height: u32, 
    dpi: usize, 
    data: &ImageBuffer<image::Rgba<u8>, wgpu::BufferView>,
) {
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let dpm = (39.370079 * dpi as f32).round() as u32;

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Best);
    encoder.set_pixel_dims(Some(png::PixelDimensions {
        xppu: dpm,
        yppu: dpm,
        unit: png::Unit::Meter,
    }));
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
}
