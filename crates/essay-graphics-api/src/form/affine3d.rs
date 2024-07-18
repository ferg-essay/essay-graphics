use core::fmt;

use essay_tensor::{prelude::*, tensor::TensorUninit};

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
        let mat = tf32!([
            [x00, x01, x02, x03],
            [x10, x11, x12, x13],
            [x20, x21, x22, x23],
            [ 0.,  0.,  0.,  1.],
        ]); 

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
        let mat = tf32!([
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
        let translate = tf32!([
            [1., 0., 0.,  x],
            [0., 1., 0.,  y],
            [0., 0., 1.,  z],
            [0., 0., 0., 1.],
        ]); 

        // TODO: optimize
        Self {
            mat: compose(&translate, &self.mat),
        }
    }

    #[inline]
    pub fn scale(&self, sx: f32, sy: f32, sz: f32) -> Self {
        let scale = tf32!([
            [sx, 0., 0., 0.],
            [0., sy, 0., 0.],
            [0., 0., sz, 0.],
            [0., 0., 0., 1.],
        ]); 

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

        let rot = tf32!([
            [cos, -sin, 0., 0.],
            [sin,  cos, 0., 0.],
            [ 0.,   0., 1., 0.],
            [ 0.,   0., 0., 1.],
        ]); 

        Self {
            mat: compose(&rot, &self.mat),
        }
    }

    #[inline]
    pub fn rot_xz(&self, theta: impl Into<Angle>) -> Self {
        let theta = theta.into();

        let sin = theta.sin();
        let cos = theta.cos();

        let rot = tf32!([
            [cos, 0., -sin, 0.],
            [ 0., 1.,   0., 0.],
            [sin, 0.,  cos, 0.],
            [ 0., 0.,   0., 1.],
        ]); 

        Self {
            mat: compose(&rot, &self.mat),
        }
    }

    #[inline]
    pub fn rot_yz(&self, theta: impl Into<Angle>) -> Self {
        let theta = theta.into();

        let sin = theta.sin();
        let cos = theta.cos();

        let rot = tf32!([
            [1.,  0.,   0., 0.],
            [0., cos, -sin, 0.],
            [0., sin,  cos, 0.],
            [0.,  0.,   0., 1.],
        ]); 

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

        let n = points.rows();

        unsafe {
            let mut out = TensorUninit::<f32>::new(3 * n);

            let mat = self.mat.as_slice();
            let xyz = points.as_slice();
            let o = out.as_mut_slice();

            for i in 0..n {
                let row = 3 * i;

                let x = xyz[row];
                let y = xyz[row + 1];
                let z = xyz[row + 2];

                o[row + 0] = x * mat[0] + y * mat[1] + z * mat[2] + mat[3];
                o[row + 1] = x * mat[4] + y * mat[5] + z * mat[6] + mat[7];
                o[row + 2] = x * mat[8] + y * mat[9] + z * mat[10] + mat[11];
            }

            Tensor::from_uninit(out, points.shape())
        }
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
    assert_eq!(x.shape().as_slice(), &[4, 4]);
    assert_eq!(y.shape().as_slice(), &[4, 4]);

    unsafe {
        let mut out = TensorUninit::<f32>::new(16);

        let o = out.as_mut_slice();
        let x = x.as_slice();
        let y = y.as_slice();

        o[0] = x[0] * y[0] + x[1] * y[4] + x[2] * y[8];
        o[1] = x[0] * y[1] + x[1] * y[5] + x[2] * y[9];
        o[2] = x[0] * y[2] + x[1] * y[6] + x[2] * y[10];
        o[3] = x[0] * y[3] + x[1] * y[7] + x[2] * y[11] + x[3];

        o[4] = x[4] * y[0] + x[5] * y[4] + x[6] * y[8];
        o[5] = x[4] * y[1] + x[5] * y[5] + x[6] * y[9];
        o[6] = x[4] * y[2] + x[5] * y[6] + x[6] * y[10];
        o[7] = x[4] * y[3] + x[5] * y[7] + x[6] * y[11] + x[7];

        o[8]  = x[8] * y[0] + x[9] * y[4] + x[10] * y[8];
        o[9]  = x[8] * y[1] + x[9] * y[5] + x[10] * y[9];
        o[10] = x[8] * y[2] + x[9] * y[6] + x[10] * y[10];
        o[11] = x[8] * y[3] + x[9] * y[7] + x[10] * y[11] + x[11];

        o[12] = 0.;
        o[13] = 0.;
        o[14] = 0.;
        o[15] = 1.;

        Tensor::from_uninit(out, [4, 4])
    }
}
