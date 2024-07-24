use crate::{renderer::Canvas, Bounds, Point};

// TODO: Consider changing these to abstract events like Pan, Zoom because
// of tablets, etc.
#[derive(Clone, Debug)]
pub enum Event {
    Resize(Bounds<Canvas>),

    MouseLeftPress(Point),
    MouseLeftRelease(Point),
    Pan(Point, Point, Point),
    ResetView(Point),

    MouseRightPress(Point),
    MouseRightRelease(Point),
    MouseRightDrag(Point, Point),
    ZoomBounds(Point, Point),
    MouseRightDoubleClick(Point),

    MouseMiddlePress(Point),
    MouseMiddleRelease(Point),
    MouseMiddleDrag(Point, Point),
    MouseMiddleDoubleClick(Point),

    KeyPress(Point, char),
}

impl Event {
    #[inline]
    pub fn point(&self) -> Point {
        match self {
            Event::Resize(_) => Point(0., 0.),

            Event::MouseLeftPress(point) => *point,
            Event::MouseLeftRelease(point) => *point,
            Event::Pan(point, _, _) => *point,
            Event::ResetView(point) => *point,

            Event::MouseRightPress(point) => *point,
            Event::MouseRightRelease(point) => *point,
            Event::MouseRightDrag(point, _) => *point,
            Event::ZoomBounds(point, _) => *point,
            Event::MouseRightDoubleClick(point) => *point,

            Event::MouseMiddlePress(point) => *point,
            Event::MouseMiddleRelease(point) => *point,
            Event::MouseMiddleDrag(point, _) => *point,
            Event::MouseMiddleDoubleClick(point) => *point,

            Event::KeyPress(point, _) => *point,
        }
    }
}