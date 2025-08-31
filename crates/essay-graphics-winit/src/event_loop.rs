use std::{sync::{Arc, Mutex}, time::Instant};

use essay_graphics_api::{
    input::Input, output::Command, renderer
};
use winit::{
    event::{self, StartCause, WindowEvent }, 
    event_loop::{ControlFlow, EventLoop},
};

use crate::{input_event, MainLoopHandle};

pub fn run_event_loop(
    event_loop: EventLoop<()>, 
    mut handle: impl MainLoopHandle,
) -> renderer::Result<()> {
    let result = Arc::new(Mutex::new(ResultHandle::default()));
    let result_handle = result.clone();

    let mut wait_until: Option<Instant> = None;
    let mut input = Input::default();
    let mut is_input_stale = true;

    event_loop.run(move |event, window_target| {
        is_input_stale |= input_event(&mut input, &event);

        match event {
            event::Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                if is_input_stale {
                    handle.input(&mut input);
                    is_input_stale = false;
                }

                match handle.redraw() {
                    Ok(mut output) => {
                        for cmd in output.drain_commands() {
                            match cmd {
                                Command::RedrawAfterDelay(instant) => {
                                    wait_until = Some(instant);
                                },
                            }
                        }
                    }
                    Err(err) => {
                        result_handle.lock().unwrap().err = Some(err);
                        window_target.exit();
                    }
                }

                input.update_after_draw();
            }
            event::Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                wait_until = None;
                handle.request_redraw();
            },
            event::Event::AboutToWait => {
                if is_input_stale {
                    handle.request_redraw();
                }
            },

            event::Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => window_target.exit(),
            _ => {}
        }
                
        match wait_until {
            Some(wait_until) => {
                window_target.set_control_flow(ControlFlow::WaitUntil(wait_until));
            },
            None => {
                window_target.set_control_flow(ControlFlow::Wait);
            }
        }
    }).unwrap();

    let err = result.lock().unwrap().take();

    if let Some(err) = err {
        Err(err)
    } else {
        Ok(())
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


