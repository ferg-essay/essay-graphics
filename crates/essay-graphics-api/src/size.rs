#[derive(Clone, Copy, Debug)]
pub struct Size(pub f32, pub f32);

impl Size {
    #[inline]
    pub fn width(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.1
    }
}

impl From<[f32; 2]> for Size {
    #[inline]
    fn from(value: [f32; 2]) -> Self {
        Size(value[0], value[1])
    }
}
