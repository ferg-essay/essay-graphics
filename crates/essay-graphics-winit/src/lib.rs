mod input;
mod event_loop;

use std::time::Instant;

use essay_graphics_api::{input::Input, output::Output, renderer::Result};
pub use input::{input_event};
pub use event_loop::{run_event_loop};

pub trait MainLoopHandle {
    fn request_redraw(&mut self);
    
    fn input(&mut self, input: &Input) -> Option<Instant>;

    fn redraw(&mut self) -> Result<Output>;
}
