use std::time::{Duration, Instant};

use essay_graphics_api::{input::Input, renderer::{self, Drawable}};
use essay_graphics_winit::{run_event_loop, MainLoopHandle};
use winit::{event_loop::EventLoop, window::{CursorIcon, Window}};

use super::{render::render_draw, PlotCanvas};

pub struct WgpuMainLoop {
    title: Option<String>,
    delay: Option<Duration>,
}

impl WgpuMainLoop {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = Some(String::from(title));

        self
    }

    pub fn delay(&mut self, delay: Option<Duration>) -> &mut Self {
        self.delay = delay;

        self
    }

    pub fn main_loop(&mut self, draw: Box<dyn Drawable>) -> renderer::Result<()> {
        let event_loop = EventLoop::new().unwrap();
        let window = winit::window::Window::new(&event_loop).unwrap();

        if let Some(title) = &self.title {
            window.set_title(title);
        }

        window.set_cursor_icon(CursorIcon::Default);

        let wgpu_device = pollster::block_on(init_wgpu_device(&window));

        let mut handle = MainLoopData::new(wgpu_device, draw);

        handle.canvas.set_scale_factor(window.scale_factor() as f32);

        run_event_loop(event_loop, handle)
    }
}

impl Default for WgpuMainLoop {
    fn default() -> Self {
        Self { 
            title: None,
            delay: None,
        }
    }
}

struct MainLoopDevice<'window> {
    // instance: wgpu::Instance,
    // adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'window>,
    window: &'window Window,
}

struct MainLoopData<'window> {
    // instance: wgpu::Instance,
    // adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'window>,
    window: &'window Window,

    canvas: PlotCanvas,
    drawable: Box<dyn Drawable>,
}

impl<'window> MainLoopData<'window> {
    fn new(
        device: MainLoopDevice<'window>, 
        draw: Box<dyn Drawable>
    ) -> Self {
        let canvas = PlotCanvas::new(
            &device.device,
            &device.queue,
            device.config.format,
            device.config.width,
            device.config.height,
        );

        Self {
            // instance: device.instance,
            // adapter: device.adapter,
            device: device.device,
            queue: device.queue,
            config: device.config,
            surface: device.surface,
            window: device.window,

            canvas,
            drawable: draw,
        }
    }

    fn main_render(&mut self) {
        let frame = self.surface.get_current_texture()
            .expect("Failed to get next swap chain texture");
    
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
    
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
    
        let is_flush = true;
        render_draw(&mut self.canvas, &self.device, &self.queue, Some(&view), is_flush,
            |ui| {
                self.drawable.draw(ui)
        }).unwrap();
        //self.canvas.draw(self.drawable.as_mut(), &self.device, &self.queue, &view).unwrap();
    
        frame.present();
    }
}
/*
impl MainLoopHandle for MainLoopData<'_> {
    fn set_scale_factor(&mut self, scale_factor: f32) {
        self.canvas.set_scale_factor(scale_factor);
    }

    fn input_mut(&mut self) -> &mut Input {
        self.canvas.input_mut()
    }

    fn resized(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        // self.canvas.set_scale_factor(self.window.scale_factor() as f32);
        self.canvas.resize(&self.device, width, height);
        // canvas.set_scale_factor()
        self.canvas.request_redraw(true);
    }

    fn request_redraw(&mut self) {
        self.canvas.request_redraw(true);
    }

    fn about_to_wait(&mut self) -> renderer::Result<()> {
        if self.canvas.is_request_redraw() {
            self.canvas.request_redraw(false);

            self.main_render();
            self.input_mut().update_after_draw();
        }

        Ok(())
    }
}
*/
impl MainLoopHandle for MainLoopData<'_> {
    fn request_redraw(&mut self) {
        self.window.request_redraw();
    }

    fn input(&mut self, input: &Input) -> Option<Instant> {
        self.canvas.set_input(input);
        None
    }

    fn redraw(&mut self) -> renderer::Result<Option<Instant>> {
        self.main_render();

        Ok(None)
    }
}

async fn init_wgpu_device<'window>(window: &'window Window) -> MainLoopDevice<'window> {
    let size = window.inner_size();

    let instance = wgpu::Instance::default();

    let surface: wgpu::Surface<'window> = instance.create_surface(window).unwrap();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            },
        )
        .await
        .expect("Failed to create device");

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let texture_format = swapchain_capabilities.formats[0];

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: texture_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: Default::default(),
    };

    surface.configure(&device, &config);

    MainLoopDevice {
        device,
        queue,
        // instance,
        // adapter,
        surface,
        window,
        config,
    }
}

