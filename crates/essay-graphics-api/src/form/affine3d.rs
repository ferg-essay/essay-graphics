use core::fmt;

use essay_tensor::{ten, tensor::Tensor};

use crate::{Affine2d, Angle};

#[derive(Clone)]
pub struct Affine3d {
    mat: Tensor,
}

impl Affine3d {
    #[inline]
    pub fn new(
        x00: f32, x01: f32, x02: f32, x03: f32,
        x10: f32, x11: f32, x12: f32, x13: f32,
        x20: f32, x21: f32, x22: f32, x23: f32,
    ) -> Affine3d {
        let mat = ten![
            [x00, x01, x02, x03],
            [x10, x11, x12, x13],
            [x20, x21, x22, x23],
            [ 0.,  0.,  0.,  1.],
        ]; 

        Self {
            mat
        }
    }

    #[inline]
    pub fn mat(&self) -> Tensor {
        self.mat.clone()
    }

    #[inline]
    pub fn eye() -> Self {
        // TODO: use Tensor::eye
        let mat = ten!([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]); 

        Self {
            mat
        }
    }

    #[inline]
    pub fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        let translate = ten![
            [1., 0., 0.,  x],
            [0., 1., 0.,  y],
            [0., 0., 1.,  z],
            [0., 0., 0., 1.],
        ]; 

        // TODO: optimize
        Self {
            mat: compose(&translate, &self.mat),
        }
    }

    #[inline]
    pub fn scale(&self, sx: f32, sy: f32, sz: f32) -> Self {
        let scale = ten![
            [sx, 0., 0., 0.],
            [0., sy, 0., 0.],
            [0., 0., sz, 0.],
            [0., 0., 0., 1.],
        ]; 

        // TODO: optimize
        Self {
            mat: compose(&scale, &self.mat),
        }
    }

    #[inline]
    pub fn rot_xy(&self, theta: impl Into<Angle>) -> Self {
        let theta = theta.into();
        
        let sin = theta.sin();
        let cos = theta.cos();

        let rot = ten![
            [cos, -sin, 0., 0.],
            [sin,  cos, 0., 0.],
            [ 0.,   0., 1., 0.],
            [ 0.,   0., 0., 1.],
        ]; 

        Self {
            mat: compose(&rot, &self.mat),
        }
    }

    #[inline]
    pub fn rot_xz(&self, theta: impl Into<Angle>) -> Self {
        let theta = theta.into();

        let sin = theta.sin();
        let cos = theta.cos();

        let rot = ten![
            [cos, 0., -sin, 0.],
            [ 0., 1.,   0., 0.],
            [sin, 0.,  cos, 0.],
            [ 0., 0.,   0., 1.],
        ]; 

        Self {
            mat: compose(&rot, &self.mat),
        }
    }

    #[inline]
    pub fn rot_yz(&self, theta: impl Into<Angle>) -> Self {
        let theta = theta.into();

        let sin = theta.sin();
        let cos = theta.cos();

        let rot = ten![
            [1.,  0.,   0., 0.],
            [0., cos, -sin, 0.],
            [0., sin,  cos, 0.],
            [0.,  0.,   0., 1.],
        ]; 

        Self {
            mat: compose(&rot, &self.mat),
        }
    }

    pub fn rotate_around_xy(&self, x: f32, y: f32, z: f32, theta: impl Into<Angle>) -> Self {
        self.translate(-x, -y, -z).rot_xy(theta).translate(x, y, z)
    }

    pub fn rotate_around_xz(&self, x: f32, y: f32, z: f32, theta: impl Into<Angle>) -> Self {
        self.translate(-x, -y, -z).rot_xz(theta).translate(x, y, z)
    }

    pub fn rotate_around_yz(&self, x: f32, y: f32, z: f32, theta: impl Into<Angle>) -> Self {
        self.translate(-x, -y, -z).rot_yz(theta).translate(x, y, z)
    }

    pub fn matmul(&self, y: &Affine3d) -> Self {
        Self {
            mat: compose(&self.mat, &y.mat),
        }
    }

    pub fn transform(&self, points: &Tensor) -> Tensor {
        assert!(points.rank() == 2);
        assert!(points.cols() == 3);

        let mat = self.mat.as_slice();

        points.map_row(|point| {
            let x = point[0];
            let y = point[1];
            let z = point[2];

            [
                x * mat[0] + y * mat[1] + z * mat[2] + mat[3],
                x * mat[4] + y * mat[5] + z * mat[6] + mat[7],
                x * mat[8] + y * mat[9] + z * mat[10] + mat[11],
            ]
        })
    }
}

impl fmt::Debug for Affine3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Affine3d").field("mat", &self.mat).finish()
    }
}

impl From<&Affine2d> for Affine3d {
    fn from(value: &Affine2d) -> Self {
        let mat = value.mat();

        Affine3d::new(
            mat[0], mat[1], 0., mat[2],
            mat[3], mat[4], 0., mat[5],
            0., 0., 1., 0.,
        )
    }
}

fn compose(x: &Tensor, y: &Tensor) -> Tensor {
    // assert_eq!(x.shape().as_slice(), &[4, 4]);
    // assert_eq!(y.shape().as_slice(), &[4, 4]);
    let x = x.as_slice();
    let y = y.as_slice();

    let o = [
        x[0] * y[0] + x[1] * y[4] + x[2] * y[8],
        x[0] * y[1] + x[1] * y[5] + x[2] * y[9],
        x[0] * y[2] + x[1] * y[6] + x[2] * y[10],
        x[0] * y[3] + x[1] * y[7] + x[2] * y[11] + x[3],

        x[4] * y[0] + x[5] * y[4] + x[6] * y[8],
        x[4] * y[1] + x[5] * y[5] + x[6] * y[9],
        x[4] * y[2] + x[5] * y[6] + x[6] * y[10],
        x[4] * y[3] + x[5] * y[7] + x[6] * y[11] + x[7],

        x[8] * y[0] + x[9] * y[4] + x[10] * y[8],
        x[8] * y[1] + x[9] * y[5] + x[10] * y[9],
        x[8] * y[2] + x[9] * y[6] + x[10] * y[10],
        x[8] * y[3] + x[9] * y[7] + x[10] * y[11] + x[11],

        0.,
        0.,
        0.,
        1.,
    ];

    Tensor::from(o).reshape([4, 4])
}
