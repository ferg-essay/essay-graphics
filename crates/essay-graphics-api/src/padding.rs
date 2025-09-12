use std::ops;

use crate::{Bounds, Coord, Rectangle, Size};


#[derive(Clone, Copy, Default, Debug)]
pub struct Padding {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32
}

impl Padding {
    pub const ZERO: Padding = Padding {left: 0., bottom: 0., right: 0., top: 0.};

    pub fn new(left: f32, bottom: f32, right: f32, top: f32) -> Self {
        Self {
            left,
            bottom,
            right,
            top
        }
    }

    pub fn from_all(size: f32) -> Self {
        Self::new(size, size, size, size)
    }

    pub fn from_pair(width: f32, height: f32) -> Self {
        Self::new(width, height, width, height)
    }

    #[inline]
    pub fn left(&self) -> f32 {
        self.left
    }

    #[inline]
    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    #[inline]
    pub fn right(&self) -> f32 {
        self.right
    }

    #[inline]
    pub fn top(&self) -> f32 {
        self.top
    }
    
    #[inline]
    pub fn height(&self) -> f32 {
        self.top + self.bottom
    }
    
    #[inline]
    pub fn width(&self) -> f32 {
        self.left + self.right
    }
}

impl ops::Add<Padding> for Padding {
    type Output = Padding;

    #[inline]
    fn add(self, rhs: Padding) -> Self::Output {
        Padding {
            left: self.left + rhs.left,
            bottom: self.bottom + rhs.bottom,
            right: self.right + rhs.right,
            top: self.top + rhs.top,
        }
    }
}

impl ops::Add<f32> for Padding {
    type Output = Padding;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Padding {
            left: self.left + rhs,
            bottom: self.bottom + rhs,
            right: self.right + rhs,
            top: self.top + rhs,
        }
    }
}

impl<T: Coord> ops::Add<Padding> for Bounds<T> {
    type Output = Bounds<T>;

    fn add(self, rhs: Padding) -> Self::Output {
        Bounds::new0(
            self.x0() - rhs.left, 
            self.y0() - rhs.bottom,
            self.width() + (rhs.left + rhs.right),
            self.height() + (rhs.top + rhs.bottom),
        )
    }
}

impl ops::Sub<Padding> for Rectangle {
    type Output = Rectangle;

    fn sub(self, rhs: Padding) -> Self::Output {
        Rectangle::new(
            self.x + rhs.left, 
            self.y + rhs.bottom,
            self.width - (rhs.left + rhs.right),
            self.height - (rhs.top + rhs.bottom),
        )
    }
}

impl ops::Add<Padding> for Rectangle {
    type Output = Rectangle;

    fn add(self, rhs: Padding) -> Self::Output {
        Rectangle::new(
            self.x - rhs.left, 
            self.y - rhs.bottom,
            self.width + (rhs.left + rhs.right),
            self.height + (rhs.top + rhs.bottom),
        )
    }
}

impl<T: Coord> ops::Sub<Padding> for Bounds<T> {
    type Output = Bounds<T>;

    fn sub(self, rhs: Padding) -> Self::Output {
        Bounds::new0(
            self.x0() + rhs.left, 
            self.y0() + rhs.bottom,
            self.width() - (rhs.left + rhs.right),
            self.height() - (rhs.top + rhs.bottom),
        )
    }
}

impl From<f32> for Padding {
    fn from(value: f32) -> Self {
        Padding::new(value, value, value, value)
    }
}

impl From<Size> for Padding {
    fn from(Size { width, height }: Size) -> Self {
        Padding::new(width, height, width, height)
    }
}

impl From<(f32, f32)> for Padding {
    fn from((width, height): (f32, f32)) -> Self {
        Padding::new(width, height, width, height)
    }
}
