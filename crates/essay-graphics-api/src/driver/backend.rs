use super::FigureApi;

#[derive(Debug)]
pub enum DeviceErr {
    NotImplemented,
}

pub type Result<T, E = DeviceErr> = std::result::Result<T, E>;


pub trait Backend {
    // fn renderer(&mut self) -> &dyn Renderer;

    fn main_loop(&mut self, figure: Box<dyn FigureApi>) -> Result<()>;
}
