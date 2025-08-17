use std::{time::Instant};

use essay_graphics_api::{input::Input, output::Output};
use essay_graphics_api::renderer::Result;
use essay_graphics_winit::{run_event_loop, MainLoopHandle};
use winit::{event_loop::EventLoop, window::Window};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = Window::new(&event_loop).unwrap();
    window.set_title("Empty Window");
    window.set_visible(true);

    let handle = Handle { window };

    run_event_loop(event_loop, handle).unwrap();
}

struct Handle {
    window: Window,
}

impl MainLoopHandle for Handle {
    fn request_redraw(&mut self) {
        self.window.request_redraw();
    }

    fn input(&mut self, input: &Input) -> Option<Instant> {
        println!("Input");
        for event in input.events() {
            println!("  {:?}", event);
        }

        None
    }

    fn redraw(&mut self) -> Result<Output> {
        println!("Redraw");
        Ok(Output::default())
    }
}