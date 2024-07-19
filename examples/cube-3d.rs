use driver::Renderer;
use essay_graphics::{layout::{LayoutMainLoop, ViewTrait}, prelude::*};
use essay_tensor::Tensor;
use form::{Form, FormId, Matrix4};

fn main() { 
    let mut figure = LayoutMainLoop::new();

    let mut form = Form::new();
    // let mut vertices = Vec::<[f32; 3]>::new();
    square(&mut form, [
        [-1., -1., -1.],
        [-1., -1., 1.],
        [-1., 1., -1.],
        [-1., 1., 1.]
    ], 0.1);

    square(&mut form, [
        [1., -1., -1.],
        [1., -1., 1.],
        [1., 1., -1.],
        [1., 1., 1.]
    ], 0.3);

    square(&mut form, [
        [-1., -1., -1.],
        [-1., -1., 1.],
        [1., -1., -1.],
        [1., -1., 1.]
    ], 0.6);

    square(&mut form, [
        [-1., 1., -1.],
        [-1., 1., 1.],
        [1., 1., -1.],
        [1., 1., 1.]
    ], 0.8);

    figure.add_view((), 
        CubeView::new(form, texture_colors(&[
            Color::from("red"),
            Color::from("blue"),
            Color::from("orange"),
            Color::from("teal"),
        ]))
    );

    // println!("Path {:?} ", view.read(|t| t.path()));

    figure.show();
}

fn square(
    form: &mut Form, 
    vertices: [[f32; 3]; 4],
    //uv0: [f32; 2],
    //uv1: [f32; 2],
    v: f32,
) {
    let x0 = 0.5;
    let x1 = 0.5;
    let y0 = v;
    let y1 = v;

    let v0 = form.vertex(vertices[0], [x0, y0]);
    let v1 = form.vertex(vertices[1], [x0, y1]);
    let v2 = form.vertex(vertices[2], [x1, y0]);
    let v3 = form.vertex(vertices[3], [x1, y1]);

    form.triangle([v0, v1, v3]);
    form.triangle([v3, v2, v0]);

}

struct CubeView {
    form: Form,
    form_id: Option<FormId>,
    texture: Tensor<u8>,
    
    camera: Camera,


    is_dirty: bool,
}

impl CubeView {
    fn new(form: Form, texture: Tensor<u8>) -> Self {
        Self {
            form,
            form_id: None,
            camera: Camera::new([0., 0.2, -4.]),
            texture,
            is_dirty: true,
        }
    }

    fn fill_model(&mut self, renderer: &mut dyn Renderer) {
        let texture = renderer.create_texture_rgba8(&self.texture);

        self.form.texture(texture);

        self.form_id = Some(renderer.create_form(&self.form));
    }

    fn camera(&self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) -> Matrix4 {
        let matrix = self.camera.matrix();
        let bounds = renderer.bounds();
        let to = Matrix4::view_to_canvas_unit(pos, bounds);

        to.matmul(&matrix)
    }
}

impl ViewTrait for CubeView {
    // fn update_pos(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
    // }

    fn draw(&mut self, renderer: &mut dyn driver::Renderer, pos: &Bounds<Canvas>) {
        if self.is_dirty {
            self.is_dirty = false;
            self.fill_model(renderer);
        }

        if let Some(id) = self.form_id {
            let pos = Bounds::<Canvas>::new(
                (0.5 * pos.xmax(), 0.5 * pos.ymax()),
                (pos.xmax(), pos.ymax())
            );
            let camera = self.camera(renderer, &pos);

            renderer.draw_form(
                id,
                &camera,
                &Clip::Bounds(pos.p0(), pos.p1())
            ).unwrap();
        }
    }

    fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
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
    rot: Matrix4,
}

impl Camera {
    fn new(eye: [f32; 3]) -> Self {
        Self {
            eye: eye.into(),
            rot: Matrix4::eye(),
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
        self.rot = self.rot.rot_xz(yaw);
    }

    fn matrix(&self) -> Matrix4 {
        let mut camera = Matrix4::eye();

        camera = camera.translate(self.eye[0], self.eye[1], self.eye[2]);
        camera = self.rot.matmul(&camera);

        //let fov = 45.0f32;
        let fov = 150.0f32;
        camera = camera.projection(fov.to_radians(), 1., 0.1, 100.);

    
        // let view = pos.affine_to(renderer.bounds());
        // let scale = pos.height();
        //camera = camera.matmul(&view);

        camera
    }
}

fn texture_colors(colors: &[Color]) -> Tensor<u8> {
    let mut vec = Vec::<[u8; 4]>::new();

    let size = 8;
    for color in colors {
        for _ in 0..size * size {
            vec.push(color.to_rgba_vec());
        }
    }

    Tensor::from(vec).reshape([colors.len() * size, size, 4])
}