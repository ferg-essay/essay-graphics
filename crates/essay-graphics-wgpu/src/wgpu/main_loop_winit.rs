use std::{sync::{Arc, Mutex}, time::Instant};

use essay_graphics_api::{
    input::{Event, Input, Key},
    renderer,
    Point, Size
};
use winit::{
    event::{self, ElementState, MouseButton, StartCause, WindowEvent }, 
    event_loop::{ControlFlow, EventLoop}, keyboard::{self, NamedKey}, 
};

pub trait MainLoopHandle {
    fn set_scale_factor(&mut self, scale_factor: f32);

    fn resized(&mut self, width: u32, height: u32);

    fn input_mut(&mut self) -> &mut Input;

    fn request_redraw(&mut self);

    fn about_to_wait(&mut self) -> renderer::Result<()>;

    fn get_wait_until(&mut self) -> Option<Instant> {
        None
    }
}

pub fn run_event_loop(
    event_loop: EventLoop<()>, 
    mut handle: impl MainLoopHandle,
) -> renderer::Result<()> {
    let result = Arc::new(Mutex::new(ResultHandle::default()));
    let result_handle = result.clone();
    // let mut cursor = CursorState::new();
    let mut size = Size(0., 0.);
    let mut wait_until: Option<Instant> = None;

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
                if let Some(key) = map_key(&event.logical_key) {
                    if event.state == ElementState::Pressed {
                        handle.input_mut().event(Event::KeyPress(key));
                    } else if event.state == ElementState::Released {
                        handle.input_mut().event(Event::KeyRelease(key));
                    }
                }
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
            event::Event::NewEvents(StartCause::ResumeTimeReached { .. }) 
                | event::Event::AboutToWait => {
                let now = Instant::now();
                let start_time = wait_until.unwrap_or(now);

                if start_time <= now {
                    if let Err(err) = handle.about_to_wait() {
                        result_handle.lock().unwrap().err = Some(err);
                        window_target.exit();
                    };
                }

                if let Some(next_wait_until) = handle.get_wait_until() {
                    wait_until = Some(next_wait_until);
                    window_target.set_control_flow(ControlFlow::WaitUntil(next_wait_until));
                }
            }
            _ => {}
        }
    }).unwrap();

    let err = result.lock().unwrap().take();

    if let Some(err) = err {
        Err(err)
    } else {
        Ok(())
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

fn map_key(key: &keyboard::Key) -> Option<Key> {
    match key {
        keyboard::Key::Named(named_key) => {
            match named_key {
                NamedKey::Space => Some(Key::Space),
                _ => None,
            }
        }
        keyboard::Key::Character(key) => {
            match key.as_str() {
                " " => Some(Key::Space),

                "a" => Some(Key::A),
                "b" => Some(Key::B),
                "c" => Some(Key::C),
                "d" => Some(Key::D),
                "e" => Some(Key::E),
                "f" => Some(Key::F),
                "g" => Some(Key::G),
                "h" => Some(Key::H),
                "i" => Some(Key::I),
                "j" => Some(Key::J),
                "k" => Some(Key::K),
                "l" => Some(Key::L),
                "m" => Some(Key::M),
                "n" => Some(Key::N),
                "o" => Some(Key::O),
                "p" => Some(Key::P),
                "q" => Some(Key::Q),
                "r" => Some(Key::R),
                "s" => Some(Key::S),
                "t" => Some(Key::T),
                "u" => Some(Key::U),
                "v" => Some(Key::V),
                "w" => Some(Key::W),
                "x" => Some(Key::X),
                "y" => Some(Key::Y),
                "z" => Some(Key::Z),

                "0" => Some(Key::N0),
                "1" => Some(Key::N1),
                "2" => Some(Key::N2),
                "3" => Some(Key::N3),
                "4" => Some(Key::N4),
                "5" => Some(Key::N5),
                "6" => Some(Key::N6),
                "7" => Some(Key::N7),
                "8" => Some(Key::N8),
                "9" => Some(Key::N9),

                _ => None,
            }
        }
        keyboard::Key::Unidentified(_) => {
            None
        }
        keyboard::Key::Dead(_) => {
            None
        }
    }
    
}

struct ResultHandle {
    err: Option<renderer::RenderErr>,
}

impl ResultHandle {
    fn take(&mut self) -> Option<renderer::RenderErr> {
        self.err.take()
    }
}

impl Default for ResultHandle {
    fn default() -> Self {
        Self { 
            err: None,
        }
    }
}


