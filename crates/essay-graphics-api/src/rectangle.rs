use core::fmt;
use std::ops;

use crate::{Point, Size};

pub struct Rectangle<T = f32> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T
}

impl<T> Rectangle<T> {
    pub const fn new(x: T, y: T, width: T, height: T) -> Self {
        Self { x, y, width, height }
    }

    #[inline]
    #[must_use]
    pub fn with_x(mut self, x: T) -> Self {
        self.x = x;

        self
    }

    #[inline]
    #[must_use]
    pub fn with_y(mut self, y: T) -> Self {
        self.y = y;

        self
    }

    #[inline]
    #[must_use]
    pub fn with_width(mut self, width: T) -> Self {
        self.width = width;

        self
    }

    #[inline]
    #[must_use]
    pub fn with_height(mut self, height: T) -> Self {
        self.height = height;

        self
    }
}

impl<T: Clone> Rectangle<T> {
    #[inline]
    pub fn p0(&self) -> Point<T> {
        Point {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }

    #[inline]
    pub fn size(&self) -> Size<T> {
        Size {
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}

impl<T> Rectangle<T>
where
    T: Clone + ops::Add<Output=T>
{
    #[inline]
    pub fn p1(&self) -> Point<T> {
        Point {
            x: self.x.clone() + self.width.clone(),
            y: self.y.clone() + self.height.clone(),
        }
    }

    #[inline]
    pub fn xmin(&self) -> T {
        self.x.clone()
    }

    #[inline]
    pub fn xmax(&self) -> T {
        self.x.clone() + self.width.clone()
    }

    #[inline]
    pub fn ymin(&self) -> T {
        self.y.clone()
    }

    #[inline]
    pub fn ymax(&self) -> T {
        self.y.clone() + self.height.clone()
    }
}

impl Rectangle {
    pub const ZERO: Rectangle = Rectangle::new(0., 0., 0., 0.);
    pub const UNIT: Rectangle = Rectangle::new(0., 0., 1., 1.);
    pub const INFINITE: Rectangle = Rectangle::new(f32::MIN, f32::MIN, f32::MAX, f32::MAX);
    
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.x == 0. && self.y == 0. && self.width == 0. && self.height == 0.
    }

    #[inline]
    pub fn contains(&self, point: impl Into<Point>) -> bool {
        let Point { x, y } = point.into();

        self.x <= x && x <= self.x + self.width
        && self.y <= y && y <= self.y + self.height
    }

    pub fn union(&self, rhs: impl Into<Rectangle>) -> Self {
        let rhs = rhs.into();

        let x0 = self.x.min(rhs.x);
        let y0 = self.y.min(rhs.y);

        let x1 = self.xmax().max(rhs.xmax());
        let y1 = self.ymax().max(rhs.ymax());

        Self {
            x: x0,
            y: y0,
            width: x1 - x0,
            height: y1 - y0
        }
    }

    #[inline]
    pub fn snap(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            width: self.width.ceil(),
            height: self.height.ceil(),
        }
    }
}

impl<T: Clone> Clone for Rectangle<T> {
    fn clone(&self) -> Self {
        Self { 
            x: self.x.clone(), 
            y: self.y.clone(), 
            width: self.width.clone(), 
            height: self.height.clone() 
        }
    }
}

impl<T: Default> Default for Rectangle<T> {
    fn default() -> Self {
        Self { 
            x: Default::default(), 
            y: Default::default(), 
            width: Default::default(), 
            height: Default::default() 
        }
    }
}

impl<T: Default> From<Size<T>> for Rectangle<T> {
    fn from(value: Size<T>) -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            width: value.width,
            height: value.height,
        }
    }
}

impl<T: Default> From<Point<T>> for Rectangle<T> {
    fn from(value: Point<T>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: T::default(),
            height: T::default(),
        }
    }
}

impl<T: Copy> Copy for Rectangle<T> {}

impl<T: PartialEq> PartialEq for Rectangle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.width == other.width && self.height == other.height
    }
}

// impl<T: Eq> Eq for Rectangle<T> {}

impl Eq for Rectangle<f32> {}

impl<T: fmt::Debug> fmt::Debug for Rectangle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "Rectangle({:?},{:?}; {:?}x{:?})", 
            self.x,
            self.y,
            self.width,
            self.height
        )
    }
}
