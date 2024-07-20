use std::time::Instant;

use essay_graphics_api::{driver::{DeviceErr, Drawable}, Bounds, Canvas, CanvasEvent, Point};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent }, 
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

    pub fn main_loop(&mut self, drawable: Box<dyn Drawable>) -> Result<(), DeviceErr> {
        let event_loop = EventLoop::new().unwrap();
        let window = winit::window::Window::new(&event_loop).unwrap();

        if let Some(title) = &self.title {
            window.set_title(title);
        }

        window.set_cursor_icon(CursorIcon::Default);

        let wgpu_device = pollster::block_on(init_wgpu_device(&window));
    
        run_event_loop(event_loop, window, wgpu_device, drawable);

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

struct MainLoopDevice {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface,
}

fn run_event_loop(
    event_loop: EventLoop<()>, 
    window: Window, 
    args: MainLoopDevice,
    drawable: Box<dyn Drawable>,
) {
    let MainLoopDevice {
        instance,
        adapter,
        mut config,
        device,
        surface,
        queue,
    } = args;

    let mut drawable = drawable;

    let mut canvas = PlotCanvas::new(
        &device,
        &queue,
        config.format,
        config.width,
        config.height,
    );

    let pan_min = 20.;
    let zoom_min = 20.;

    // TODO: is double clicking not recommended?
    let dbl_click = 500; // time in millis

    let mut cursor = CursorState::new();
    let mut mouse = MouseState::new();

    event_loop.run(move |event, window_target| {
        let _ = (&instance, &adapter, &drawable);

        // let mut renderer = PlotRenderer::new(&mut canvas, &device, Some(&queue), None);

        window_target.set_control_flow(ControlFlow::Wait);
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                config.width = size.width;
                config.height = size.height;
                surface.configure(&device, &config);
                // figure_renderer.set_canvas_bounds(config.width, config.height);
                let bounds = Bounds::<Canvas>::from([size.width as f32, size.height as f32]);
                // drawable.update(&mut renderer, &bounds);
                canvas.resize(&device, size.width, size.height);
                canvas.request_redraw(true);
                let mut renderer = PlotRenderer::new(&mut canvas, &device, Some(&queue), None);
                drawable.event(&mut renderer, &CanvasEvent::Resize(bounds));
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput {
                    state,
                    button,
                    ..
                },
                ..
            } => {
                let mut renderer = PlotRenderer::new(&mut canvas, &device, Some(&queue), None);
                match button {
                    MouseButton::Left => {
                        mouse.left = state;

                        if state == ElementState::Pressed {
                            drawable.event(
                                &mut renderer,
                                &CanvasEvent::MouseLeftPress(cursor.position),
                            );
                            let now = Instant::now();

                            if now.duration_since(mouse.left_press_time).as_millis() < dbl_click {
                                drawable.event(
                                    &mut renderer,
                                    &CanvasEvent::ResetView(cursor.position),
                                )
                            }

                            mouse.left_press_start = cursor.position;
                            mouse.left_press_last = cursor.position;
                            mouse.left_press_time = now;
                            window.set_cursor_icon(CursorIcon::Grab);
                        } else {
                            window.set_cursor_icon(CursorIcon::Default);
                        }
                    },
                    MouseButton::Right => {
                        mouse.right = state;

                        match state {
                            ElementState::Pressed => {
                                drawable.event(
                                    &mut renderer,
                                    &CanvasEvent::MouseRightPress(cursor.position),
                                );

                                mouse.right_press_start = cursor.position;
                                mouse.right_press_time = Instant::now();
                                window.set_cursor_icon(CursorIcon::Crosshair);
                            }
                            ElementState::Released => {
                                drawable.event(
                                    &mut renderer,
                                    &CanvasEvent::MouseRightRelease(cursor.position),
                                );

                                if zoom_min <= mouse.right_press_start.dist(&cursor.position) {
                                    drawable.event(
                                        &mut renderer,
                                        &CanvasEvent::ZoomBounds(
                                            mouse.right_press_start, 
                                            cursor.position
                                        )
                                    );
                                }
                                window.set_cursor_icon(CursorIcon::Default);
                            }
                        }
                    },
                    _ => {}
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved {
                    position,
                    ..
                },
                ..
            } => {
                cursor.position = Point(position.x as f32, config.height as f32 - position.y as f32);
                let mut renderer = PlotRenderer::new(&mut canvas, &device, Some(&queue), None);

                if mouse.left == ElementState::Pressed 
                    && pan_min <= mouse.left_press_start.dist(&cursor.position) {
                    drawable.event(
                        &mut renderer,
                        &CanvasEvent::Pan(
                            mouse.left_press_start, 
                            mouse.left_press_last, 
                            cursor.position
                        ),
                    );

                    mouse.left_press_last = cursor.position;
                }
                if mouse.right == ElementState::Pressed
                    && pan_min <= mouse.left_press_start.dist(&cursor.position) {
                        drawable.event(
                            &mut renderer,
                            &CanvasEvent::MouseRightDrag(mouse.left_press_start, cursor.position),
                    );
                }
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { event, .. },
                ..
            } => {
                let mut renderer = PlotRenderer::new(&mut canvas, &device, Some(&queue), None);
                if event.state == ElementState::Pressed {
                    let pos = Point(0., 0.);

                    match event.logical_key {
                        Key::Character(key) => {
                            let ch = key.chars().next().unwrap();
                            drawable.event(
                                &mut renderer,
                                &CanvasEvent::KeyPress(pos, ch)
                            );
                        }
                        Key::Named(NamedKey::Space) => {
                            // TODO: replace with KeyPressNamed
                            drawable.event(
                                &mut renderer,
                                &CanvasEvent::KeyPress(pos, ' ')
                            );
                        },
                        Key::Named(NamedKey::Tab) => {
                            // TODO: replace with KeyPressNamed
                            drawable.event(
                                &mut renderer,
                                &CanvasEvent::KeyPress(pos, '\r')
                            );
                        },
                        Key::Named(NamedKey::Enter) => {
                            // TODO: replace with KeyPressNamed
                            drawable.event(
                                &mut renderer,
                                &CanvasEvent::KeyPress(pos, '\n')
                            );
                        },
                        Key::Named(_) => {},
                        Key::Unidentified(_) => {},
                        Key::Dead(_) => {},
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                canvas.request_redraw(true);
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => window_target.exit(),
            Event::AboutToWait => {
                if canvas.is_request_redraw() {
                    canvas.request_redraw(false);

                    main_render(&device, &queue, &surface, &mut canvas, &mut drawable);
                }
            }
            _ => {}
        }
    }).unwrap();
}

struct MouseState {
    left: ElementState,
    left_press_start: Point,
    left_press_last: Point,
    left_press_time: Instant,

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
            left_press_time: Instant::now(),

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

fn main_render(
    device: &wgpu::Device,
    queue: &wgpu::Queue, 
    surface: &wgpu::Surface,
    canvas: &mut PlotCanvas,
    drawable: &mut Box<dyn Drawable>
) {
    let frame = surface.get_current_texture()
        .expect("Failed to get next swap chain texture");

    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

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

    queue.submit(Some(encoder.finish()));

    canvas.draw(drawable, device, queue, &view);

    frame.present();
}
