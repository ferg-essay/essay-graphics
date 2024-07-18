use core::fmt;

use essay_tensor::{prelude::*, tensor::TensorUninit};

use crate::{Affine2d, Angle};

#[derive(Clone)]
pub struct Matrix4 {
    mat: Tensor,
}

impl Matrix4 {
    #[inline]
    pub fn new(mat: impl Into<Tensor>) -> Matrix4 {
        let mat = mat.into();
        // let shape = mat.shape();

        // assert_eq!(shape.mat.shape(), [4, 4]);

        Self {
            mat,
        }
    }

    #[inline]
    pub fn mat(&self) -> Tensor {
        self.mat.clone()
    }

    #[inline]
    pub fn eye() -> Self {
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
            mat: translate.matmul(&self.mat),
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
            mat: scale.matmul(&self.mat)
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
            mat: rot.matmul(&self.mat)
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
            mat: rot.matmul(&self.mat),
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
            mat: rot.matmul(&self.mat),
        }
    }

    #[inline]
    pub fn rotate_around_xy(&self, x: f32, y: f32, z: f32, theta: impl Into<Angle>) -> Self {
        self.translate(-x, -y, -z).rot_xy(theta).translate(x, y, z)
    }

    #[inline]
    pub fn rotate_around_xz(&self, x: f32, y: f32, z: f32, theta: impl Into<Angle>) -> Self {
        self.translate(-x, -y, -z).rot_xz(theta).translate(x, y, z)
    }

    #[inline]
    pub fn rotate_around_yz(&self, x: f32, y: f32, z: f32, theta: impl Into<Angle>) -> Self {
        self.translate(-x, -y, -z).rot_yz(theta).translate(x, y, z)
    }

    #[inline]
    pub fn projection(&self, fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let uh = (0.5 * fov).tan().recip();
        let uw = uh / aspect;
        let f_depth = far / (far - near);
        let fn_depth = far * near / (far - near);

        /*
        let project = tf32!([
            [uw, 0.,       0., 0.],
            [0., uh,       0., 0.],
            [0., 0., f_depth, -fn_depth],
            [0., 0., 1., 0.],
        ]);
        */ 

        let project = tf32!([
            [uw, 0.,       0., 0.],
            [0., uh,       0., 0.],
            [0., 0., - f_depth, -fn_depth],
            [0., 0., - 1., 0.],
        ]); 

        
        /*
        let project = tf32!([
            [uw, 0.,       0., 0.],
            [0., uh,       0., 0.],
            [0., 0., -f_depth, -1.],
            [0., 0., - fn_depth, 0.],
        ]);
        */

        // TODO: optimize
        Self {
            mat: project.matmul(&self.mat)
        }
    }

    pub fn matmul(&self, y: impl Into<Matrix4>) -> Self {
        Self {
            mat: self.mat.matmul(&y.into().mat)
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

                let w = x * mat[12] + y * mat[13] + z * mat[14] + mat[15];
                let f = w.recip();

                o[row + 0] = f * (x * mat[0] + y * mat[1] + z * mat[2] + mat[3]);
                o[row + 1] = f * (x * mat[4] + y * mat[5] + z * mat[6] + mat[7]);
                o[row + 2] = f * (x * mat[8] + y * mat[9] + z * mat[10] + mat[11]);
            }

            Tensor::from_uninit(out, points.shape())
        }
    }
}

impl fmt::Debug for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Affine3d").field("mat", &self.mat).finish()
    }
}

impl From<&Affine2d> for Matrix4 {
    fn from(value: &Affine2d) -> Self {
        let tensor = value.mat();
        let mat = tensor.as_slice();

        Matrix4::new([
            [mat[0], mat[1], 0., mat[2]],
            [mat[3], mat[4], 0., mat[5]],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
    }
}

impl From<&Matrix4> for [[f32; 4]; 4] {
    fn from(value: &Matrix4) -> Self {
        let mat = value.mat.as_slice();
        /*
        [
            [mat[0], mat[1], mat[2], mat[3]],
            [mat[4], mat[5], mat[6], mat[7]],
            [mat[8], mat[9], mat[10], mat[11]],
            [mat[12], mat[13], mat[14], mat[15]],
        ]
        */
        [
            [mat[0], mat[4], mat[8], mat[12]],
            [mat[1], mat[5], mat[9], mat[13]],
            [mat[2], mat[6], mat[10], mat[14]],
            [mat[3], mat[7], mat[11], mat[15]],
        ]
    }
}
