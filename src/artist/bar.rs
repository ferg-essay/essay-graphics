use essay_graphics_api::{Canvas, Bounds, Clip, PathOpt, Path, driver::Renderer};
use essay_tensor::{Tensor, init::linspace};

use crate::{frame::{Data, LegendHandler}, graph::ConfigArc, data_artist_option_struct, path_style_options};

use super::{Artist, PathStyle, PlotArtist, PlotId, paths, ToCanvas};

pub struct Bar {
    height: Tensor,

    x: Option<Tensor>,
    width: Option<Tensor>,
    bottom: Option<Tensor>,

    style: PathStyle,

    extent: Bounds<Data>,
    paths: Vec<Path<Data>>,

    is_stale: bool,
}

impl Bar {
    pub fn new(data: impl Into<Tensor>) -> Self {
        let data : Tensor = data.into();

        assert!(data.rank() == 1, "bar requires 1D value {:?}", data.shape().as_slice());

        Self {
            height: data,
            style: PathStyle::new(),

            x: None,
            width: None,
            bottom: None,

            extent: Bounds::<Data>::none(),
            paths: Vec::new(),
            is_stale: true,
        }
    }

    pub(crate) fn data(&mut self, data: impl Into<Tensor>) {
        let data = data.into();

        assert!(self.height.shape() == data.shape(), "bar data shape must match initial data. new={:?} old={:?}",
            data.shape().as_slice(), self.height.shape().as_slice());

        self.height = data;
        self.is_stale = true;
    }

    pub(crate) fn x(&mut self, x: impl Into<Tensor>) {
        let x = x.into();

        assert!(x.shape() == self.height.shape(), 
                "bar x coords must match data. x={:?} data={:?}", 
                x.shape().as_slice(), self.height.shape().as_slice());

        self.x = Some(x);
    }

    pub(crate) fn width(&mut self, width: impl Into<Tensor>) {
        let width = width.into();

        if width.len() == 1 {
            self.width = Some(Tensor::fill(width[0], self.height.len()));
        } else {
            assert!(width.shape() == self.height.shape(), 
                "bar width must match data width={:?} data={:?}", 
                width.shape().as_slice(), self.height.shape().as_slice());

            self.width = Some(width);
        }
    }

    pub(crate) fn bottom(&mut self, bottom: impl Into<Tensor>) {
        let bottom = bottom.into();

        if bottom.len() == 1 {
            self.bottom = Some(Tensor::fill(bottom[0], self.height.len()));
        } else {
            assert!(bottom.shape() == self.height.shape(), 
                "bar bottom must match data. bottom={:?} data={:?}", 
                bottom.shape().as_slice(), self.height.shape().as_slice());

            self.bottom = Some(bottom);
        }
    }
}

impl Artist<Data> for Bar {
    fn update(&mut self, _canvas: &Canvas) {
        if self.is_stale {
            self.is_stale = false;

            let len = self.height.len();

            let x = match &self.x {
                Some(x) => x.clone(),
                None => linspace(0., len as f32 - 1., len)
            };

            let bottom = match &self.bottom {
                Some(bottom) => bottom.clone(),
                None => Tensor::fill(0., len),
            };

            let min = x.reduce_min()[0];
            let max = x.reduce_max()[0];

            let w2 = match &self.width {
                Some(width) => width * 0.5,
                None => Tensor::fill(0.4, [len]),
            };

            let y_min = bottom.reduce_min()[0];
            let y_max = (&bottom + &self.height).reduce_max()[0];

            self.extent = Bounds::new((min - w2[0], y_min), (max + w2[len - 1], y_max));

            let mut paths = Vec::<Path<Data>>::new();

            for i in 0..self.height.len() {
                let x = x[i];

                paths.push(paths::rect(
                    (x - w2[i], bottom[i]), 
                    (x + w2[i], bottom[i] + self.height[i])
                ));
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

impl PlotArtist<Data> for Bar {
    type Opt = BarOpt;

    fn config(&mut self, cfg: &ConfigArc, id: PlotId) -> Self::Opt {
        self.style = PathStyle::from_config(cfg, "bar");

        unsafe { BarOpt::new(id) }
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

data_artist_option_struct!(BarOpt, Bar);

impl BarOpt {
    path_style_options!(style);

    pub fn data(&mut self, data: impl Into<Tensor>) -> &mut Self {
        self.write(|artist| {
            artist.data(data);
        });

        self
    }

    pub fn x(&mut self, x: impl Into<Tensor>) -> &mut Self {
        self.write(|artist| {
            artist.x(x);
        });

        self
    }

    pub fn width(&mut self, width: impl Into<Tensor>) -> &mut Self {
        self.write(|artist| {
            artist.width(width);
        });

        self
    }

    pub fn bottom(&mut self, bottom: impl Into<Tensor>) -> &mut Self {
        self.write(|artist| {
            artist.bottom(bottom);
        });

        self
    }
}
