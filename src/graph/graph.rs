use core::fmt;

use crate::{artist::{
    Artist, PlotArtist, PlotId,
}, frame::{Data, LayoutArc, FrameId, FrameArtist, FrameTextOpt, AxisOpt, AspectMode}};

use super::{style::{PlotOptArtist, PlotOpt}};

pub struct Graph {
    id: FrameId,

    layout: LayoutArc,
}

impl Graph {
    pub(crate) fn new(id: FrameId, layout: LayoutArc) -> Self {
        let mut graph = Self {
            id, 
            layout,
        };

        graph.default_properties();

        graph
    }

    #[inline]
    pub fn id(&self) -> FrameId {
        self.id
    }

    fn text_opt(&self, artist: FrameArtist) -> FrameTextOpt {
        let layout = self.layout.clone();
        self.layout.borrow().frame(self.id).text_opt(layout, artist)
    }

    pub fn title(&mut self, label: &str) -> FrameTextOpt {
        let mut opt = self.text_opt(FrameArtist::Title);
        opt.label(label);
        opt
    }

    pub fn x(&mut self) -> AxisOpt {
        let layout = self.layout.clone();

        AxisOpt::new(layout, self.id(), FrameArtist::X)
    }

    pub fn y(&mut self) -> AxisOpt {
        let layout = self.layout.clone();

        AxisOpt::new(layout, self.id(), FrameArtist::Y)
    }

    pub fn x_label(&mut self, label: &str) -> FrameTextOpt {
        let mut opt = self.text_opt(FrameArtist::XLabel);
        opt.label(label);
        opt
    }

    pub fn y_label(&mut self, label: &str) -> FrameTextOpt {
        let mut opt = self.text_opt(FrameArtist::YLabel);
        opt.label(label);
        opt
    }

    pub fn aspect(&mut self, aspect: f32) -> &mut Self {
        self.layout.borrow_mut()
            .frame_mut(self.id)
            .data_mut()
            .aspect(aspect);

        self
    }

    pub fn aspect_mode(&mut self, mode: AspectMode) -> &mut Self {
        self.layout.borrow_mut()
            .frame_mut(self.id)
            .data_mut()
            .aspect_mode(mode);

        self
    }

    pub fn flip_y(&mut self, is_flip_y: bool) -> &mut Self {
        self.layout.borrow_mut()
            .frame_mut(self.id)
            .data_mut()
            .flip_y(is_flip_y);

        self
    }

    pub fn colorbar(&mut self) -> &mut Self {
        let _id = self.layout.borrow_mut()
            .frame_mut(self.id)
            .colorbar();

        self
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
        self.add_plot_artist(PlotOptArtist::new(artist))
    }

    pub fn add_plot_artist<'a, A>(
        &mut self, 
        artist: A,
    ) -> A::Opt 
    where
        A: PlotArtist<Data> + 'static
    {
        let id = self.layout.borrow_mut()
            .frame_mut(self.id)
            .data_mut()
            .add_artist(artist);

        let plot_id = PlotId::new(
            self.layout.clone(),
            id
        );

        self.layout.write(move |layout| {
            let config = layout.config().clone();

            layout
                .frame_mut(id.frame())
                .data_mut()
                .artist_mut::<A>(id)
                .config(&config, plot_id)
        })
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Graph[{}]({},{}; {}x{})",
            self.id.index(),
            self.layout.borrow().frame(self.id).pos().xmin(),
            self.layout.borrow().frame(self.id).pos().ymin(),
            self.layout.borrow().frame(self.id).pos().width(),
            self.layout.borrow().frame(self.id).pos().height(),
        )
    }
}
