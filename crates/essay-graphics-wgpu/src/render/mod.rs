mod render_canvas;
pub mod render;
pub mod context;
pub mod hatch;
mod lines;
mod triangulate3;

pub use render::PlotRenderer;
pub(crate) use render_canvas::RenderCanvas;