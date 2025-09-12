use std::sync::Arc;

use essay_graphics_api::{input::Input, output::Output};

use crate::{style::UiTheme, ui::{alloc::AllocPair, context::UiState, widget::WidgetRects, AllocSize, Context, PainterLayers}, util::IdMap};

pub(crate) struct UiRender {
    pub state: UiState,

    pub context: Context,
    pub theme: Arc<UiTheme>,
    
    pub layers: PainterLayers,

    pub input: Input,
    pub output: Option<Output>,
}

#[derive(Default)]
pub struct RenderPass {
    pub widgets: WidgetRects,

    // view_size: ViewSizeCache,
    pub alloc_map: IdMap<AllocPair>,

    // pub output: Option<Output>,
}

impl RenderPass {
    pub fn widgets(&self) -> &WidgetRects {
        &self.widgets
    }
}
