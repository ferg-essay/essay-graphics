use essay_graphics_api::{Canvas, Bounds, Clip, PathOpt, Path, driver::Renderer};
use essay_tensor::Tensor;

use crate::{frame::{Data, LegendHandler}, graph::ConfigArc, data_artist_option_struct, path_style_options};

use super::{Artist, PathStyle, PlotArtist, PlotId, paths, ToCanvas};

pub struct Quiver {
    x: Tensor,
    y: Tensor,
    u: Tensor,
    v: Tensor,
    style: PathStyle,

    extent: Bounds<Data>,
    paths: Vec<Path<Data>>,

    is_stale: bool,
}

impl Quiver {
    pub fn new(
        x: impl Into<Tensor>,
        y: impl Into<Tensor>,
        u: impl Into<Tensor>,
        v: impl Into<Tensor>,
    ) -> Self {
        let x : Tensor = x.into();
        let y : Tensor = y.into();
        let u : Tensor = u.into();
        let v : Tensor = v.into();

        // let uv = u.stack([v.clone()], ());

        assert_eq!(u.shape(), v.shape(), "quiver requires matching u,v shape. u={:?}, v={:?}", 
            u.shape().as_slice(),
            v.shape().as_slice(),
        );

        Self {
            x,
            y,
            u,
            v,
            style: PathStyle::new(),
            extent: Bounds::<Data>::none(),
            paths: Vec::new(),
            is_stale: true,
        }
    }

    pub(crate) fn uv(&mut self, uv: Tensor) {
        assert!(uv.rank() == 3, "quiver requires rank-3 value {:?}", uv.shape().as_slice());

        //self.uv = uv;
        self.is_stale = true;
    }

    fn draw_arrow(x: f32, y: f32, u: f32, v: f32) -> Path::<Data> {
        // TODO: check if size should try to match area instead of
        // arrow length

        let wt = 0.07;
        let xt = v * wt;
        let yt = -u * wt;

        let wh = 0.25;
        let xh = v * wh;
        let yh = -u * wh;

        let lh = 0.6;
        let uh = lh * u;
        let vh = lh * v;

        if u == 0. && v == 0. {
            return paths::rect((x - 0.1 * wt, y - 0.1 * wt), (x + 0.1 * wt, y + 0.1 * wt))
        }

        Path::move_to(x - xt, y - yt)
            .line_to(x + xt, y + yt)

            .line_to(x + xt + uh, y + yt + vh)
            .line_to(x + xh + uh, y + yh + vh)
            .line_to(x + u, y + v)
            .line_to(x - xh + uh, y - yh + vh)

            .close_poly(x - xt + uh, y - yt + vh)
            .to_path()
    }
}

impl Artist<Data> for Quiver {
    fn update(&mut self, _canvas: &Canvas) {
        if self.is_stale {
            self.is_stale = false;

            let x_min = self.x.reduce_min()[0];
            let x_max = self.x.reduce_max()[0];

            let y_min = self.y.reduce_min()[0];
            let y_max = self.y.reduce_max()[0];

            self.extent = Bounds::new((x_min, y_min), (x_max, y_max));

            let magnitude = self.u.hypot(&self.v);
            let max = magnitude.reduce_max()[0].max(f32::EPSILON);

            let mut paths = Vec::<Path<Data>>::new();

            let dx = (self.x[1] - self.x[0]) / max;
            let dy = (self.y[1] - self.y[0]) / max;

            for (j, y) in self.y.iter().enumerate() {
                for (i, x) in self.x.iter().enumerate() {
                    paths.push(Self::draw_arrow(*x, *y, self.u[(j, i)] * dx, self.v[(j, i)] * dy));
                }
            }

            self.paths = paths;
        }
    }
    
    fn get_extent(&mut self) -> Bounds<Data> {
        self.extent.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let style = self.style.push(style);

        for path in &self.paths {
            let c_path: Path<Canvas> = path.transform(to_canvas);
            renderer.draw_path(&c_path, &style, clip).unwrap();
        }
    }
}

impl PlotArtist<Data> for Quiver {
    type Opt = QuiverOpt;

    fn config(&mut self, cfg: &ConfigArc, id: PlotId) -> Self::Opt {
        self.style = PathStyle::from_config(cfg, "quiver");

        // TODO: when Cycle is changed, this shouldn't be necessary
        if self.style.get_face_color().is_none() {
            self.style.color("k");
        }

        unsafe { QuiverOpt::new(id) }
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

data_artist_option_struct!(QuiverOpt, Quiver);

impl QuiverOpt {
    path_style_options!(style);

    pub fn data(&mut self, data: impl Into<Tensor>) -> &mut Self {
        let data : Tensor = data.into();

        self.write(|artist| {
            artist.uv(data);
        });

        self
    }
}
