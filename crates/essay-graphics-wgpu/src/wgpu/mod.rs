mod bezier_mesh;
pub mod canvas;
mod form3d;
mod hatch;
mod main_loop_wgpu;
//mod main_loop_winit;
mod mesh2d;
mod mesh2d_color;
mod text;
mod text_texture;
mod text_cache;
mod texture_store;
pub mod hardcopy;

pub use canvas::PlotCanvas;

pub use main_loop_wgpu::WgpuMainLoop;

//pub use main_loop_winit::{
//    MainLoopHandle, run_event_loop,
//};

pub use hardcopy::WgpuHardcopy;

//pub(crate) use triangulate3::fill_shape;
