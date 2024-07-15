use essay_graphics_api::{Point, Canvas, Bounds, driver::Renderer, Clip, PathOpt, Color, CapStyle};
use essay_tensor::{Tensor, init::linspace, tf32};

use crate::frame::Data;

use super::{Artist, grid_color::GridColor, paths, PathStyle, ToCanvas};

pub struct Colorbar {
    bounds: Bounds<Data>,
    pos: Bounds<Canvas>,
    mesh: GridColor,
    data: Tensor,
}

impl Colorbar {
    pub fn new() -> Self {
        Self {
            bounds: Bounds::zero(),
            pos: Bounds::zero(),
            data: tf32!([0., 1.]),
            mesh: GridColor::new(tf32!([[0.]])),
        }
    }

    pub(crate) fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.pos = pos;
    }
}

impl Artist<Data> for Colorbar {
    fn update(&mut self, canvas: &Canvas) {
        let is_triangle = false;
        if is_triangle {
            self.bounds = Bounds::new(Point(0., 0.), Point(2., 100.));
        } else {
            self.bounds = Bounds::new(Point(0., 0.), Point(2., 101.));
        }
        let x = linspace(0., 1., 101);//.reshape([101, 1]);
        self.data = x.stack([x.clone()], -1);
        self.mesh.set_data(self.data.clone());
        self.mesh.update(canvas);
    }

    fn get_extent(&mut self) -> Bounds<Data> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        _to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let to_canvas = ToCanvas::new(
            self.pos.clone(), 
            self.bounds.affine_to(&self.pos)
        );
        // self.mesh.draw(renderer, &to_canvas, clip, style);

        let path = paths::bounds(&self.pos);
        let mut pstyle = PathStyle::new();
        pstyle.face_color(Color(0x0));
        pstyle.edge_color(Color(0xff));
        pstyle.cap_style(CapStyle::Projecting);
        pstyle.line_width(0.7);

        self.mesh.draw(renderer, &to_canvas, clip, style);
        renderer.draw_path(&path, &pstyle, clip).unwrap();
    }
}