use std::{sync::Arc, time::{Duration, Instant}};

use essay_graphics_api::{input::Input, output::Output, renderer::{self, App, Pos, Renderer}, Size};
use essay_graphics_winit::{run_event_loop, MainLoopHandle};
use wgpu::util::StagingBelt;
use winit::{event_loop::EventLoop, window::{CursorIcon, Window}};

use crate::{render::{render::{RenderWgpu, State}, RenderCanvas}, PlotRenderer};

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

    pub fn main_loop(&mut self, app: Box<dyn App>) -> renderer::Result<()> {
        let event_loop = EventLoop::new().unwrap();
        let window = winit::window::Window::new(&event_loop).unwrap();

        if let Some(title) = &self.title {
            window.set_title(title);
        }

        window.set_cursor_icon(CursorIcon::Default);

        let window = Arc::new(window);

        let viewport = WgpuViewport::from_window(window);

        /*
        let wgpu_device = pollster::block_on(init_wgpu_device(window.clone()));

        let mut handle = WgpuViewport::new(wgpu_device, app);

        handle.canvas.set_scale_factor(window.scale_factor() as f32);
        */

        let handle = WgpuHandle {
            viewport,
            app,
        };

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

/*
struct MainLoopDevice<'window> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'window>,
    window: &'window Window,

}
    */
struct MainLoopDevice {
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'static>,
    window: Arc<Window>,
}

struct WgpuHandle {
    viewport: WgpuViewport,
    app: Box<dyn App>,
}

impl MainLoopHandle for WgpuHandle {
    fn request_redraw(&mut self) {
        self.viewport.window.request_redraw();
    }

    fn input(&mut self, input: &Input) -> Option<Instant> {
        self.viewport.input = input.clone();

        None
    }

    fn redraw(&mut self) -> renderer::Result<Output> {
        let result = self.viewport.render(
            |ui| self.app.render(ui)
        )?;
        
        Ok(result.unwrap_or_else(|| Output::default()))
    }
}

pub struct WgpuViewport {
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'static>,
    window: Arc<Window>,

    canvas: RenderCanvas,
    input: Input,
}

impl WgpuViewport {
    pub fn from_window(window: Arc<Window>) -> Self {
        let wgpu_device = pollster::block_on(init_wgpu_device(window.clone()));

        let mut viewport = WgpuViewport::new(wgpu_device);

        viewport.canvas.set_scale_factor(window.scale_factor() as f32);

        viewport
    }

    fn new(
        device: MainLoopDevice, 
        // app: Box<dyn App>
    ) -> Self {
        let canvas = RenderCanvas::new(
            &device.device,
            &device.queue,
            device.config.format,
            device.config.width,
            device.config.height,
            device.window.scale_factor() as f32,
        );

        let mut input = Input::default();
        input.size = Size::new(device.config.width as f32, device.config.height as f32);

        Self {
            device: device.device,
            queue: device.queue,
            surface: device.surface,
            config: device.config,
            window: device.window,

            canvas,
            // app,
            input,
        }
    }

    pub fn window_clone(&self) -> Arc<Window> {
        self.window.clone()
    }

    pub fn render<R>(
        &mut self, 
        draw: impl FnOnce(&mut dyn Renderer) -> renderer::Result<R>
    ) -> renderer::Result<Option<R>> {
        let pos = Pos::from(self.input.size);
        // let pos = self.canvas.pos();

        if pos.width() == 0. {
            return Ok(None); // Output::default());
        }

        if pos != self.canvas.pos() {
            self.config.width = self.input.size.width as u32;
            self.config.height = self.input.size.height as u32;

            self.surface.configure(&self.device, &self.config);
        }

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
    
        let staging = StagingBelt::new(2048 * 128);

        let mut wgpu = RenderWgpu {
            device: &self.device,
            queue: &self.queue,
            view: &view,
            scissor: None,
            encoder: None,
            state: State::PreInit,
            bounds: pos,
            staging,
        };

        self.canvas.resize(&mut wgpu, pos);

        let result = PlotRenderer::render(
            &mut wgpu,
            &mut self.canvas,
            &self.input,
            /*
            |ui| {
                //self.app.render(ui)
                app.render(ui)
            }
            */
            draw
        ).unwrap();

        frame.present();

        Ok(Some(result))
    }
}

//async fn init_wgpu_device<'window>(window: &'window Window) -> MainLoopDevice<'window> {
async fn init_wgpu_device(window: Arc<Window>) -> MainLoopDevice {
    let size = window.inner_size();

    let instance = wgpu::Instance::default();

    let surface: wgpu::Surface = instance.create_surface(window.clone()).unwrap();

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
        surface,
        window,
        config,
    }
}

