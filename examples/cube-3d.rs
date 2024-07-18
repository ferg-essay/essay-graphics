use driver::Renderer;
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
    camera: Camera,
    is_dirty: bool,
}

impl CubeView {
    fn new(vertices: impl Into<Tensor>, triangles: impl Into<Tensor<u32>>) -> Self {
        Self {
            vertices: vertices.into(),
            triangles: triangles.into(),
            camera: Camera::new([0., 0.2, -4.]),
            is_dirty: true,
        }
    }

    fn fill_model(&mut self, renderer: &mut dyn Renderer) {

    }

    fn camera(&self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) -> Matrix4 {
        self.camera.matrix()
    }
}

impl ViewTrait for CubeView {
    // fn update_pos(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
    // }

    fn draw(&mut self, renderer: &mut dyn driver::Renderer, pos: &Bounds<Canvas>) {
        if self.is_dirty {
            self.fill_model(renderer);
        }

        let camera = self.camera(renderer, pos);

        renderer.draw_3d(
            self.vertices.clone(), 
            self.triangles.clone(), 
            Color::from("teal"),
            &camera,
            &Clip::None
        ).unwrap();
    }

    fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        println!("Cube {:?}", event);
        match event {
            CanvasEvent::KeyPress(_, 'w') => {
                self.camera.forward(0.1);
                renderer.request_redraw(&Bounds::zero());
            }
            CanvasEvent::KeyPress(_, 's') => {
                self.camera.forward(-0.1);
                renderer.request_redraw(&Bounds::zero());
            }
            CanvasEvent::KeyPress(_, 'a') => {
                self.camera.right(-0.1);
                renderer.request_redraw(&Bounds::zero());
            }
            CanvasEvent::KeyPress(_, 'd') => {
                self.camera.right(0.1);
                renderer.request_redraw(&Bounds::zero());
            }

            CanvasEvent::KeyPress(_, 'q') => {
                self.camera.yaw(Angle::Deg(10.));
                renderer.request_redraw(&Bounds::zero());
            }
            CanvasEvent::KeyPress(_, 'e') => {
                self.camera.yaw(Angle::Deg(-10.));
                renderer.request_redraw(&Bounds::zero());
            }

            CanvasEvent::KeyPress(_, 'r') => {
                self.camera.up(0.1);
                renderer.request_redraw(&Bounds::zero());
            }
            CanvasEvent::KeyPress(_, 'f') => {
                self.camera.up(-0.1);
                renderer.request_redraw(&Bounds::zero());
            }
            _ => {}
        }
    }
}

struct Camera {
    eye: [f32; 3],
    yaw: Angle,
    matrix: Matrix4,
}

impl Camera {
    fn new(eye: [f32; 3]) -> Self {
        Self {
            eye: eye.into(),
            yaw: Angle::Rad(0.),
            matrix: Matrix4::eye(),
        }
    }

    fn forward(&mut self, delta: f32) {
        self.eye = [self.eye[0], self.eye[1], self.eye[2] + delta];
    }

    fn right(&mut self, delta: f32) {
        self.eye = [self.eye[0] - delta, self.eye[1], self.eye[2]];
    }

    fn up(&mut self, delta: f32) {
        self.eye = [self.eye[0], self.eye[1] - delta, self.eye[2]];
    }

    fn yaw(&mut self, yaw: impl Into<Angle>) {
        self.yaw = Angle::Rad(self.yaw.to_radians() + yaw.into().to_radians());
    }

    fn matrix(&self) -> Matrix4 {
        let mut camera = Matrix4::eye();

        camera = camera.translate(self.eye[0], self.eye[1], self.eye[2]);
        camera = camera.rot_xz(self.yaw);

        let fov = 45.0f32;
        camera = camera.projection(fov.to_radians(), 1., 0.1, 100.);

    
        // let view = pos.affine_to(renderer.bounds());
        // let scale = pos.height();
        //camera = camera.matmul(&view);

        camera
    }
}