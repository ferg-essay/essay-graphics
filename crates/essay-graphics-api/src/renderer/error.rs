
pub type Result<T, E=RenderErr> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum RenderErr {
    NotImplemented,
    RedrawRequired,
    Message(String),
}

impl From<&str> for RenderErr {
    fn from(value: &str) -> Self {
        RenderErr::Message(String::from(value))
    }
}
