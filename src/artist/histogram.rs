use essay_graphics_api::{Canvas, Bounds, Clip, PathOpt, Path, driver::Renderer};
use essay_tensor::Tensor;

use crate::{frame::{Data, LegendHandler}, graph::ConfigArc, data_artist_option_struct, path_style_options};

use super::{Artist, PathStyle, PlotArtist, PlotId, paths, ToCanvas};

pub struct Histogram {
    data: Tensor,
    style: PathStyle,

    bins: Tensor,
    count: Tensor,
    extent: Bounds<Data>,
    paths: Vec<Path<Data>>,

    is_stale: bool,
}

impl Histogram {
    pub fn new(data: impl Into<Tensor>) -> Self {
        let data : Tensor = data.into();

        assert!(data.rank() == 1, "histogram requires 1D value {:?}", data.shape().as_slice());

        Self {
            data,
            style: PathStyle::new(),
            bins: Tensor::zeros([1]),
            count: Tensor::zeros([1]),
            extent: Bounds::<Data>::none(),
            paths: Vec::new(),
            is_stale: true,
        }
    }

    pub(crate) fn _set_data(&mut self, data: Tensor) {
        assert!(data.rank() == 1, "histogram requires 1D value {:?}", data.shape().as_slice());

        self.data = data;
        self.is_stale = true;
    }
}

impl Artist<Data> for Histogram {
    fn update(&mut self, _canvas: &Canvas) {
        if self.is_stale {
            self.is_stale = false;

            let (count, bins) = essay_tensor::stats::histogram(&self.data, ());

            self.count = count;
            self.bins = bins;

            let (min, max) = (self.bins[0], self.bins[self.bins.len() - 1]);
            let c_max = self.count.reduce_max()[0];

            self.extent = Bounds::new((min, 0.), (max, c_max));

            let mut paths = Vec::<Path<Data>>::new();

            for i in 0..self.count.len() {
                paths.push(paths::rect(
                    (self.bins[i], 0.), 
                    (self.bins[i + 1], self.count[i])
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

impl PlotArtist<Data> for Histogram {
    type Opt = HistogramOpt;

    fn config(&mut self, cfg: &ConfigArc, id: PlotId) -> Self::Opt {
        self.style = PathStyle::from_config(cfg, "histogram");

        unsafe { HistogramOpt::new(id) }
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

data_artist_option_struct!(HistogramOpt, Histogram);

impl HistogramOpt {
    path_style_options!(style);

    pub fn data(&mut self, data: impl Into<Tensor>) -> &mut Self {
        let data = data.into();
        assert!(data.rank() == 2, "Histogram data must be 1D. Shape={:?}", data.shape().as_slice());

        self.write(|artist| {
            artist.data = data;
            artist.is_stale = true;
        });

        self
    }
}
