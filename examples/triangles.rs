use renderer::{Canvas, Drawable, Renderer};
use essay_graphics::{layout::{MainLoop, Page}, prelude::*};
use essay_tensor::Tensor;

fn main() { 
    let mut page = Page::new();

    page.view(((0., 0.), [0.5, 0.5]),
        TriangleView::new()
    );

    MainLoop::new().show(page);
}

struct TriangleView {
    vertices: Tensor,
    triangles: Tensor<u32>,
}

impl TriangleView {
    fn new() -> Self {
        Self {
            vertices: Tensor::from(Vec::<[f32; 2]>::new()),
            triangles: Tensor::from(Vec::<[u32; 3]>::new()),
        }
    }
}

impl Drawable for TriangleView {
    // fn update_pos(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
    // }

    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        let mut colors = Vec::<u32>::new();

        colors.push(Color::from("teal").to_rgba());
        colors.push(Color::from("red").to_rgba());
        colors.push(Color::from("blue").to_rgba());
        colors.push(Color::from("orange").to_rgba());

        let colors = Tensor::from(colors);

        renderer.draw_triangles(self.vertices.clone(), colors.clone(), self.triangles.clone())?;
        println!("Draw");
        Ok(())
    }

    fn resize(&mut self, _renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) -> Bounds<Canvas> {
        println!("Resize {:?}", pos);
        let (x0, y0) = (pos.xmin(), pos.ymin());
        let (w, h) = (pos.width(), pos.height());
        let (x1, y1) = (x0 + w, y0 + h);

        let mut vertices = Vec::<[f32; 2]>::new();
        let mut triangles = Vec::<[u32; 3]>::new();

        vertices.push([x0, y0]);
        vertices.push([x1, y0]);
        vertices.push([x0, y1]);
        vertices.push([x1, y1]);

        triangles.push([0, 1, 2]);
        triangles.push([2, 3, 1]);

        self.vertices = Tensor::from(vertices);
        self.triangles = Tensor::from(triangles);

        pos.clone()
    }
}
