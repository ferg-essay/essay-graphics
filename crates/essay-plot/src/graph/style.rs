use essay_plot_base::{Coord, Canvas, Bounds, driver::Renderer, Affine2d, Clip, PathOpt};

use crate::{
    artist::{Artist, PathStyle},
    frame::{Data, ArtistId, LayoutArc, FrameId}, 
    data_artist_option_struct, path_style_options,
};

use super::ConfigArc;

data_artist_option_struct!(PlotOpt, PlotOptArtist<Data>);

impl PlotOpt {
    path_style_options!(style);
}

pub struct PlotOptArtist<M: Coord> {
    artist: Box<dyn Artist<M>>,
    style: PathStyle,
}

impl<M: Coord> PlotOptArtist<M> {
    pub fn new<A>(artist: A) -> Self
    where
        A: Artist<M> + 'static
    {
        Self {
            artist: Box::new(artist),
            style: PathStyle::new(),
        }
    }
}

impl<M: Coord> Artist<M> for PlotOptArtist<M> {
    fn update(&mut self, canvas: &Canvas) {
        self.artist.update(canvas);
    }

    fn get_extent(&mut self) -> Bounds<M> {
        self.artist.get_extent()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &Affine2d,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let style = self.style.push(style);

        self.artist.draw(renderer, to_canvas, clip, &style);
    }
}

impl<M: Coord> PlotArtist<M> for PlotOptArtist<M> {
    type Opt = PlotOpt;

    fn config(
        &mut self, 
        cfg: &super::ConfigArc, 
        id: PlotId,
    ) -> Self::Opt {
        self.style = PathStyle::from_config(cfg, "artist");

        unsafe { PlotOpt::new(id) }
    }
}

pub struct PlotId {
    layout: LayoutArc,
    artist_id: ArtistId,
}

impl PlotId {
    pub(crate) fn new(
        layout: LayoutArc, 
        artist_id: ArtistId
    ) -> Self {
        Self {
            layout,
            artist_id
        }
    }

    pub fn layout(&self) -> &LayoutArc {
        &self.layout
    }

    pub fn id(&self) -> &ArtistId {
        &self.artist_id
    }
}

pub trait PlotArtist<M: Coord> : Artist<M> {
    type Opt;
    
    fn config(
        &mut self, 
        cfg: &ConfigArc, 
        id: PlotId,
    ) -> Self::Opt;
}

pub trait SimpleArtist<M: Coord> : Artist<M> {
}

//pub trait PathStyleArtist : Artist<Data> {
//    fn style_mut(&mut self) -> &mut PathStyle;
//}
