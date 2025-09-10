use crate::{Affine2d, Color, Point};

#[derive(Clone)]
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
        let Point { x: x0, y: y0 } = p0.into();
        let Point { x: x1, y: y1 } = p1.into();
        let Point { x: x2, y: y2 } = p2.into();

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
        (p1, uv1): (impl Into<Point>, impl Into<Point>),
        (p2, uv2): (impl Into<Point>, impl Into<Point>),
    ) {
        let (p0, uv0) = (p0.into(), uv0.into());
        let (p1, uv1) = (p1.into(), uv1.into());
        let (p2, uv2) = (p2.into(), uv2.into());

        self.vertices.push([p0.x, p0.y, uv0.x, uv0.y]);
        self.vertices.push([p1.x, p1.y, uv1.x, uv1.y]);
        self.vertices.push([p2.x, p2.y, uv2.x, uv2.y]);
    }

    #[inline]
    pub fn rect_uv(
        &mut self, 
        (p0, uv0): (impl Into<Point>, impl Into<Point>), 
        (p1, uv1): (impl Into<Point>, impl Into<Point>),
    ) {
        let (p0, uv0) = (p0.into(), uv0.into());
        let (p1, uv1) = (p1.into(), uv1.into());

        self.vertices.push([p0.x, p0.y, uv0.x, uv0.y]);
        self.vertices.push([p1.x, p0.y, uv1.x, uv0.y]);
        self.vertices.push([p1.x, p1.y, uv1.x, uv1.y]);

        self.vertices.push([p0.x, p0.y, uv0.x, uv0.y]);
        self.vertices.push([p1.x, p1.y, uv1.x, uv1.y]);
        self.vertices.push([p0.x, p1.y, uv0.x, uv1.y]);
    }

    #[inline]
    pub fn as_slice(&self) -> &[[f32; 4]] {
        self.vertices.as_slice()
    }

    pub fn transform(&self, affine: &Affine2d) -> Self {
        let vertices = self.vertices.iter().map(|vertex| {
            let Point { x, y } = affine.transform_point(Point::new(vertex[0], vertex[1]));

            [x, y, vertex[2], vertex[3]]
        }).collect();

        Self {
            vertices,
        }
    }
}

pub struct Mesh2dColor {
    pub vertices: Vec<([f32; 2], Color)>,
}

impl Mesh2dColor {
    #[inline]
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    #[inline]
    pub fn triangle(
        &mut self, 
        p0: (impl Into<Point>, impl Into<Color>),
        p1: (impl Into<Point>, impl Into<Color>), 
        p2: (impl Into<Point>, impl Into<Color>),
    ) {
        let (Point { x: x0, y: y0 }, color0) = (p0.0.into(), p0.1.into());
        let (Point { x: x1, y: y1 }, color1) = (p1.0.into(), p1.1.into());
        let (Point { x: x2, y: y2 }, color2) = (p2.0.into(), p2.1.into());

        self.vertices.push(([x0, y0], color0));
        self.vertices.push(([x1, y1], color1));
        self.vertices.push(([x2, y2], color2));
    }

    #[inline]
    pub fn as_slice(&self) -> &[([f32; 2], Color)] {
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

        self.vertices.push([p0.x, p0.y, uv0.x, uv0.y, 1., -1., width_above, width_below]);
        self.vertices.push([p1.x, p1.y, uv1.x, uv1.y, 0., 1., width_above, width_below]);
        self.vertices.push([p2.x, p2.y, uv2.x, uv2.y, -1., -1., width_above, width_below]);
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
        let Point { x: x0, y: y0 } = p0.into();
        let Point { x: x1, y: y1 } = p1.into();
        let Point { x: x2, y: y2 } = p2.into();

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