use std::ops::Deref;

use essay_graphics_api::{
    Coord, Bounds, Affine2d, Canvas, PathOpt,
    driver::Renderer, Clip,
};

use crate::{graph::ConfigArc, frame::{LayoutArc, ArtistId, LegendHandler}};

pub trait Artist<M: Coord> : Send {
    fn update(&mut self, canvas: &Canvas);

    fn get_extent(&mut self) -> Bounds<M>;
    
    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    );
}

pub trait PlotArtist<M: Coord> : Artist<M> {
    type Opt;
    
    fn config(
        &mut self, 
        cfg: &ConfigArc, 
        id: PlotId,
    ) -> Self::Opt;

    fn get_legend(&self) -> Option<LegendHandler>;
}

pub trait IntoArtist<M: Coord> {
    type Artist : PlotArtist<M>;

    fn into_artist(self) -> Self::Artist;
}

impl<M: Coord, A: PlotArtist<M>> IntoArtist<M> for A {
    type Artist = Self;

    fn into_artist(self) -> Self::Artist {
        self
    }
}

pub trait SimpleArtist<M: Coord> : Artist<M> {
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

pub struct ToCanvas {
    pos_frame: Bounds<Canvas>,
    to_canvas: Affine2d,
}

impl ToCanvas {
    pub fn new(pos_frame: Bounds<Canvas>, to_canvas: Affine2d) -> Self {
        Self {
            pos_frame,
            to_canvas
        }
    }

    pub fn pos(&self) -> &Bounds<Canvas> {
        &self.pos_frame
    }

    pub fn to_canvas(&self) -> &Affine2d {
        &self.to_canvas
    }
}

impl Deref for ToCanvas {
    type Target = Affine2d;

    fn deref(&self) -> &Self::Target {
        self.to_canvas()
    }
}
