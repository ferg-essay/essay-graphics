use essay_graphics_api::renderer::{FontSetMetrics, GlyphSize};

pub struct TestFontSet {
}

impl TestFontSet {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl FontSetMetrics for TestFontSet {
    fn glyph_size(&mut self, size: f32, _glyph: char) -> GlyphSize {
        GlyphSize {
            width: size,
            ascent: size,
            descent: 0.,
            lsb: 0.,
        }
    }
}