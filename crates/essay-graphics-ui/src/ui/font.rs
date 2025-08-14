use std::{collections::{HashMap}, fs};

use swash::{FontRef, scale::{ScaleContext}, CacheKey, Charmap};

pub struct FontCache {
    context: ScaleContext,
    fonts: Vec<Font>,
    glyph_map: HashMap<GlyphId, GlyphRect>,
}

impl FontCache {
    pub fn new(fonts: Vec<Font>) -> Self {
        Self {
            context: ScaleContext::new(),
            fonts: fonts,
            glyph_map: HashMap::default(),
        }
    }

    pub fn glyph(&mut self, size: u16, glyph: char) -> GlyphRect {
        // let font_id = self.font(font_name).id;
        let glyph_id = GlyphId::new(size, glyph);

        if let Some(glyph) = self.glyph_map.get(&glyph_id) {
            return glyph.clone();
        }

        let rect = self.add_glyph(size as f32, glyph);

        self.glyph_map.insert(glyph_id, rect.clone());

        rect
    }

    fn add_glyph(&self, size: f32, ch: char) -> GlyphRect {
        for font in &self.fonts {
            let glyph = font.charmap().map(ch);

            if glyph != 0 {
                let glyphs = font.as_ref().glyph_metrics(&[]);
                glyphs.scale(size);

                return GlyphRect {
                    advance_width: glyphs.advance_width(glyph),
                    advance_height: glyphs.advance_height(glyph),
                    lsb: glyphs.lsb(glyph),
                };
            }
        }

        panic!("Unknown glyph for {:?}", ch);
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

struct TextStore {
    width: usize,
    height: usize,

    data: Vec<u8>,

    tail: usize,
    cursors: Vec<TextCursor>,
}

impl TextStore {
    fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && width % 256 == 0);
        assert!(height > 0);

        let mut data = Vec::new();
        data.resize(width * height, 0);

        Self {
            width,
            height,
            data,
            tail: 0,
            cursors: Vec::new(),
        }
    }

    fn add_glyph(&mut self, width: usize, height: usize, data: &Vec<u8>) -> (usize, usize) {
        let cursor = self.cursor(width, height);

        let (x, y) = (cursor.x(), cursor.y());
        let c_w = cursor.width;

        // let offset = 0;//cursor.offset;

        for j in 0..height {
            for i in 0..width {
                self.data[x + i + (j + y) * c_w] = data[i + j * width];
            }
        }

        (x, y)
    }

    fn cursor(&mut self, width: usize, height: usize) -> TextCursor {
        let height = height.max(1);

        let height_chunk = height + 31;
        let height_chunk = height_chunk - height_chunk % 32;

        let len = self.cursors.len();
        for i in (0..len).rev() {
            if self.cursors[i].height == height_chunk {
                if width <= self.cursors[i].width.saturating_sub(self.cursors[i].x) {
                    return self.cursors[i].add_x(width);
                }

                self.cursors.remove(i);
            }
        }

        return self.add_cursor(height_chunk).add_x(width);
    }

    fn add_cursor(&mut self, height: usize) -> &mut TextCursor {
        assert!(height > 0 && height % 32 == 0);

        let len = self.cursors.len();
        self.cursors.push(TextCursor::new(self.width, height, self.tail));

        self.tail += self.width * height;

        if self.data.len() < self.tail {
            self.height = 2 * self.height;

            self.data.resize(self.width * self.height, 0);
        }

        let cursor = &mut self.cursors[len];

        let n_x = cursor.x + 1;
        if n_x % 4 > 0 {
            cursor.x += 4 - cursor.x % 4;
        }

        cursor
    }
}

#[derive(Clone)]
struct TextCursor {
    width: usize,
    height: usize,

    offset: usize,

    x: usize,
}

impl TextCursor {
    fn new(width: usize, height: usize, offset: usize) -> Self {
        TextCursor {
            width,
            height,
            offset,
            x: 0,
        }
    }

    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.offset / self.width
    }

    fn add_x(&mut self, width: usize) -> TextCursor {
        let cursor = self.clone();

        let mut n_x = self.x + width + 1;
        if n_x % 4 > 0 {
            n_x += 4 - n_x % 4;
        }

        self.x = n_x;

        cursor
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

#[derive(Clone, Debug)]
pub struct GlyphRect {
    advance_width: f32,
    advance_height: f32,
    lsb: f32,
}
