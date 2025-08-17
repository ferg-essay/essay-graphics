use crate::output::Command;

#[derive(Default)]
pub struct Output {
    commands: Vec<Command>,
}

impl Output {
    pub fn command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn drain_commands<'a>(&'a mut self) -> impl ExactSizeIterator<Item=Command> + 'a {
        self.commands.drain(..)
    }
}
