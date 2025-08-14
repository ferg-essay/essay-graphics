use essay_graphics_api::renderer::{FontSetMetrics, GlyphSize, GraphicsContext};

use std::{collections::{HashMap}, fs};

use swash::{FontRef, CacheKey, Charmap};

pub struct WgpuGraphicsContext {

}

impl WgpuGraphicsContext {
    pub(crate) fn new() -> Self {
        Self {
        }
    }
}

impl GraphicsContext for WgpuGraphicsContext {
    fn default_font_set(&self) -> Box<dyn FontSetMetrics> {
        Box::new(FontCache::new(vec![
            Font::from_data(include_bytes!(
                "../../assets/fonts/deja-vu/DejaVuSans.ttf"
            )),

            Font::from_data(include_bytes!(
                "../../assets/fonts/noto-emoji/NotoEmoji-Regular.ttf"
            ))
        ]))
    }
}

pub struct FontCache {
    fonts: Vec<Font>,
    glyph_map: HashMap<GlyphId, GlyphSize>,
}

impl FontCache {
    pub fn new(fonts: Vec<Font>) -> Self {
        Self {
            fonts: fonts,
            glyph_map: HashMap::default(),
        }
    }

    pub fn glyph(&mut self, size: u16, glyph: char) -> GlyphSize {
        // let font_id = self.font(font_name).id;
        let glyph_id = GlyphId::new(size, glyph);

        if let Some(glyph) = self.glyph_map.get(&glyph_id) {
            return glyph.clone();
        }

        let rect = self.add_glyph(size as f32, glyph);

        self.glyph_map.insert(glyph_id, rect.clone());

        rect
    }

    fn add_glyph(&self, size: f32, ch: char) -> GlyphSize {
        for font in &self.fonts {
            let glyph = font.charmap().map(ch);

            if glyph != 0 {
                let glyphs = font.as_ref().glyph_metrics(&[]).scale(size);

                return GlyphSize {
                    width: glyphs.advance_width(glyph),
                    height: glyphs.advance_height(glyph),
                    // lsb: glyphs.lsb(glyph),
                }
            }
        }

        panic!("Unknown glyph for {:?}", ch);
    }
}

impl FontSetMetrics for FontCache {
    fn glyph_size(&mut self, size: f32, glyph: char) -> GlyphSize {
        self.glyph((size + 0.5) as u16, glyph)
    }
}

pub struct Font {
    data: Vec<u8>,
    offset: u32,
    key: CacheKey,
}

impl Font {
    pub fn load(path: &str) -> Option<Font> {
        fs::read(path).map_or(None,|font_data| {
            Some(Self::from_data(font_data.as_slice()))
        })
    }

    pub fn from_data(data: &[u8]) -> Self {
        let index = 0;

        let font = FontRef::from_index(data, index).unwrap();
        let (offset, key) = (font.offset, font.key);

        Self { data: data.to_vec(), offset, key }
    }

    fn charmap(&self) -> Charmap {
        self.as_ref().charmap()
    }

    fn as_ref(&self) -> FontRef {
        FontRef {
            data: &self.data,
            offset: self.offset,
            key: self.key
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct GlyphId {
    size: u16,
    glyph: char,
}

impl GlyphId {
    fn new(size: u16, glyph: char) -> Self {
        Self {
            size,
            glyph,
        }
    }
}
