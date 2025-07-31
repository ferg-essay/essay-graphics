pub mod api {
    pub use essay_graphics_api::*;
}

pub mod wgpu {
    pub use essay_graphics_wgpu::*;
}

pub mod winit {
    pub use essay_graphics_winit::*;
}

pub mod ui {
    pub use essay_graphics_ui::ui::*;
}

pub mod layout {
    pub use essay_graphics_ui::page::*;
}

pub mod prelude {
    // pub use crate::graph::Figure;
    // pub use crate::plot::{Plot, PlotOpt};

    pub use crate::api::*;
}