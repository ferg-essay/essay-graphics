use std::time::Instant;

#[derive(Clone, Debug)]
pub enum Command {
    RedrawAfterDelay(Instant),
}