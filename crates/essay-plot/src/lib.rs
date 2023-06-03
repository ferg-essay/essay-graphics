pub mod artist;
pub mod axes;
pub mod figure;
pub mod plot;
pub mod backend;

pub mod prelude {
    //pub use crate::plotly::{Plot, PlotOpt};
    //pub use crate::criterion::{Plot, PlotOpt};
    //pub use crate::egui::{Plot, PlotOpt};
    pub use crate::figure::{Figure};
    pub use crate::plot::{Plot, PlotOpt};
}