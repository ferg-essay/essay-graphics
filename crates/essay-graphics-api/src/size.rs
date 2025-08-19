use std::ops;

#[derive(Clone, Copy, Debug, Default)]
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

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl From<[f32; 2]> for Size {
    #[inline]
    fn from(value: [f32; 2]) -> Self {
        Size(value[0], value[1])
    }
}

impl From<Size> for [f32; 2] {
    #[inline]
    fn from(value: Size) -> Self {
        [value.width(), value.height()]
    }
}

impl ops::Add<Size> for Size {
    type Output = Self;

    fn add(self, rhs: Size) -> Self::Output {
        Size(self.0 + rhs.0, self.1 + rhs.1)
    }
}
