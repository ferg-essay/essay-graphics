use essay_graphics_api::Point;

pub(super) struct Mesh2d {
    pub vertices: Vec<[f32; 2]>,
}

impl Mesh2d {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn triangle(&mut self, p0: Point, p1: Point, p2: Point) {
        self.vertices.push([p0.0, p0.1]);
        self.vertices.push([p1.0, p1.1]);
        self.vertices.push([p2.0, p2.1]);
    }
}
