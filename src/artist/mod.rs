mod norm;
mod collection;
mod colorbar;
mod colormaps;
mod colormap;
mod container;
mod contour;
mod cycle;
mod color;
mod grid_color;
mod histogram;
mod image;
mod style;
mod triplot;
pub mod paths;
mod text;
mod stem;
mod tricontour;
mod markers;
pub mod patch;
mod artist;
mod lines;

pub use artist::{
    Artist, IntoArtist, PlotArtist, PlotId,
};

pub use collection::PathCollection;

pub use container::{
    Container, ContainerOpt
};

pub use color::ColorCycle;

pub use colorbar::Colorbar;

pub use colormap::ColorMap;

pub use colormaps::ColorMaps;

pub use grid_color::{
    GridColor, GridColorOpt, Shading,
};

pub use contour::Contour;

pub use norm::{
    Norm, Norms,
};

pub use histogram::{
    Histogram, HistogramOpt, 
};

pub use image::Image;

pub use tricontour::TriContour;

pub use cycle::StyleCycle;

pub use triplot::TriPlot;

pub use lines::{
    Lines2d, LinesOpt, DrawStyle,
};

pub use markers::{
    Markers, MarkerStyle, IntoMarker,
};

pub use patch::{
    Patch,
    PatchTrait,
    arrow, Arrow,
};

pub use stem::{
    Stem, StemOpt,
};

pub use style::PathStyle;

pub use text::{
    Text, // TextStyle,
};
