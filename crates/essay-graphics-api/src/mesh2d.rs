use crate::Point;

pub struct Mesh2d {
    pub vertices: Vec<[f32; 4]>,
}

impl Mesh2d {
    #[inline]
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    #[inline]
    pub fn triangle(
        &mut self, 
        p0: impl Into<Point>, 
        p1: impl Into<Point>, 
        p2: impl Into<Point>
    ) {
        let Point(x0, y0) = p0.into();
        let Point(x1, y1) = p1.into();
        let Point(x2, y2) = p2.into();

        // default UV assumes 64x64 hatch-like texture
        let f = 1. / 64.;

        self.vertices.push([x0, y0, f * x0, f * y0]);
        self.vertices.push([x1, y1, f * x1, f * y1]);
        self.vertices.push([x2, y2, f * x2, f * y2]);
    }

    #[inline]
    pub fn triangle_uv(
        &mut self, 
        (p0, uv0): (impl Into<Point>, impl Into<Point>), 
        (p1, uv1): (impl Into<Point>, impl Into<Point>), // bezier control point
        (p2, uv2): (impl Into<Point>, impl Into<Point>),
    ) {
        let (p0, uv0) = (p0.into(), uv0.into());
        let (p1, uv1) = (p1.into(), uv1.into());
        let (p2, uv2) = (p2.into(), uv2.into());

        self.vertices.push([p0.0, p0.1, uv0.0, uv0.1]);
        self.vertices.push([p1.0, p1.1, uv1.0, uv1.1]);
        self.vertices.push([p2.0, p2.1, uv2.0, uv2.1]);
    }

    #[inline]
    pub fn as_slice(&self) -> &[[f32; 4]] {
        self.vertices.as_slice()
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
        (p0, uv0): (impl Into<Point>, impl Into<Point>), 
        (p1, uv1): (impl Into<Point>, impl Into<Point>), // bezier control point
        (p2, uv2): (impl Into<Point>, impl Into<Point>),
        width_above: f32,
        width_below: f32,
    ) {
        let (p0, uv0) = (p0.into(), uv0.into());
        let (p1, uv1) = (p1.into(), uv1.into());
        let (p2, uv2) = (p2.into(), uv2.into());

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

impl From<&Vec<[[f32; 2]; 3]>> for Mesh2d {
    fn from(value: &Vec<[[f32; 2]; 3]>) -> Self {
        let mut mesh = Mesh2d::new();

        for tri in value {
            mesh.triangle(tri[0], tri[1], tri[2]);
        }

        mesh
    }
}