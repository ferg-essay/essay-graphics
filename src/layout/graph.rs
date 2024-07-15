use core::fmt;

use crate::{
    artist::{Artist, IntoArtist, PlotArtist},
    frame::Data, 
    graph::{PlotOpt, PlotOptArtist},
};

use super::{FrameId, GraphId, LayoutArc};

#[derive(Clone)]
pub struct Graph {
    id: GraphId,
    frame_id: FrameId,

    layout: LayoutArc,
}

impl Graph {
    pub(crate) fn new(id: GraphId, frame_id: FrameId, layout: LayoutArc) -> Self {
        let mut graph = Self {
            id,
            frame_id, 
            layout,
        };

        graph.default_properties();

        graph
    }

    #[inline]
    pub fn id(&self) -> GraphId {
        self.id
    }

    #[inline]
    pub fn frame_id(&self) -> FrameId {
        self.frame_id
    }

    fn default_properties(&mut self) {
        //self.title.font().size(12.);
    }

    // TODO: should there be a plain add_artist that doesn't wrap PlotStyle?

    pub fn add_simple_artist<'a, A>(
        &mut self, 
        artist: A,
    ) -> PlotOpt
    where
        A: Artist<Data> + 'static
    {
        self.artist(PlotOptArtist::new(artist))
    }

    /*
    pub fn artist<'a, A>(
        &mut self, 
        artist: A,
    ) -> A::Opt 
    where
        A: PlotArtist<Data> + 'static
    */

    pub fn artist<'a, A>(
        &mut self, 
        _artist: A,
    ) -> <A::Artist as PlotArtist<Data>>::Opt 
    where
        A: IntoArtist<Data> + 'static
    {
        todo!();
        /*
        let artist = artist.into_artist();

        let id = self.layout.write(|l|
            l.frame_mut(self.frame_id)
            .data_mut()
            .add_artist(artist)
        );

        let plot_id = PlotId::new(
            self.layout.clone(),
            id
        );

        self.layout.write(move |layout| {
            let config = layout.config().clone();

            layout
                .frame_mut(id.frame())
                .data_mut()
                .artist_mut::<A::Artist>(id)
                .config(&config, plot_id)
        })
        */
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pos = self.layout.read(|l| l.frame(self.frame_id).pos().clone());

        write!(f, "Graph[{}]({},{}; {}x{})",
            self.frame_id.index(),
            pos.xmin(),
            pos.ymin(),
            pos.width(),
            pos.height(),
        )
    }
}
