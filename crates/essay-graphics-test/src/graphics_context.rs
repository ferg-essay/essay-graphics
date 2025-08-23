use essay_graphics_api::renderer::{FontSetMetrics, GraphicsContext};

use crate::TestFontSet;

pub struct TestGraphicsContext {

}

impl TestGraphicsContext {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl GraphicsContext for TestGraphicsContext {
    fn default_font_set(&self) -> Box<dyn FontSetMetrics> {
        Box::new(TestFontSet::new())
    }
}