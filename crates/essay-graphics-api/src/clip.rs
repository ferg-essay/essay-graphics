use crate::{renderer::Canvas, Bounds, Point};

#[derive(Clone, Debug, Copy)]
pub enum Clip {
    None,
    Bounds(Point, Point),
}

impl From<&Bounds<Canvas>> for Clip {
    fn from(value: &Bounds<Canvas>) -> Self {
        Clip::Bounds(
            Point(value.xmin(), value.ymin()),
            Point(value.xmax(), value.ymax()),
        )
    }
}