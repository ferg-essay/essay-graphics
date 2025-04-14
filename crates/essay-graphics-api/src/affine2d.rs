use core::fmt;

use essay_tensor::{ten, tensor::Tensor};

use crate::{renderer::Canvas, Coord, Path, Point};

#[derive(Clone)]
pub struct Affine2d {
    mat: [f32; 6],
}

impl Affine2d {
    #[inline]
    pub fn new(
        a: f32, b: f32, c: f32, 
        d: f32, e: f32, f: f32
    ) -> Affine2d {
        Self {
            mat: [a, b, c, d, e, f]
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[f32] {
        &self.mat
    }

    #[inline]
    pub fn mat(&self) -> Tensor {
        let m = &self.mat;

        ten![
            [m[0], m[1], m[2]],
            [m[3], m[4], m[5]],
            [0., 0., 1.],

        ]
    }

    #[inline]
    pub fn eye() -> Self {
        Self {
            mat: [
                1., 0., 0., 
                0., 1., 0.
            ]
        }
    }

    #[inline]
    pub fn translate(&self, x: f32, y: f32) -> Self {
        let m = self.mat;

        Self {
            mat: [
                m[0], m[1], m[2] + x,
                m[3], m[4], m[5] + y,
            ]
        }
    }

    #[inline]
    pub fn scale(&self, sx: f32, sy: f32) -> Self {
        let m = self.mat;

        Self {
            mat: [
                sx * m[0], sx * m[1], sx * m[2],
                sy * m[3], sy * m[4], sy * m[5],
            ]
        }
    }

    #[inline]
    pub fn rotate(&self, theta: f32) -> Self {
        if theta == 0. {
            return self.clone();
        }

        let (sin, cos) = theta.sin_cos();

        let rot = Self {
            mat: [
                cos, -sin, 0.,
                sin,  cos, 0.,
            ]
        };

        matmul(&rot, &self)
    }

    #[inline]
    pub fn rotate_around(&self, x: f32, y: f32, theta: f32) -> Self {
        if theta == 0. {
            return self.clone();
        }

        self.translate(-x, -y).rotate(theta).translate(x, y)
    }

    #[inline]
    pub fn rotate_deg(&self, deg: f32) -> Self {
        self.rotate(deg.to_radians())
    }

    #[inline]
    pub fn rotate_unit(&self, unit: f32) -> Self {
        self.rotate((0.25 - unit) * std::f32::consts::PI)
    }

    #[inline]
    pub fn matmul(&self, y: &Affine2d) -> Self {
        matmul(self, y)
    }

    #[inline]
    pub fn compose(&self, y: &Affine2d) -> Self {
        matmul(&y, &self)
    }

    pub fn transform(&self, points: &Tensor) -> Tensor {
        assert!(points.rank() > 1);
        assert!(points.cols() == 2);

        let mat = self.mat;

        points.map_row(|point| {
            let x = point[0];
            let y = point[1];

            [
                x * mat[0] + y * mat[1] + mat[2],
                x * mat[3] + y * mat[4] + mat[5],
            ]
        })
    }

    #[inline]
    pub fn transform_point(&self, point: Point) -> Point {
        let mat = self.mat;

        let Point(x, y) = point;

        Point(
            x * mat[0] + y * mat[1] + mat[2],
            x * mat[3] + y * mat[4] + mat[5],
        )
    }
    
    pub fn transform_path<T: Coord>(&self, path: &Path<T>) -> Path<Canvas> {
        let mat = self.mat;

        path.map(|Point(x, y)| {
            Point(
                x * mat[0] + y * mat[1] + mat[2],
                x * mat[3] + y * mat[4] + mat[5]
            )
        })
    }

    #[inline]
    pub fn strip_translation(&self) -> Self {
        let mat = self.mat;

        Self::new(
            mat[0], mat[1], 0.,
            mat[3], mat[4], 0.,
        )
    }
}

impl fmt::Debug for Affine2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Affine2d").field("mat", &self.mat).finish()
    }
}

pub fn eye() -> Affine2d {
    Affine2d::new(
        1., 0., 0.,
        0., 1., 0.
    )
}

pub fn scale(scale_x: f32, scale_y: f32) -> Affine2d {
    Affine2d::new(
        scale_x, 0., 0.,
        0., scale_y, 0.,
    )
}

pub fn translate(x: f32, y: f32) -> Affine2d {
    Affine2d::new(
        1., 0., x,
        0., 1., y,
    )
}

pub fn rotate(theta: f32) -> Affine2d {
    let sin = theta.sin();
    let cos = theta.cos();

    Affine2d::new(
        cos, -sin, 0.,
        sin, cos, 0.
    )
}

pub fn rotate_deg(deg: f32) -> Affine2d {
    let theta = deg.to_radians();
    
    let (sin, cos) = theta.sin_cos();

    Affine2d::new(
        cos, -sin, 0.,
        sin, cos, 0.
    )
}

fn matmul(x: &Affine2d, y: &Affine2d) -> Affine2d {
    let x = x.mat;
    let y = y.mat;

    let mat = [
        x[0] * y[0] + x[1] * y[3],
        x[0] * y[1] + x[1] * y[4],
        x[0] * y[2] + x[1] * y[5] + x[2],

        x[3] * y[0] + x[4] * y[3],
        x[3] * y[1] + x[4] * y[4],
        x[3] * y[2] + x[4] * y[5] + x[5],
    ];

    Affine2d {
        mat,
    }
}
