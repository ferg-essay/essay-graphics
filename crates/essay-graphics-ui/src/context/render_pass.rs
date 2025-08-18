use essay_graphics_api::{output::Output, renderer::Canvas, Bounds};

use crate::{context::widget::WidgetRects, page::Page, util::IdMap};


#[derive(Default)]
pub struct RenderPass {
    pub widgets: WidgetRects,

    // view_size: ViewSizeCache,
    pub alloc_map: IdMap<CacheAlloc>,

    pub output: Option<Output>,
}

impl RenderPass {
    pub fn widgets(&self) -> &WidgetRects {
        &self.widgets
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CacheAlloc {
    pub view: Bounds<Page>,
    pub fixed: Bounds<Canvas>,
}

impl CacheAlloc {
    pub(crate) fn is_changed(&self, alloc_cache: &Option<CacheAlloc>) -> bool {
        false
    }
}
