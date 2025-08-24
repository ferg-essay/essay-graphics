use core::fmt;

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
}

impl Rectangle {
    pub const ZERO: Rectangle = Rectangle::new(0., 0., 0., 0.);
    pub const UNIT: Rectangle = Rectangle::new(0., 0., 1., 1.);
    pub const INFINITE: Rectangle = Rectangle::new(f32::MIN, f32::MIN, f32::MAX, f32::MAX);
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

impl<T: Copy> Copy for Rectangle<T> {}

impl<T: PartialEq> PartialEq for Rectangle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.width == other.width && self.height == other.height
    }
}

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
