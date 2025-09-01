use std::sync::Arc;

use essay_graphics_api::{output::Output, renderer::Canvas, Bounds};

use crate::{page::Page, style::UiStyle, ui::{widget::WidgetRects, AllocCache, Context}, util::IdMap};

pub struct UiRender {
    pub pass: RenderPass,
    pub last_pass: RenderPass,

    pub context: Context,
    pub theme: Arc<UiStyle>,
}

#[derive(Default)]
pub struct RenderPass {
    pub widgets: WidgetRects,

    // view_size: ViewSizeCache,
    pub alloc_map: IdMap<AllocCache>,

    pub output: Option<Output>,
}

impl RenderPass {
    pub fn widgets(&self) -> &WidgetRects {
        &self.widgets
    }
}
