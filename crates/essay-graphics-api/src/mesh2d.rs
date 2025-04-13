use crate::Point;

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

pub struct BezierMesh2d {
    pub vertices: Vec<[f32; 8]>,
}

impl BezierMesh2d {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    #[inline]
    pub fn triangle_uv(
        &mut self, 
        (p0, uv0): (Point, Point), 
        (p1, uv1): (Point, Point), // bezier control point
        (p2, uv2): (Point, Point),
        width_above: f32,
        width_below: f32,
    ) {
        self.vertices.push([p0.0, p0.1, uv0.0, uv0.1, 1., -1., width_above, width_below]);
        self.vertices.push([p1.0, p1.1, uv1.0, uv1.1, 0., 1., width_above, width_below]);
        self.vertices.push([p2.0, p2.1, uv2.0, uv2.1, -1., -1., width_above, width_below]);
    }

    #[inline]
    pub fn triangle(
        &mut self, 
        p0: impl Into<Point>, 
        p1: impl Into<Point>, // bezier control point
        p2: impl Into<Point>,
        width_above: f32,
        width_below: f32,
    ) {
        let Point(x0, y0) = p0.into();
        let Point(x1, y1) = p1.into();
        let Point(x2, y2) = p2.into();

        self.vertices.push([x0, y0, 0., 0., 1., -1., width_above, width_below]);
        self.vertices.push([x1, y1, 0., 0., 0., 1., width_above, width_below]);
        self.vertices.push([x2, y2, 0., 0., -1., -1., width_above, width_below]);
    }

    #[inline]
    pub fn as_slice(&self) -> &[[f32; 8]] {
        self.vertices.as_slice()
    }
}
