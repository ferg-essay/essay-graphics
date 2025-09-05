use std::ops;

#[derive(Clone, Copy, Debug, Default)]
pub struct Size<T = f32> {
    pub width: T,
    pub height: T
}

impl<T> Size<T> {
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl Size {
    pub const ZERO: Size = Size::new(0., 0.);
    pub const UNIT: Size = Size::new(1., 0.);
    pub const INFINITE: Size = Size::new(f32::INFINITY, f32::INFINITY);

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.width == 0. && self.height == 0.
    }

    #[inline]
    pub fn max(&self, rhs: &Self) -> Self {
        Self {
            width: self.width.max(rhs.width),
            height: self.height.max(rhs.height),
        }
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl<T> From<[T; 2]> for Size<T> {
    #[inline]
    fn from([width, height]: [T; 2]) -> Self {
        Self::new(width, height)
    }
}

impl<T> From<Size<T>> for [T; 2] {
    #[inline]
    fn from(value: Size<T>) -> Self {
        [value.width, value.height]
    }
}

impl<T> ops::Add<Size<T>> for Size<T>
    where
        T: ops::Add<Output = T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl<T> ops::Sub<Size<T>> for Size<T>
    where
        T: ops::Sub<Output = T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.width - rhs.width, self.height - rhs.height)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Length {
    Shrink,
    Pixels(f32),
    Fill,
    View(f32),
}

impl From<Size> for Size<Length> {
    fn from(value: Size) -> Self {
        Size::new(
            Length::Pixels(value.width),
            Length::Pixels(value.height),
        )
    }
}
