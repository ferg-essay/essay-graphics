use std::f32::consts::{FRAC_PI_2, TAU};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point(pub f32, pub f32);

impl Point {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn is_below(&self, p0: &Point, p1: &Point) -> bool {
        let Point(x, y) = self;
        let Point(x0, y0) = p0;
        let Point(x1, y1) = p1;

        if x0 == x1 {
            false
        } else if x0 <= x && x < x1 || x1 < x && x <= x0 {
            let y_line = (y0 * (x1 - x) + y1 * (x - x0)) / (x1 - x0);

            *y < y_line
        } else {
            false
        }
    }

    #[inline]
    pub fn dist(&self, p: &Point) -> f32 {
        let dx = self.0 - p.0;
        let dy = self.1 - p.1;

        dx.hypot(dy)
    }
}

impl From<&Point> for Point {
    #[inline]
    fn from(value: &Point) -> Self {
        *value
    }
}

impl From<[f32; 2]> for Point {
    #[inline]
    fn from(value: [f32; 2]) -> Self {
        Point(value[0], value[1])
    }
}

impl From<(f32, f32)> for Point {
    #[inline]
    fn from(value: (f32, f32)) -> Self {
        Point(value.0, value.1)
    }
}

impl From<&[f32; 2]> for Point {
    #[inline]
    fn from(value: &[f32; 2]) -> Self {
        Point(value[0], value[1])
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
}

impl From<f32> for Angle {
    fn from(value: f32) -> Self {
        Angle::Rad(value)
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
