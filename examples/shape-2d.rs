use renderer::{Canvas, Drawable, Renderer};
use essay_graphics::{layout::Layout, prelude::*};
use essay_graphics_wgpu::WgpuMainLoop;
use essay_tensor::Tensor;
use form::{Shape, ShapeId};

fn main() { 
    let mut layout = Layout::new();

    let mut form = Shape::new();
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

    layout.view(((0.5, 0.5), [0.5, 0.5]),
        ShapeView::new(form, texture_colors(&[
            Color::from("red"),
            Color::from("blue"),
            Color::from("orange"),
            Color::from("teal"),
        ]))
    );

    WgpuMainLoop::new().main_loop(Box::new(layout)).unwrap();
}

fn square(
    form: &mut Shape, 
    vertices: [[f32; 2]; 4],
    v: f32,
) {
    let x0 = 0.5;
    let x1 = 0.5;
    let y0 = v;
    let y1 = v;

    form.vertex(vertices[0], [x0, y0]);
    form.vertex(vertices[1], [x0, y1]);
    form.vertex(vertices[2], [x1, y0]);
    form.vertex(vertices[3], [x1, y1]);
    form.vertex(vertices[2], [x1, y1]);
    form.vertex(vertices[0], [x1, y1]);
}

struct ShapeView {
    form: Shape,
    form_id: Option<ShapeId>,
    texture: Tensor<u8>,

    is_dirty: bool,
}

impl ShapeView {
    fn new(form: Shape, texture: Tensor<u8>) -> Self {
        Self {
            form,
            form_id: None,
            texture,
            is_dirty: true,
        }
    }

    fn fill_model(&mut self, renderer: &mut dyn Renderer) {
        let texture = renderer.create_texture_rgba8(&self.texture);

        self.form.texture(texture);

        self.form_id = Some(renderer.create_shape(&self.form));
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

        if let Some(id) = self.form_id {
            let canvas = renderer.pos().clone();
            let bounds = Bounds::<Canvas>::from(((0., 0.), [1., 1.]));
            let camera = bounds.affine_to(&canvas);
            
            renderer.draw_shape(
                id,
                &camera,
            )?;
        }

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