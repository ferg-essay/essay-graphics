use super::Drawable;

#[derive(Debug)]
pub enum DeviceErr {
    NotImplemented,
}

pub type Result<T, E = DeviceErr> = std::result::Result<T, E>;


pub trait Backend {
    fn main_loop(&mut self, drawable: Box<dyn Drawable>) -> Result<()>;
}
