use core::fmt;
use std::{any::type_name, marker::PhantomData};

use essay_tensor::{ten, tensor::Tensor};

use crate::{Rectangle, Size};

use super::{Point, Affine2d};

///
/// Boundary box consisting of two unordered points
/// 
pub struct Bounds<M: Coord> {
    rect: Rectangle,

    marker: PhantomData<fn(M)>,
}

impl<M: Coord> Bounds<M> {
    pub const ZERO: Bounds<M> = Self::new0(0., 0., 0., 0.);
    pub const UNIT: Bounds<M> = Self::new0(0., 0., 1., 1.);

    pub const NONE: Bounds<M> = Self::new0(f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    pub const INFINITY: Bounds<M> = Self::new0(f32::MIN, f32::MIN, f32::MAX, f32::MAX);

    // TODO: renaming
    #[must_use]
    pub const fn new0(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            rect: Rectangle::new(x, y, width, height),

            marker: PhantomData::<fn(M)>,
        }
    }

    #[must_use]
    pub fn new(p0: impl Into<Point>, p1: impl Into<Point>) -> Self {
        let Point { x: x0, y: y0 } = p0.into();
        let Point { x: x1, y: y1 } = p1.into();

        let xmin = x0.min(x1);
        let xmax = x0.max(x1);

        let ymin = y0.min(y1);
        let ymax = y0.max(y1);

        Self {
            rect: Rectangle::new(xmin, ymin, xmax - xmin, ymax - ymin),

            marker: PhantomData::<fn(M)>,
        }
    }

    #[must_use]
    pub fn new_flat(p0: impl Into<Point>, p1: impl Into<Point>) -> Self {
        let Point { x: x0, y: y0 } = p0.into();
        let Point { x: x1, y: y1 } = p1.into();

        Self {
            rect: Rectangle::new(x0, y0, x1 - x0, y1 - y0),
            marker: PhantomData::<fn(M)>,
        }
    }

    #[must_use]
    pub fn from_point(p0: impl Into<Point>) -> Self {
        let Point { x, y } = p0.into();

        Self {
            rect: Rectangle::new(x, y, 0., 0.),
            marker: PhantomData::<fn(M)>,
        }
    }

    #[must_use]
    pub fn from_size(size: impl Into<Size>) -> Self {
        let Size { width, height } = size.into();

        Self {
            rect: Rectangle::new(0., 0., width, height),
            marker: PhantomData::<fn(M)>,
        }
    }

    #[inline]
    #[must_use]
    pub fn extent(width: f32, height: f32) -> Self {
        Self::from_size(Size::new(width, height))
    }

    #[inline]
    pub fn from_bounds(
        x0: f32, 
        y0: f32, 
        width: f32, 
        height: f32
    ) -> Bounds<M> {
        Self {
            rect: Rectangle::new(x0, y0, width, height),
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn none() -> Bounds<M> {
        Self::NONE
    }

    #[inline]
    pub fn infinity() -> Bounds<M> {
        Self::INFINITY
    }

    #[inline]
    pub fn zero() -> Bounds<M> {
        Self::ZERO
    }

    #[inline]
    pub fn unit() -> Self {
        Self::UNIT
    }

    #[inline]
    pub fn is_none(self) -> bool {
        self.rect.x == f32::MAX
        && self.rect.y == f32::MAX
        && self.rect.width == f32::MIN
        && self.rect.height == f32::MIN
    }

    #[inline]
    pub fn or(self, default: Bounds<M>) -> Self {
        if ! self.is_none() {
            self.clone()
        } else {
            default            
        }
    }

    #[inline]
    pub fn is_zero(self) -> bool {
        self.rect.x == 0. && self.rect.y == 0. && self.rect.width == 0. && self.rect.height == 0.
    }

    #[inline]
    pub fn p0(self) -> Point {
        Point::new(self.rect.x, self.rect.y)
    }

    #[inline]
    pub fn p1(self) -> Point {
        Point::new(self.rect.x + self.rect.width, self.rect.y + self.rect.height)
    }

    #[inline]
    pub fn pos(self) -> Point {
        self.p0()
    }

    #[inline]
    pub fn size(self) -> Size {
        Size::new(self.rect.width, self.rect.height)
    }

    #[inline]
    pub fn x0(self) -> f32 {
        self.rect.x
    }

    #[inline]
    pub fn y0(self) -> f32 {
        self.rect.y
    }

    #[inline]
    pub fn x1(self) -> f32 {
        self.rect.x + self.rect.width
    }

    #[inline]
    pub fn y1(self) -> f32 {
        self.rect.y + self.rect.height
    }

    #[inline]
    pub fn xmin(self) -> f32 {
        self.rect.x
    }

    #[inline]
    pub fn ymin(self) -> f32 {
        self.rect.y
    }

    #[inline]
    pub fn min(self) -> (f32, f32) {
        (self.rect.x, self.rect.y)
    }

    #[inline]
    pub fn xmax(self) -> f32 {
        self.x1()
    }

    #[inline]
    pub fn ymax(self) -> f32 {
        self.y1()
    }

    #[inline]
    pub fn max(self) -> (f32, f32) {
        (self.x1(), self.y1())
    }

    #[inline]
    pub fn xmid(self) -> f32 {
        self.rect.x + 0.5 * self.rect.width
    }

    #[inline]
    pub fn ymid(self) -> f32 {
        self.rect.y + 0.5 * self.rect.height
    }

    #[inline]
    pub fn mid(self) -> (f32, f32) {
        (self.xmid(), self.ymid())
    }

    #[inline]
    pub fn width(self) -> f32 {
        self.rect.width
    }

    #[inline]
    pub fn height(self) -> f32 {
        self.rect.height
    }

    #[inline]
    #[deprecated]
    pub fn width_abs(self) -> f32 {
        self.width().abs()
    }

    #[inline]
    #[deprecated]
    pub fn height_abs(self) -> f32 {
        self.height().abs()
    }

    #[inline]
    pub fn contains(self, point: impl Into<Point>) -> bool {
        let point = point.into();
        self.contains_x(point.x) && self.contains_y(point.y)
    }

    #[inline]
    pub fn contains_x(self, x: f32) -> bool {
        self.rect.x <= x && x <= self.rect.x + self.rect.width
    }

    #[inline]
    pub fn contains_y(self, y: f32) -> bool {
        self.rect.y <= y && y <= self.rect.y + self.rect.height
    }

    pub fn corners(self) -> Tensor {
        ten![
            [self.x0(), self.y0()],
            [self.x0(), self.y1()],
            [self.x1(), self.y1()],
            [self.x1(), self.y0()],
        ]
    }

    #[must_use]
    #[deprecated]
    pub fn abs(&mut self) -> Self {
        self.clone()
    }

    pub fn affine_to<N>(self, box_to: impl Into<Bounds<N>>) -> Affine2d
    where
        N: Coord
    {
        let box_to = box_to.into();

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

    pub fn affine_to_exact<N>(self, box_to: impl Into<Bounds<N>>) -> Affine2d
    where
        N: Coord
    {
        let box_to = box_to.into();

        let a_x0 = self.x0();
        let a_y0 = self.y0();

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

    pub fn union(self, b: impl Into<Bounds<M>>) -> Self {
        let b = b.into();

        Self::new(
            [self.x0().min(b.x0()), self.y0().min(b.y0())],
                [self.x1().max(b.x1()), self.y1().max(b.y1())],
        )
    }

    //
    // Update the width
    //
    #[must_use]
    pub fn with_width(self, width: f32) -> Self {
        Self::new0(self.rect.x, self.rect.y, width, self.rect.height)
    }

    //
    // Returns bounds for a sub-area with the specified aspect ratio
    //
    #[must_use]
    pub fn with_aspect(self, aspect: f32) -> Self {
        let self_aspect = self.width() / self.height();

        if aspect < self_aspect {
            let width = self.height() * aspect;
            let margin = 0.5 * (self.width() - width);

            Self::new(
                Point::new(self.xmin() + margin, self.ymin()),
                Point::new(self.xmax() - margin, self.ymax()),
            )
        } else {
            let height = self.width() / aspect;
            let margin = 0.5 * (self.height() - height);

            Self::new(
                Point::new(self.xmin(), self.ymin() + margin),
                Point::new(self.xmax(), self.ymax() - margin),
            )
        }
    }

    //
    // Returns bounds for a sub-area with the specified margin
    //
    #[must_use]
    pub fn with_margin(self, margin: f32) -> Self {
        Self::new0(
            self.rect.x + margin, 
            self.rect.y + margin,
            self.rect.width - 2. * margin,
            self.rect.height - 2. * margin,
        )
    }

    //
    // Returns bounds for a sub-area with the specified margins
    //
    #[must_use]
    pub fn with_margins(self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self::new0(
            self.rect.x + left, 
            self.rect.y + bottom,
            self.rect.width - (left + right),
            self.rect.height - (top + bottom),
        )
    }

    #[must_use]
    pub fn round_ui(self) -> Self {
        Self::new0(
            self.rect.x.round(),
            self.rect.y.round(),
            self.rect.width.round(),
            self.rect.height.round(),
        )
    }
}

impl<M: Coord> Default for Bounds<M> {
    fn default() -> Self {
        Self::none()
    }
}

impl<M: Coord> Clone for Bounds<M> {
    fn clone(&self) -> Self {
        Self::new0(self.rect.x, self.rect.y, self.rect.width, self.rect.height)
    }
}

impl<M: Coord> Copy for Bounds<M> {
}

impl<M: Coord> PartialEq for Bounds<M> {
    fn eq(&self, other: &Self) -> bool {
        self.rect == other.rect
    }
}

impl<M: Coord> Eq for Bounds<M> {}

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
            self.width(),
            self.height()
        )
    }
}

impl<M: Coord> From<&Bounds<M>> for Bounds<M> {
    #[inline]
    fn from(value: &Bounds<M>) -> Self {
        value.clone()
    }
}

impl<M: Coord> From<Rectangle> for Bounds<M> {
    #[inline]
    fn from(rect: Rectangle) -> Self {
        Bounds::new0(rect.x, rect.y, rect.width, rect.height)
    }
}

impl<M: Coord> From<(Point, Size)> for Bounds<M> {
    #[inline]
    fn from((point, size): (Point, Size)) -> Self {
        Bounds::new0(point.x, point.y, size.width, size.height)
    }
}

impl<M: Coord> From<Bounds<M>> for (Point, Size) {
    #[inline]
    fn from(value: Bounds<M>) -> Self {
        (
            value.p0(),
            value.size()
        )
    }
}

impl<M: Coord> From<Size> for Bounds<M> {
    #[inline]
    fn from(value: Size) -> Self {
        Bounds::from_size(value)
    }
}

impl<M: Coord> From<Bounds<M>> for Size {
    #[inline]
    fn from(value: Bounds<M>) -> Self {
        Size::new(
            value.width(),
            value.height(),
        )
    }
}

impl<M: Coord> From<Bounds<M>> for Rectangle {
    fn from(value: Bounds<M>) -> Self {
        value.rect
    }
}

impl<M: Coord> From<Option<Size>> for Bounds<M> {
    #[inline]
    fn from(value: Option<Size>) -> Self {
        match value {
            Some(size) => Self::from(size),
            None => Bounds::none(),
        }
    }
}

/// [w, h]
impl<M: Coord> From<[f32; 2]> for Bounds<M> {
    #[inline]
    fn from(value: [f32; 2]) -> Self {
        Bounds::from_size(value)
    }
}

/// [w, h]
impl<M: Coord> From<Bounds<M>> for [f32; 2] {
    #[inline]
    fn from(value: Bounds<M>) -> Self {
        [value.width(), value.height()]
    }
}

impl<M: Coord> From<Point> for Bounds<M> {
    #[inline]
    fn from(value: Point) -> Self {
        Bounds::from_point(value)
    }
}

/// (x0, y0)
impl<M: Coord> From<([f32; 2], Option<Size>)> for Bounds<M> {
    #[inline]
    fn from(value: ([f32; 2], Option<Size>)) -> Self {
        let [x, y] = value.0;

        match value.1 {
            Some(size) => Bounds::new(
                Point::new(x, y),
                Point::new(x + size.width, y + size.height),
            ),
            None => Bounds::new(
                Point::new(x, y),
                Point::new(x, y),
            )
        }
    }
}

/// ([x, y], [width, height])
impl<M: Coord> From<([f32; 2], [f32; 2])> for Bounds<M> {
    #[inline]
    fn from(([x, y], [w, h]): ([f32; 2], [f32; 2])) -> Self {
        Bounds::new(
            Point::new(x, y),
            Point::new(x + w, y + h),
        )
    }
}

/// ([x, y], [width, height])
impl<M: Coord> From<Bounds<M>> for ([f32; 2], [f32; 2]) {
    #[inline]
    fn from(value: Bounds<M>) -> Self {
        (
            [value.xmin(), value.ymin()],
            [value.width(), value.height()],
        )
    }
}

/// [[x0, y0], [x1, y1]]
impl<M: Coord> From<Bounds<M>> for [[f32; 2]; 2] {
    #[inline]
    fn from(value: Bounds<M>) -> Self {
        [
            [value.xmin(), value.ymin()],
            [value.xmax(), value.ymax()],
        ]
    }
}

/// [Point, Point]
impl<M: Coord> From<[Point; 2]> for Bounds<M> {
    #[inline]
    fn from(value: [Point; 2]) -> Self {
        Bounds::new(
            value[0],
            value[1],
        )
    }
}

/// [[x0, y0], [x1, y1]]
impl<M: Coord> From<[[f32; 2]; 2]> for Bounds<M> {
    #[inline]
    fn from([p0, p1]: [[f32; 2]; 2]) -> Self {
        Bounds::new(
            Point::new(p0[0], p0[1]),
            Point::new(p1[0], p1[1]),
        )
    }
}

impl<M: Coord> From<&Tensor> for Bounds<M> {
    fn from(value: &Tensor) -> Self {
        assert!(value.rank() == 2, "Bounds::from Tensor requires a 2d tensor {:?}", value.shape().as_vec());
        assert!(value.cols() == 2, "Bounds::from Tensor requires a 2d tensor {:?}", value.shape().as_vec());

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

        Bounds::new0(x0, y0, x1 - x0, y1 - y0)
    }
}

impl<M: Coord> From<Bounds<M>> for Tensor {
    fn from(value: Bounds<M>) -> Self {
        value.corners()
    }
}


///
/// The coordinate for bounds
/// 
pub trait Coord: 'static {}

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
        let bounds = Bounds::<Test>::new(Point::new(1., 2.), Point::new(3., 4.));

        assert_eq!(bounds.is_zero(), false);
        assert_eq!(bounds.is_none(), false);

        assert_eq!(bounds.x0(), 1.);
        assert_eq!(bounds.y0(), 2.);

        assert_eq!(bounds.x1(), 3.);
        assert_eq!(bounds.y1(), 4.);

        let b2 = Bounds::<Test>::new(Point::new(1., 2.), Point::new(3., 4.));

        assert_eq!(bounds == b2, true);
        assert_eq!(b2 == bounds, true);

        let b2 = Bounds::<Test>::new(Point::new(3., 4.), Point::new(1., 2.));

        assert_eq!(bounds == b2, false);
        assert_eq!(b2 == bounds, false);

        let b2 = Bounds::<Test>::new(Point::new(0., 2.), Point::new(3., 4.));

        assert_eq!(bounds == b2, false);
        assert_eq!(b2 == bounds, false);

        let b2 = Bounds::<Test>::new(Point::new(1., 0.), Point::new(3., 4.));

        assert_eq!(bounds == b2, false);
        assert_eq!(b2 == bounds, false);

        let b2 = Bounds::<Test>::new(Point::new(1., 2.), Point::new(0., 4.));

        assert_eq!(bounds == b2, false);
        assert_eq!(b2 == bounds, false);

        let b2 = Bounds::<Test>::new(Point::new(1., 2.), Point::new(3., 0.));

        assert_eq!(bounds == b2, false);
        assert_eq!(b2 == bounds, false);
    }

    #[test]
    fn bounds_from() {
        let bounds = Bounds::<Test>::new(Point::new(1., 2.), Point::new(3., 4.));

        assert_eq!(bounds, Bounds::<Test>::from([[1., 2.], [3., 4.]]));
        assert_ne!(bounds, Bounds::<Test>::from([[3., 4.], [1., 2.]]));

        assert_eq!(
            Bounds::<Test>::from(([1., 2.], None)),
            Bounds::<Test>::new([1., 2.], [1., 2.])
        );

        assert_eq!(
            Bounds::<Test>::from([1., 2.]),
            Bounds::<Test>::new([0., 0.], [1., 2.])
        );

        assert_eq!(
            Bounds::<Test>::from(([10., 20.], [1., 2.])),
            Bounds::<Test>::new([10., 20.], [11., 22.])
        );

        assert!(Bounds::<Test>::from(None).is_none());
    }

    #[test]
    fn bounds_methods() {
        let b1 = Bounds::<Test>::new(Point::new(1., 20.), Point::new(3., 40.));
        let b2 = Bounds::<Test>::new(Point::new(3., 40.), Point::new(1., 20.));

        assert_eq!(b1.p0(), Point::new(1., 20.));
        assert_eq!(b1.p1(), Point::new(3., 40.));

        assert_eq!(b2.p0(), Point::new(3., 40.));
        assert_eq!(b2.p1(), Point::new(1., 20.));

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