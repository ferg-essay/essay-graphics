use core::fmt;

use essay_tensor::{ten, tensor::Tensor};

use crate::{renderer::Canvas, Angle, Coord, Path, Point};

#[derive(Clone)]
pub struct Affine2d {
    mat: Tensor,
}

impl Affine2d {
    pub fn new(
        a: f32, b: f32, c: f32, 
        d: f32, e: f32, f: f32
    ) -> Affine2d {
        let mat = ten![
            [a, b, c],
            [d, e, f],
            [0., 0., 1.],
        ]; 

        Self {
            mat
        }
    }

    pub fn mat(&self) -> Tensor {
        self.mat.clone()
    }

    pub fn eye() -> Self {
        // TODO: use Tensor::eye
        let mat = ten![
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 0., 1.],
        ]; 

        Self {
            mat
        }
    }

    pub fn translate(&self, x: f32, y: f32) -> Self {
        let translate = ten![
            [1., 0., x],
            [0., 1., y],
            [0., 0., 1.],
        ]; 

        Self {
            mat: matmul(&translate, &self.mat),
        }
    }

    pub fn scale(&self, sx: f32, sy: f32) -> Self {
        let scale = ten![
            [sx, 0., 0.],
            [0., sy, 0.],
            [0., 0., 1.],
        ]; 

        Self {
            mat: matmul(&scale, &self.mat),
        }
    }

    pub fn rotate(&self, theta: impl Into<Angle>) -> Self {
        let (sin, cos) = theta.into().sin_cos();

        let rot = ten![
            [cos, -sin, 0.],
            [sin,  cos, 0.],
            [0.,   0.,  1.],
        ]; 

        Self {
            mat: matmul(&rot, &self.mat),
        }
    }

    pub fn rotate_around(&self, x: f32, y: f32, theta: f32) -> Self {
        self.translate(-x, -y).rotate(theta).translate(x, y)
    }

    pub fn rotate_deg(&self, deg: f32) -> Self {
        self.rotate(deg.to_radians())
    }

    pub fn rotate_unit(&self, unit: f32) -> Self {
        self.rotate((0.25 - unit) * std::f32::consts::PI)
    }

    pub fn matmul(&self, y: &Affine2d) -> Self {
        Self {
            mat: matmul(&self.mat, &y.mat),
        }
    }

    pub fn compose(&self, y: &Affine2d) -> Self {
        Self {
            mat: matmul(&y.mat, &self.mat),
        }
    }

    pub fn transform(&self, points: &Tensor) -> Tensor {
        assert!(points.rank() == 2);
        assert!(points.cols() == 2);

        let mat = self.mat.as_slice();

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
        let mat = self.mat.as_slice();

        let Point(x, y) = point;

        Point(
            x * mat[0] + y * mat[1] + mat[2],
            x * mat[3] + y * mat[4] + mat[5],
        )
    }
    
    pub fn transform_path<T: Coord>(&self, path: &Path<T>) -> Path<Canvas> {
        let mat = self.mat.as_slice();

        path.map(|Point(x, y)| {
            Point(
                x * mat[0] + y * mat[1] + mat[2],
                x * mat[3] + y * mat[4] + mat[5]
            )
        })
    }

    #[inline]
    pub fn strip_translation(&self) -> Self {
        let mat = self.mat.as_slice();

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
    
    let sin = theta.sin();
    let cos = theta.cos();

    Affine2d::new(
        cos, -sin, 0.,
        sin, cos, 0.
    )
}

fn matmul(x: &Tensor, y: &Tensor) -> Tensor {
    assert_eq!(x.rank(), 2);
    assert_eq!(x.rows(), 3);
    assert_eq!(x.cols(), 3);
    assert_eq!(y.rank(), 2);
    assert_eq!(y.rows(), 3);
    assert_eq!(y.cols(), 3);

    let x = x.as_slice();
    let y = y.as_slice();

    let o = [
        x[0] * y[0] + x[1] * y[3],
        x[0] * y[1] + x[1] * y[4],
        x[0] * y[2] + x[1] * y[5] + x[2],

        x[3] * y[0] + x[4] * y[3],
        x[3] * y[1] + x[4] * y[4],
        x[3] * y[2] + x[4] * y[5] + x[5],

        0.,
        0.,
        1.,
    ];

    Tensor::from(o).reshape([3, 3])
}
