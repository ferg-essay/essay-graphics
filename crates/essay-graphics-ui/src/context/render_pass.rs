use std::sync::Arc;

use essay_graphics_api::{output::Output, renderer::Canvas, Bounds};

use crate::{context::{widget::WidgetRects, Context}, page::Page, style::UiStyle, util::IdMap};

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

#[derive(Clone, Debug, PartialEq)]
pub struct AllocCache {
    pub view: Bounds<Page>,
    pub fixed: Bounds<Canvas>,
}

impl AllocCache {
    pub(crate) fn is_changed(&self, alloc_cache: &Option<AllocCache>) -> bool {
        false
    }
}
