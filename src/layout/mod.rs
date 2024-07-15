mod figure;
pub mod graph;
mod layout;
mod frame;

pub use graph::Graph;

pub use figure::{
    Figure, FigureInner, GraphId,
};

use frame::Frame;
    // pub use style::PlotOpt;

pub use layout::{
    Layout, LayoutArc, FrameId,
};
