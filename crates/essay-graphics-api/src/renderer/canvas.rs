use crate::{Bounds, Coord};

pub struct Canvas {}

impl Coord for Canvas {}

pub type Pos = Bounds<Canvas>;