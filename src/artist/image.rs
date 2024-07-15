use essay_graphics_api::driver::Renderer;
use essay_graphics_api::{Canvas, Bounds, Point, Clip, PathOpt};
use essay_tensor::Tensor;

use crate::data_artist_option_struct;
use crate::frame::LegendHandler;
use crate::graph::ConfigArc;
use crate::{frame::Data, artist::{Norms, Norm}};

use super::{Artist, ColorMap, ColorMaps, PlotArtist, PlotId, ToCanvas};

pub struct Image {
    data: Tensor,
    norm: Norm,
    color_map: ColorMap,
    extent: Option<Bounds<Data>>,
    is_stale: bool,
}

impl Image {
    pub fn new(data: impl Into<Tensor>) -> Self {
        let data : Tensor = data.into();

        assert!(data.rank() == 2, "image requires 2d value {:?}", data.shape().as_slice());

        Self {
            data,
            norm: Norm::from(Norms::Linear),
            color_map: ColorMaps::Default.into(), // ColorMaps::Default.into(),
            extent: None,
            is_stale: true,
        }
    }

    pub fn norm(&mut self, norm: impl Into<Norm>) -> &mut Self {
        self.norm = norm.into();

        self
    }

    pub fn color_map(&mut self, cmap: impl Into<ColorMap>) -> &mut Self {
        self.color_map = cmap.into();

        self
    }
}

impl Artist<Data> for Image {
    fn update(&mut self, _canvas: &Canvas) {
        self.norm.set_bounds(&self.data);
    }
    
    fn get_extent(&mut self) -> Bounds<Data> {
        match &self.extent {
            Some(extent) => extent.clone(),
            None => {
                let (rows, cols) = (self.data.rows(), self.data.cols());

               Bounds::new(
                    Point(0.0, 0.),
                    Point(cols as f32, rows as f32),
                )
            }
        }
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        _style: &dyn PathOpt,
    ) {
        //let to_canvas = to_canvas.translate(0., self.).scale(1., -1.);
        let extent = self.get_extent();
        let bounds = Bounds::new(
            to_canvas.transform_point(extent.p0()),
            to_canvas.transform_point(extent.p1()),
        );
    
        let norm = &self.norm;
        let colormap = &self.color_map;
    
        let mut colors = Vec::<u8>::new();
        for v in self.data.iter() {
            let v = norm.norm(*v);
            let color = colormap.map(v);
            colors.push(color.r8());
            colors.push(color.g8());
            colors.push(color.b8());
            colors.push(color.a8());
        }
    
        // todo [width, height, 4]
        let colors = Tensor::from(colors).reshape([self.data.rows(), self.data.cols(), 4]);
    
        renderer.draw_image(&bounds, &colors, clip).unwrap();
    }
}

impl PlotArtist<Data> for Image {
    type Opt = ImageOpt;

    fn config(&mut self, _cfg: &ConfigArc, id: PlotId) -> Self::Opt {
        unsafe { ImageOpt::new(id) }
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

data_artist_option_struct!(ImageOpt, Image);

impl ImageOpt {
    pub fn data(&mut self, data: impl Into<Tensor>) -> &mut Self {
        let data = data.into();
        assert!(data.rank() == 2, "Image data must be rank 2. Shape={:?}", data.shape().as_slice());

        self.write(|artist| {
            artist.data = data;
            artist.is_stale = true;
        });

        self
    }

    pub fn extent(&mut self, extent: impl Into<Bounds<Data>>) -> &mut Self {
        self.write(|artist| {
            artist.extent = Some(extent.into());
        });

        self
    }

    pub fn norm(&mut self, norm: impl Into<Norm>) -> &mut Self {
            self.write(|artist| {
            artist.norm(norm);
        });

        self
    }

    pub fn color_map(&mut self, cmap: impl Into<ColorMap>) -> &mut Self {
        self.write(|artist| {
           artist.color_map(cmap);
        });

        self
    }
}
