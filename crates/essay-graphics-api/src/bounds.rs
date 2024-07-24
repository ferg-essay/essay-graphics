use core::fmt;
use std::{marker::PhantomData, any::type_name};

use essay_tensor::{Tensor, tf32};

use super::{Point, Coord, Affine2d};

///
/// Boundary box consisting of two unordered points
/// 
pub struct Bounds<M: Coord> {
    p0: Point,
    p1: Point,

    marker: PhantomData<fn(M)>,
}

impl<M: Coord> Bounds<M> {
    pub fn new(p0: impl Into<Point>, p1: impl Into<Point>) -> Self {
        Self {
            p0: p0.into(),
            p1: p1.into(),
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn extent(width: f32, height: f32) -> Self {
        Self::new(Point(0., 0.), Point(width, height))
    }

    pub fn from_bounds(
        x0: f32, 
        y0: f32, 
        width: f32, 
        height: f32
    ) -> Bounds<M> {
        Bounds {
            p0: Point(x0, y0),
            p1: Point(x0 + width, x0 + height),
            marker: PhantomData,
        }
    }

    pub fn none() -> Bounds<M> {
        Bounds {
            p0: Point(f32::MAX, f32::MAX),
            p1: Point(f32::MIN, f32::MIN),
            marker: PhantomData,
        }
    }

    pub fn zero() -> Bounds<M> {
        Bounds {
            p0: Point(0., 0.),
            p1: Point(0., 0.),
            marker: PhantomData,
        }
    }

    pub fn unit() -> Self {
        Self::new(Point(0., 0.), Point(1., 1.))
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        self.p0 == Point(f32::MAX, f32::MAX)
        && self.p1 == Point(f32::MIN, f32::MIN)
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.p0 == Point(0., 0.) && self.p1 == Point(0., 0.)
    }

    #[inline]
    pub fn x0(&self) -> f32 {
        self.p0.x()
    }

    #[inline]
    pub fn y0(&self) -> f32 {
        self.p0.y()
    }

    #[inline]
    pub fn x1(&self) -> f32 {
        self.p1.x()
    }

    #[inline]
    pub fn y1(&self) -> f32 {
        self.p1.y()
    }

    #[inline]
    pub fn xmin(&self) -> f32 {
        self.p0.x().min(self.p1.x())
    }

    #[inline]
    pub fn ymin(&self) -> f32 {
        self.p0.y().min(self.p1.y())
    }

    #[inline]
    pub fn min(&self) -> (f32, f32) {
        (self.xmin(), self.ymin())
    }

    #[inline]
    pub fn xmax(&self) -> f32 {
        self.p0.x().max(self.p1.x())
    }

    #[inline]
    pub fn ymax(&self) -> f32 {
        self.p0.y().max(self.p1.y())
    }

    #[inline]
    pub fn max(&self) -> (f32, f32) {
        (self.xmax(), self.ymax())
    }

    #[inline]
    pub fn xmid(&self) -> f32 {
        0.5 * (self.p0.x() + self.p1.x())
    }

    #[inline]
    pub fn ymid(&self) -> f32 {
        0.5 * (self.p0.y() + self.p1.y())
    }

    #[inline]
    pub fn mid(&self) -> (f32, f32) {
        (self.xmid(), self.ymid())
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.xmax() - self.xmin()
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.ymax() - self.ymin()
    }

    #[inline]
    pub fn contains(&self, point: impl Into<Point>) -> bool {
        let point = point.into();
        self.contains_x(point.x()) && self.contains_y(point.y())
    }

    #[inline]
    pub fn contains_x(&self, x: f32) -> bool {
        self.x0() <= x && x <= self.x1()
            || self.x1() <= x && x <= self.x0()

    }

    #[inline]
    pub fn contains_y(&self, y: f32) -> bool {
        self.y0() <= y && y <= self.y1()
            || self.y1() <= y && y <= self.y0()

    }

    pub fn corners(&self) -> Tensor {
        tf32!([
            [self.p0.x(), self.p0.y()],
            [self.p0.x(), self.p1.y()],
            [self.p1.x(), self.p1.y()],
            [self.p1.x(), self.p0.y()],
        ])
    }

    pub fn affine_to<N>(&self, box_to: &Bounds<N>) -> Affine2d
    where
        N: Coord
    {
        let a_x0 = self.xmin();
        let a_y0 = self.ymin();

        let epsilon = f32::EPSILON;
        let a_width = self.width().max(epsilon);
        let a_height = self.height().max(epsilon);

        let b_x0 = box_to.xmin();
        let b_y0 = box_to.ymin();

        let b_width = box_to.width();
        let b_height = box_to.height();

        Affine2d::eye()
            .translate(- a_x0, - a_y0)
            .scale(b_width / a_width, b_height / a_height)
            .translate(b_x0, b_y0)
    }

    pub fn union(&self, b: &Bounds<M>) -> Self {
        Self {
            p0: Point(
                self.xmin().min(b.xmin()),
                self.ymin().min(b.ymin()),
            ),
            p1: Point(
                self.xmax().max(b.xmax()),
                self.ymax().max(b.ymax()),
            ),
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn p0(&self) -> Point {
        self.p0
    }

    #[inline]
    pub fn p1(&self) -> Point {
        self.p1
    }
}

impl<M: Coord> Clone for Bounds<M> {
    fn clone(&self) -> Self {
        Self { 
            p0: self.p0.clone(), 
            p1: self.p1.clone(), 
            marker: self.marker.clone() 
        }
    }
}

impl<M: Coord> PartialEq for Bounds<M> {
    fn eq(&self, other: &Self) -> bool {
        self.p0 == other.p0 && self.p1 == other.p1 && self.marker == other.marker
    }
}

impl<M: Coord> fmt::Debug for Bounds<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: add marker to debug?
        let name = type_name::<M>();
        let tail = name.split("::").last();
        let tail = match tail {
            Some(tail) => tail,
            None => name,
        };
        
        write!(f, 
            "Bounds<{}>({},{}; {}x{})", 
            tail,
            self.x0(),
            self.y0(),
            self.x1() - self.x0(),
            self.y1() - self.y0()
        )
    }
}

impl<M: Coord> From<&Bounds<M>> for Bounds<M> {
    #[inline]
    fn from(value: &Bounds<M>) -> Self {
        value.clone()
    }
}

impl<M: Coord> From<()> for Bounds<M> {
    #[inline]
    fn from(_: ()) -> Self {
        Bounds::none()
    }
}

/// (x0, y0)
impl<M: Coord> From<(f32, f32)> for Bounds<M> {
    #[inline]
    fn from(value: (f32, f32)) -> Self {
        Bounds::new(
            Point(value.0, value.1),
            Point(value.0, value.1),
        )
    }
}

impl<M: Coord> From<Point> for Bounds<M> {
    #[inline]
    fn from(value: Point) -> Self {
        Bounds::new(
            value,
            value,
        )
    }
}

/// [width, height]
impl<M: Coord> From<[f32; 2]> for Bounds<M> {
    #[inline]
    fn from([w, h]: [f32; 2]) -> Self {
        Bounds::new(
            Point(0., 0.),
            Point(w, h),
        )
    }
}

/// ((x, y), [width, height])
impl<M: Coord> From<((f32, f32), [f32; 2])> for Bounds<M> {
    #[inline]
    fn from(((x, y), [w, h]): ((f32, f32), [f32; 2])) -> Self {
        Bounds::new(
            Point(x, y),
            Point(x + w, y + h),
        )
    }
}

/// [(x0, y0), (x1, y1)]
impl<M: Coord> From<[(f32, f32); 2]> for Bounds<M> {
    #[inline]
    fn from([p0, p1]: [(f32, f32); 2]) -> Self {
        Bounds::new(
            Point(p0.0, p0.1),
            Point(p1.0, p1.1),
        )
    }
}

/// (x0, y0, x1, y1)
impl<M: Coord> From<(f32, f32, f32, f32)> for Bounds<M> {
    #[inline]
    fn from((x0, y0, x1, y1): (f32, f32, f32, f32)) -> Self {
        Bounds::new(
            Point(x0, y0),
            Point(x1, y1),
        )
    }
}

impl<M: Coord> From<&Tensor> for Bounds<M> {
    fn from(value: &Tensor) -> Self {
        assert!(value.rank() == 2, "Bounds::from Tensor requires a 2d tensor {:?}", value.shape().as_slice());
        assert!(value.cols() == 2, "Bounds::from Tensor requires a 2d tensor {:?}", value.shape().as_slice());

        let mut x0 = f32::MAX;
        let mut y0 = f32::MAX;

        let mut x1 = f32::MIN;
        let mut y1 = f32::MIN;

        for point in value.iter_row() {
            x0 = x0.min(point[0]);
            y0 = y0.min(point[1]);

            x1 = x1.max(point[0]);
            y1 = y1.max(point[1]);
        }

        Bounds {
            p0: Point(x0, y0),
            p1: Point(x1, y1),
            marker: PhantomData,
        }
    }
}

impl<M: Coord> From<Bounds<M>> for Tensor {
    fn from(value: Bounds<M>) -> Self {
        value.corners()
    }
}

#[cfg(test)]
mod test {
    use crate::{Coord, Point};

    use super::Bounds;

    #[test]
    fn bounds_zero() {
        let bounds = Bounds::<Test>::zero();

        assert_eq!(bounds.is_zero(), true);
        assert_eq!(bounds.is_none(), false);

        assert_eq!(bounds.x0(), 0.);
        assert_eq!(bounds.y0(), 0.);

        assert_eq!(bounds.x1(), 0.);
        assert_eq!(bounds.y1(), 0.);
    }

    #[test]
    fn bounds_none() {
        let bounds = Bounds::<Test>::none();

        assert_eq!(bounds.is_zero(), false);
        assert_eq!(bounds.is_none(), true);

        // reversed MAX/MIN allows none() to calculate union extent.
        assert_eq!(bounds.x0(), f32::MAX);
        assert_eq!(bounds.y0(), f32::MAX);

        assert_eq!(bounds.x1(), f32::MIN);
        assert_eq!(bounds.y1(), f32::MIN);
    }

    #[test]
    fn bounds_unit() {
        let bounds = Bounds::<Test>::unit();

        assert_eq!(bounds.is_zero(), false);
        assert_eq!(bounds.is_none(), false);

        assert_eq!(bounds.x0(), 0.);
        assert_eq!(bounds.y0(), 0.);

        assert_eq!(bounds.x1(), 1.);
        assert_eq!(bounds.y1(), 1.);
    }

    #[test]
    fn bounds_new() {
        let bounds = Bounds::<Test>::new(Point(1., 2.), Point(3., 4.));

        assert_eq!(bounds.is_zero(), false);
        assert_eq!(bounds.is_none(), false);

        assert_eq!(bounds.x0(), 1.);
        assert_eq!(bounds.y0(), 2.);

        assert_eq!(bounds.x1(), 3.);
        assert_eq!(bounds.y1(), 4.);

        let b2 = Bounds::<Test>::new(Point(1., 2.), Point(3., 4.));

        assert_eq!(bounds == b2, true);
        assert_eq!(b2 == bounds, true);

        let b2 = Bounds::<Test>::new(Point(3., 4.), Point(1., 2.));

        assert_eq!(bounds == b2, false);
        assert_eq!(b2 == bounds, false);

        let b2 = Bounds::<Test>::new(Point(0., 2.), Point(3., 4.));

        assert_eq!(bounds == b2, false);
        assert_eq!(b2 == bounds, false);

        let b2 = Bounds::<Test>::new(Point(1., 0.), Point(3., 4.));

        assert_eq!(bounds == b2, false);
        assert_eq!(b2 == bounds, false);

        let b2 = Bounds::<Test>::new(Point(1., 2.), Point(0., 4.));

        assert_eq!(bounds == b2, false);
        assert_eq!(b2 == bounds, false);

        let b2 = Bounds::<Test>::new(Point(1., 2.), Point(3., 0.));

        assert_eq!(bounds == b2, false);
        assert_eq!(b2 == bounds, false);
    }

    #[test]
    fn bounds_from() {
        let bounds = Bounds::<Test>::new(Point(1., 2.), Point(3., 4.));

        assert_eq!(bounds, Bounds::<Test>::from((1., 2., 3., 4.)));
        assert_ne!(bounds, Bounds::<Test>::from((3., 4., 1., 2.)));

        assert_eq!(
            Bounds::<Test>::from((1., 2.)),
            Bounds::<Test>::new((1., 2.), (1., 2.))
        );

        assert_eq!(
            Bounds::<Test>::from([1., 2.]),
            Bounds::<Test>::new((0., 0.), (1., 2.))
        );

        assert_eq!(
            Bounds::<Test>::from(((10., 20.), [1., 2.])),
            Bounds::<Test>::new((10., 20.), (11., 22.))
        );

        assert!(Bounds::<Test>::from(()).is_none());
    }

    #[test]
    fn bounds_methods() {
        let b1 = Bounds::<Test>::new(Point(1., 20.), Point(3., 40.));
        let b2 = Bounds::<Test>::new(Point(3., 40.), Point(1., 20.));

        assert_eq!(b1.p0(), Point(1., 20.));
        assert_eq!(b1.p1(), Point(3., 40.));

        assert_eq!(b2.p0(), Point(3., 40.));
        assert_eq!(b2.p1(), Point(1., 20.));

        assert_eq!(b1.xmin(), 1.);
        assert_eq!(b2.xmin(), 1.);

        assert_eq!(b1.xmax(), 3.);
        assert_eq!(b2.xmax(), 3.);

        assert_eq!(b1.xmid(), 2.);
        assert_eq!(b2.xmid(), 2.);

        assert_eq!(b1.width(), 2.);
        assert_eq!(b2.width(), 2.);

        assert_eq!(b1.ymin(), 20.);
        assert_eq!(b2.ymin(), 20.);

        assert_eq!(b1.ymax(), 40.);
        assert_eq!(b2.ymax(), 40.);

        assert_eq!(b1.ymid(), 30.);
        assert_eq!(b2.ymid(), 30.);

        assert_eq!(b1.height(), 20.);
        assert_eq!(b2.height(), 20.);
    }

    struct Test {}
    impl Coord for Test {}
}