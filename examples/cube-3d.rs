use affine3d::Affine3d;
use essay_graphics::{layout::{LayoutMainLoop, ViewTrait}, prelude::*};
use essay_tensor::Tensor;
use matrix4::Matrix4;

fn main() { 
    let mut figure = LayoutMainLoop::new();

    let mut vertices = Vec::<[f32; 3]>::new();
    vertices.push([-1., -1., -1.]);
    vertices.push([-1., -1., 1.]);
    vertices.push([-1., 1., -1.]);
    vertices.push([-1., 1., 1.]);
    vertices.push([1., -1., -1.]);
    vertices.push([1., -1., 1.]);
    vertices.push([1., 1., -1.]);
    vertices.push([1., 1., 1.]);

    let mut triangles = Vec::<[u32; 3]>::new();
    triangles.push([0, 1, 3]);
    triangles.push([3, 2, 0]);

    triangles.push([4, 5, 7]);
    triangles.push([7, 6, 4]);

    triangles.push([0, 2, 6]);
    triangles.push([6, 4, 0]);

    triangles.push([1, 3, 7]);
    triangles.push([7, 5, 1]);

    figure.add_view((), 
        CubeView::new(vertices, triangles)
    );

    // println!("Path {:?} ", view.read(|t| t.path()));

    figure.show();
}

struct CubeView {
    vertices: Tensor,
    triangles: Tensor<u32>,
    camera: Matrix4,
}

impl CubeView {
    fn new(vertices: impl Into<Tensor>, triangles: impl Into<Tensor<u32>>) -> Self {
        Self {
            vertices: vertices.into(),
            triangles: triangles.into(),
            camera: Matrix4::eye(),
        }
    }
}

impl ViewTrait for CubeView {
    fn update(&mut self, pos: &Bounds<Canvas>, canvas: &Canvas) {
        let mut camera = Matrix4::eye();


        camera = camera.scale(0.25, 0.25, 0.25);
        camera = camera.rot_xz(Angle::Deg(45.));
        //camera = camera.rot_yz(Angle::Deg(45.));
        camera = camera.translate(0., 0., -1.0);
        //camera = camera.translate(0., 0., -5.);
        // camera = camera.scale(scale, scale, 1.);
        // camera = camera.scale(0.5, 0.5, 1.);
        let fov = 45.0f32;
        camera = camera.projection(fov.to_radians(), 1., 0.1, 100.);

        
        let view = pos.affine_to(canvas.bounds());
        // let scale = pos.height();
        //camera = camera.matmul(&view);

        self.camera = camera;
    }

    fn draw(&mut self, renderer: &mut dyn driver::Renderer) {
        renderer.draw_3d(
            self.vertices.clone(), 
            self.triangles.clone(), 
            Color::from("teal"),
            &self.camera,
            &Clip::None
        ).unwrap();
    }
}
