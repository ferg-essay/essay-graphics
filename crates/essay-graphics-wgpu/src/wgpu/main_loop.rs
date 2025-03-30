use std::time::Instant;

use essay_graphics_api::{
    input::Input,
    renderer::{Canvas, DeviceErr, Drawable}, 
    Bounds, Point, Size
};
use winit::{
    event::{self, ElementState, KeyEvent, MouseButton, WindowEvent }, 
    event_loop::{ControlFlow, EventLoop}, 
    keyboard::{Key, NamedKey}, 
    window::{CursorIcon, Window}
};

use crate::PlotCanvas;

use super::render::PlotRenderer;

pub struct WgpuMainLoop {
    title: Option<String>,
}

impl WgpuMainLoop {
    pub fn new() -> Self {
        Self {
            title: None,
        }
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = Some(String::from(title));

        self
    }

    pub fn main_loop(&mut self, draw: Box<dyn Drawable>) -> Result<(), DeviceErr> {
        let event_loop = EventLoop::new().unwrap();
        let window = winit::window::Window::new(&event_loop).unwrap();

        if let Some(title) = &self.title {
            window.set_title(title);
        }

        window.set_cursor_icon(CursorIcon::Default);

        let wgpu_device = pollster::block_on(init_wgpu_device(&window));
    
        run_event_loop(event_loop, window, wgpu_device, draw);

        Ok(())
    }
}

async fn init_wgpu_device(window: &Window) -> MainLoopDevice {
    let size = window.inner_size();

    let instance = wgpu::Instance::default();

    let surface = unsafe { instance.create_surface(&window) }.unwrap();

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
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
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
    };

    surface.configure(&device, &config);

    MainLoopDevice {
        device,
        queue,
        instance,
        adapter,
        surface,
        config,
    }
}

fn run_event_loop(
    event_loop: EventLoop<()>, 
    window: Window, 
    args: MainLoopDevice,
    drawable: Box<dyn Drawable>,
) {
    let mut handle = Box::new(MainLoopData::new(args, drawable));

    handle.set_scale_factor(window.scale_factor() as f32);

    // let mut cursor = CursorState::new();
    let mut size = Size(0., 0.);

    event_loop.run(move |event, window_target| {
        window_target.set_control_flow(ControlFlow::Wait);

        match event {
            event::Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                handle.resized(new_size.width, new_size.height);
                size = Size(new_size.width as f32, new_size.height as f32);
            }
            event::Event::WindowEvent {
                event: WindowEvent::MouseInput {
                    state,
                    button,
                    ..
                },
                ..
            } => {
                mouse_input(handle.input_mut(), &state, &button);
                handle.request_redraw();
            }
            event::Event::WindowEvent {
                event: WindowEvent::CursorMoved {
                    position,
                    ..
                },
                ..
            } => {
                let pos = Point(position.x as f32, size.height() - position.y as f32);
                
                handle.input_mut().cursor = Some(pos);
                handle.request_redraw();
            }
            event::Event::WindowEvent {
                event: WindowEvent::KeyboardInput { event, .. },
                ..
            } => {
                key_input(handle.input_mut(), &event);
            }
            event::Event::WindowEvent {
                event: WindowEvent::CursorEntered {
                    ..
                },
                ..
            } => {
            }
            event::Event::WindowEvent {
                event: WindowEvent::CursorLeft {
                    ..
                },
                ..
            } => {
            }
            event::Event::WindowEvent {
                event: WindowEvent::Focused(is_focus),
                ..
            } => {
                handle.input_mut().is_focus = is_focus;

                if ! is_focus {
                    handle.input_mut().cursor = None;
                }
            }
            event::Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                handle.request_redraw();
            },
            event::Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => window_target.exit(),
            event::Event::AboutToWait => {
                handle.about_to_wait();
            }
            _ => {}
        }
    }).unwrap();
}

struct MainLoopDevice {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface,
}

trait MainLoopHandle {
    fn set_scale_factor(&mut self, scale_factor: f32);

    fn resized(&mut self, width: u32, height: u32);

    fn input_mut(&mut self) -> &mut Input;

    fn request_redraw(&mut self);

    fn about_to_wait(&mut self);
}

struct MainLoopData {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface,

    canvas: PlotCanvas,
    drawable: Box<dyn Drawable>,
}

impl MainLoopData {
    fn new(device: MainLoopDevice, draw: Box<dyn Drawable>) -> Self {
        let canvas = PlotCanvas::new(
            &device.device,
            &device.queue,
            device.config.format,
            device.config.width,
            device.config.height,
        );

        Self {
            instance: device.instance,
            adapter: device.adapter,
            device: device.device,
            queue: device.queue,
            config: device.config,
            surface: device.surface,

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
    
        self.canvas.draw(self.drawable.as_mut(), &self.device, &self.queue, &view).unwrap();
    
        frame.present();
    }
}

impl MainLoopHandle for MainLoopData {
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

    fn about_to_wait(&mut self) {
        if self.canvas.is_request_redraw() {
            self.canvas.request_redraw(false);

            self.main_render();
            self.input_mut().update_after_draw();
        }
    }
}

fn mouse_input(
    input: &mut Input, 
    state: &ElementState, 
    button: &MouseButton
) {
    match button {
        MouseButton::Left => {
            match state {
                ElementState::Pressed => {
                    input.left_press = true;
                    input.left_click = true;
                }
                ElementState::Released => {
                    input.left_release = true;
                    input.left_press = false;
                }
            }
        },
        _ => {}
    }
}

fn key_input(
    _input: &mut Input, 
    _key: &KeyEvent,
) {
}

struct MouseState {
    left: ElementState,
    left_press_start: Point,
    left_press_last: Point,

    right: ElementState,
    right_press_start: Point,
    right_press_time: Instant,
}

impl MouseState {
    fn new() -> Self {
        Self {
            left: ElementState::Released,
            left_press_start: Point(0., 0.),
            left_press_last: Point(0., 0.),

            right: ElementState::Released,
            right_press_start: Point(0., 0.),
            right_press_time: Instant::now(),
        }
    }
}

struct CursorState {
    position: Point,
}

impl CursorState {
    fn new() -> Self {
        Self {
            position: Point(0., 0.),
        }
    }
}
