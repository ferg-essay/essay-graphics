use essay_graphics_api::{driver::Renderer, Affine2d, Bounds, Canvas, CapStyle, Clip, Color, JoinStyle, Path, PathOpt, Point};
use essay_tensor::{Tensor, tensor::TensorVec, tf32, math::normalize_unit};

use crate::{frame::{Data, LegendHandler}, artist::{Norm, Norms}, graph::ConfigArc, data_artist_option_struct};

use super::{Artist, ColorMap, ColorMaps, PathStyle, PlotArtist, PlotId, ToCanvas};

pub enum Shading {
    Flat,
    Gouraud,
}

pub struct GridColor {
    data: Tensor,
    xy: Tensor,
    color_map: ColorMap,
    shading: Shading,
    norm: Norm,

    is_stale: bool,
}

impl GridColor {
    pub fn new(data: impl Into<Tensor>) -> Self {
        let data : Tensor = data.into();

        assert!(data.rank() == 2, "colormesh requires 2d value {:?}", data.shape().as_slice());

        Self {
            data,
            xy: Tensor::empty(),
            color_map: ColorMaps::Default.into(),
            shading: Shading::Flat,
            norm: Norm::from(Norms::Linear),
            is_stale: true,
        }
    }

    pub(crate) fn set_data(&mut self, data: Tensor) {
        assert!(data.rank() == 2, "colormesh requires 2d value {:?}", data.shape().as_slice());

        self.data = data;
        self.is_stale = true;
    }

    pub(crate) fn norm(&mut self, norm: impl Into<Norm>) {
        self.norm = norm.into();
    }

    pub(crate) fn color_map(&mut self, color_map: impl Into<ColorMap>) {
        self.color_map = color_map.into();
    }

    pub(crate) fn shading(&mut self, shading: impl Into<Shading>) {
        self.shading = shading.into();
    }

    fn draw_solid_shading(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &Affine2d,
        clip: &Clip,
        _style: &dyn PathOpt,
    ) {
        let path = Path::<Data>::closed_poly(tf32!([
            [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
            [0.0, 1.0]
            ]));
            
        let scale_canvas = to_canvas.strip_translation();
        let path: Path<Canvas> = path.transform(&scale_canvas);
        let xy = to_canvas.transform(&self.xy);

        // let norm = normalize_unit(&self.data);

        let colormap = &self.color_map;

        let mut colors = TensorVec::<u32>::new();
        for v in self.data.iter() {
            let v = self.norm.norm(*v);
            colors.push(colormap.map(v).to_rgba());
        }

        let colors = colors.into_tensor();

        let mut style = PathStyle::new();
        style.edge_color(Color(0));
        style.line_width(0.);

        // style.edge_color("k");
        style.join_style(JoinStyle::Bevel);
        style.cap_style(CapStyle::Butt);

        renderer.draw_markers(&path, &xy, &tf32!(), &colors, &style, clip).unwrap();
    }

    fn draw_gouraud_shading(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &Affine2d,
        clip: &Clip,
        _style: &dyn PathOpt,
    ) {
        let xy = to_canvas.transform(&self.xy);

        let norm = normalize_unit(&self.data);
        let cmap = &self.color_map;

        let mut vertices = TensorVec::<[f32; 2]>::new();
        let mut colors = TensorVec::<u32>::new();
        let mut triangles = TensorVec::<[u32; 3]>::new();

        let (rows, cols) = (norm.rows(), norm.cols());

        let j_stride = cols; //  + 1;

        for j in 0..rows {
            for i in 0..cols {
                let x0 = xy[(j * j_stride + i, 0)];
                let y0 = xy[(j * j_stride + i, 1)];
                
                vertices.push([x0, y0]);
                colors.push(cmap.map(norm[(j, i)]).to_rgba());

                if i + 1 < cols && j + 1 < rows {
                    triangles.push([
                        (j * j_stride + i) as u32, 
                        (j * j_stride + i + 1) as u32, 
                        ((j + 1) * j_stride + i + 1) as u32,
                    ]);

                    triangles.push([
                        ((j + 1) * j_stride + i + 1) as u32, 
                        ((j + 1) * j_stride + i) as u32, 
                        (j * j_stride + i) as u32,
                    ]);
                }
            }
        }

        let vertices = vertices.into_tensor();
        let colors = colors.into_tensor();
        let triangles = triangles.into_tensor();

        renderer.draw_triangles(vertices, colors, triangles, clip).unwrap();
    }

    pub(crate) fn set_norm(&mut self, min: f32, max: f32) {
        self.norm.set_vmin(min);
        self.norm.set_vmax(max);
    }
}

impl Artist<Data> for GridColor {
    fn update(&mut self, _canvas: &Canvas) {
        if self.is_stale {
            self.is_stale = false;

            let mut xy = TensorVec::<[f32; 2]>::new();
            let (rows, cols) = (self.data.rows(), self.data.cols());

            for j in 0..rows {
                for i in 0..cols {
                    xy.push([i as f32, j as f32]);
                }
            }

            self.xy = xy.into_tensor();
            self.norm.set_bounds(&self.data);
            println!("Bounds: {:?} {:?}", self.norm.min(), self.norm.max());
        }
    }
    
    fn get_extent(&mut self) -> Bounds<Data> {
        let (rows, cols) = match self.shading {
            Shading::Gouraud => (self.data.rows() - 1, self.data.cols() - 1),
            Shading::Flat => (self.data.rows(), self.data.cols())
        };

        Bounds::new(
            Point(0.0, 0.0), 
            Point(cols as f32, rows as f32)
        )
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        match self.shading {
            Shading::Gouraud => {
                self.draw_gouraud_shading(renderer, to_canvas, clip, style);
            },
            Shading::Flat => {
                self.draw_solid_shading(renderer, to_canvas, clip, style);
            }
        }
    }
}

impl PlotArtist<Data> for GridColor {
    type Opt = GridColorOpt;

    fn config(&mut self, _cfg: &ConfigArc, id: PlotId) -> Self::Opt {
        // self.style = PathStyle::from_config(cfg, "color_grid");

        unsafe { GridColorOpt::new(id) }
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

data_artist_option_struct!(GridColorOpt, GridColor);

impl GridColorOpt {
    pub fn data(&mut self, data: impl Into<Tensor>) -> &mut Self {
        let data = data.into();
        assert!(data.rank() == 2, "ColorGrid data must be rank-2. Shape={:?}", data.shape().as_slice());

        self.write(|artist| {
            artist.data = data;
            artist.is_stale = true;
        });

        self
    }

    pub fn norm(&mut self, norm: impl Into<Norm>) -> &mut Self {
            self.write(|artist| {
            artist.norm = norm.into();
            artist.is_stale = true;
        });

        self
    }

    pub fn color_map(&mut self, cmap: impl Into<ColorMap>) -> &mut Self {
        self.write(|artist| {
        artist.color_map = cmap.into();
        artist.is_stale = true;
    });

    self
}

    pub fn shading(&mut self, shading: Shading) -> &mut Self {
        self.write(|artist| {
        artist.shading = shading;
        artist.is_stale = true;
    });

    self
}
}
