use essay_tensor::tensor::Tensor;
use renderer::{Canvas, Drawable, Renderer};
use essay_graphics::{layout::MainLoop, prelude::*};
use form::{Shape, ShapeId};

fn main() { 
    let mut form = Mesh2d::new();
    // let mut vertices = Vec::<[f32; 3]>::new();
    square(&mut form, [
        [0., 0.],
        [0., 0.5],
        [0.5, 0.5],
        [0.5, 0.]
    ], 0.1);

    square(&mut form, [
        [0.5, 0.5],
        [0.5, 1.],
        [1., 1.],
        [1., 0.5]
    ], 0.3);

    let view = ShapeView::new(form, texture_colors(&[
            Color::from("red"),
            Color::from("blue"),
            Color::from("orange"),
            Color::from("teal"),
        ]));

    MainLoop::new().show(view);
}

fn square(
    form: &mut Mesh2d, 
    vertices: [[f32; 2]; 4],
    v: f32,
) {
    let x0 = 0.5;
    let x1 = 0.5;
    let y0 = v;
    let y1 = v;

    form.triangle_uv(
        (vertices[0], [x0, y0]),
        (vertices[1], [x0, y1]),
        (vertices[2], [x1, y0])
    );

    form.triangle_uv(
        (vertices[3], [x1, y1]),
        (vertices[2], [x1, y1]),
        (vertices[0], [x1, y1])
    );
}

struct ShapeView {
    form: Mesh2d,
    texture: Tensor<u8>,
    texture_id: TextureId,

    is_dirty: bool,
}

impl ShapeView {
    fn new(form: Mesh2d, texture: Tensor<u8>) -> Self {
        Self {
            form,
            texture,
            texture_id: TextureId::default(),
            is_dirty: true,
        }
    }

    fn fill_model(&mut self, renderer: &mut dyn Renderer) {
        self.texture_id = renderer.create_texture_rgba8(&self.texture);

        // self.form.texture(texture);

        // self.form_id = Some(renderer.create_shape(&self.form));
    }
}

impl Drawable for ShapeView {
    // fn update_pos(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
    // }

    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        if self.is_dirty {
            self.is_dirty = false;
            self.fill_model(renderer);
        }

        let canvas = renderer.pos().clone();
        let bounds = Bounds::<Canvas>::from([1., 1.]);
        let camera = bounds.affine_to(&canvas);
        // &camera,
        let color = Color::white();

        renderer.draw_mesh2d(
            &self.form,
            self.texture_id,
            &[(color, camera).into()],
        )?;

        Ok(())
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
