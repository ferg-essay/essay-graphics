use crate::{renderer::Renderer, Bounds, Coord};

pub struct Canvas {}

impl Coord for Canvas {}

pub type Pos = Bounds<Canvas>;