#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    Background,
    Main,
    Popup,
}

impl Layer {
    pub const ORDER: [Layer; 3] = [
        Layer::Background,
        Layer::Main,
        Layer::Popup,
    ];

    #[inline]
    pub fn index(&self) -> usize {
        match self {
            Layer::Background => 0,
            Layer::Main => 1,
            Layer::Popup => 2,
        }
    }
}