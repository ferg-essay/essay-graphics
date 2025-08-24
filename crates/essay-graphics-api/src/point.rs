use std::{f32::consts::{FRAC_PI_2, TAU}, ops};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T = f32> {
    pub x: T, 
    pub y: T,
}

impl<T> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Point {
    pub const ZERO: Point = Point::new(0., 0.);
    pub const X: Point = Point::new(1., 0.);
    pub const Y: Point = Point::new(0., 1.);

    #[inline]
    pub fn is_below(self, p0: Point, p1: Point) -> bool {
        let [x, y] = self.into();
        let [x0, y0] = p0.into();
        let [x1, y1] = p1.into();

        if x0 == x1 {
            false
        } else if x0 <= x && x < x1 || x1 < x && x <= x0 {
            let y_line = (y0 * (x1 - x) + y1 * (x - x0)) / (x1 - x0);

            y < y_line
        } else {
            false
        }
    }

    #[inline]
    pub fn dist(self, p: Point) -> f32 {
        self.hypot(p)
    }

    #[inline]
    pub fn hypot(self, p: Point) -> f32 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;

        dx.hypot(dy)
    }

    #[inline]
    pub fn interpolate(self, p: f32, point: Point) -> Point {
        Self::new(
            (1. - p) * self.x + p * point.x,
            (1. - p) * self.y + p * point.y,
        )
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<T> ops::Add for Point<T>
where
    T: ops::Add<Output = T>
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> ops::Sub for Point<T>
where
    T: ops::Sub<Output = T>
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl From<&Point> for Point {
    #[inline]
    fn from(value: &Point) -> Self {
        *value
    }
}

impl<T> From<[T; 2]> for Point<T> {
    #[inline]
    fn from([x, y]: [T; 2]) -> Self {
        Point::new(x, y)
    }
}

impl<T> From<Point<T>> for [T; 2] {
    #[inline]
    fn from(value: Point<T>) -> Self {
        let Point { x, y } = value;

        [x, y]
    }
}

impl<T: Clone> From<&[T; 2]> for Point<T> {
    #[inline]
    fn from([x, y]: &[T; 2]) -> Self {
        Point::new(x.clone(), y.clone())
    }
}

// angle in [0., 1.]
#[derive(Clone, Copy, Debug)]
pub enum Angle {
    Rad(f32),
    Deg(f32),
    Unit(f32),
}

impl Angle {
    #[inline]
    pub fn to_radians(&self) -> f32 {
        match self {
            Angle::Rad(rad) => (*rad + TAU) % TAU,
            Angle::Deg(deg) => (deg.to_radians() + TAU) % TAU,
            Angle::Unit(unit) => ((unit + 1.) * 360.).to_radians() % TAU,
        }
    }

    #[inline]
    pub fn to_radians_arc(&self) -> f32 {
        match self {
            Angle::Rad(rad) => (*rad + TAU) % TAU,
            Angle::Deg(deg) => (deg.to_radians() + TAU) % TAU,
            Angle::Unit(unit) => ((unit + 1.) * 360.).to_radians() % TAU,
        }
    }

    #[inline]
    pub fn to_degrees(&self) -> f32 {
        match self {
            Angle::Rad(rad) => (rad.to_degrees() + 360.) % 360.,
            Angle::Deg(deg) => (*deg + 360.) % 360.,
            Angle::Unit(unit) => (unit * 360. + 360.) % 360.,
        }
    }

    #[inline]
    pub fn to_unit(&self) -> f32 {
        match self {
            Angle::Rad(rad) => (rad.to_degrees() / 360. + 1.) % 1.,
            Angle::Deg(deg) => (deg / 360. + 1.) % 1.,
            Angle::Unit(unit) => (*unit + 1.) % 1.,
        }
    }

    #[inline]
    pub fn cos(&self) -> f32 {
        self.to_radians().cos()
    }

    #[inline]
    pub fn sin(&self) -> f32 {
        self.to_radians().sin()
    }

    #[inline]
    pub fn sin_cos(&self) -> (f32, f32) {
        self.to_radians().sin_cos()
    }
}

impl From<f32> for Angle {
    fn from(value: f32) -> Self {
        Angle::Rad(value)
    }
}

impl From<Heading> for Angle {
    fn from(value: Heading) -> Self {
        match value {
            Heading::Rad(theta) => Angle::Rad(theta),
            Heading::Deg(theta) => Angle::Deg((360. + 90. - theta) % 360.),
            Heading::Unit(theta) => Angle::Unit((1.25 - theta) % 1.),
        }
    }
}

///
/// Heading represents an angular direction
///
#[derive(Clone, Copy, Debug)]
pub enum Heading {
    /// Heading in counter-clockwise radians, where 0 is East
    Rad(f32),
    /// Heading in clockwise degrees, where 0 is North
    Deg(f32),
    /// Heading in clockwise unit coordinates, where 0 is North
    Unit(f32),
}

impl Heading {
    #[inline]
    pub fn to_radians(&self) -> f32 {
        match self {
            Heading::Rad(rad) => (*rad + TAU) % TAU,
            Heading::Deg(deg) => (TAU + FRAC_PI_2 - deg.to_radians()) % TAU,
            Heading::Unit(unit) => (TAU + FRAC_PI_2 - unit * TAU) % TAU,
        }
    }

    #[inline]
    pub fn to_degrees(&self) -> f32 {
        match self {
            Heading::Rad(rad) => (360. + 90. - rad.to_degrees()) % 360.,
            Heading::Deg(deg) => (*deg + 360.) % 360.,
            Heading::Unit(unit) => (unit * 360. + 360.) % 360.,
        }
    }

    #[inline]
    pub fn to_unit(&self) -> f32 {
        match self {
            Heading::Rad(rad) => (1.25 - rad.to_degrees() / 360.) % 1.,
            Heading::Deg(deg) => (deg / 360. + 1.) % 1.,
            Heading::Unit(unit) => (*unit + 1.) % 1.,
        }
    }

    #[inline]
    pub fn cos(&self) -> f32 {
        self.to_radians().cos()
    }

    #[inline]
    pub fn sin(&self) -> f32 {
        self.to_radians().sin()
    }
}

impl From<f32> for Heading {
    fn from(value: f32) -> Self {
        Heading::Rad(value)
    }
}
