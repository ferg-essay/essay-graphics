use std::time::Instant;

use crate::output::Command;

#[derive(Default, Debug)]
pub struct Output {
    commands: Vec<Command>,
}

impl Output {
    pub fn new() -> Self {
        Output::default()
    }

    pub fn command(&mut self, command: Command) -> &mut Self {
        self.commands.push(command);

        self
    }

    pub fn redraw_after_delay(&mut self, time: Instant) -> &mut Self {
        self.command(Command::RedrawAfterDelay(time))
    }

    pub fn drain_commands<'a>(&'a mut self) -> impl ExactSizeIterator<Item=Command> + 'a {
        self.commands.drain(..)
    }
}
