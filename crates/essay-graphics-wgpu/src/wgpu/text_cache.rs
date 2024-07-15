use std::{collections::HashMap, fs, ops::Index};

use swash::{FontRef, scale::{ScaleContext, Source, Render}, CacheKey, Charmap, zeno::Format};

use super::text_texture::TextTexture;

pub struct TextCache {
    context: ScaleContext,
    font_map: HashMap<String, FontId>,
    fonts: Vec<Font>,
    glyph_map: HashMap<GlyphId, GlyphRect>,

    store: TextStore,

    is_modified: bool,
}

impl TextCache {
    pub fn new(width: u32, height: u32) -> Self {
        assert!(width % 256 == 0);

        Self {
            context: ScaleContext::new(),
            font_map: HashMap::default(),
            fonts: Vec::new(),
            glyph_map: HashMap::default(),

            store: TextStore::new(width as usize, height as usize),

            is_modified: true,
        }
    }

    pub fn font_id(&mut self, name: &str) -> FontId {
        let len = self.font_map.len();

        let id = self.font_map.entry(name.to_string())
            .or_insert_with(|| {
                FontId(len)
            }
        );

        if self.fonts.len() <= id.0 {
            self.fonts.push(load_font(name));
        }

        *id
    }

    //#[inline]
    //pub fn font(&mut self, id: FontId) -> &Font {
    //    &self.fonts[id.0]
    //}

    pub fn _font(&mut self, name: &str) -> &Font {
        let len = self.font_map.len();

        let id = self.font_map.entry(name.to_string())
            .or_insert_with(|| {
                FontId(len)
            }
        );

        if self.fonts.len() <= id.0 {
            self.fonts.push(load_font(name));
        }

        &self.fonts[id.0]
    }

    pub fn glyph(&mut self, font_id: FontId, size: u16, glyph: char) -> TextRect {
        // let font_id = self.font(font_name).id;
        let glyph_id = GlyphId::new(font_id, size, glyph);

        if let Some(rect) = self.find_glyph(&glyph_id) {
            return TextRect::new(&rect, self.store.width, self.store.height)
        }

        let rect = self.add_glyph(font_id, size as f32, glyph);

        self.glyph_map.insert(glyph_id, rect.clone()); 

        TextRect::new(&rect, self.store.width, self.store.height)
    }

    fn find_glyph(&mut self, glyph_id: &GlyphId) -> Option<GlyphRect> {
        match self.glyph_map.get(glyph_id) {
            Some(rect) => Some(rect.clone()),
            None => None,
        }
    }

    fn add_glyph(&mut self, font_id: FontId, size: f32, ch: char) -> GlyphRect {
        let font = &self.fonts[font_id.0];

        let glyph = font.charmap().map(ch);

        let mut scaler = self.context
            .builder(font.as_ref())
            .size(size)
            .build();

        let image = Render::new(&[
            Source::Outline,
        ]).format(Format::Alpha)
        .render(&mut scaler, glyph)
        .unwrap();

        let placement = image.placement;

        let p_w = placement.width as usize;
        let p_h = placement.height as usize;

        let (x, y) = self.store.add_glyph(
            p_w, 
            p_h,
            &image.data
        );

        let metrics = font.as_ref().metrics(&[]).scale(size);
        let descent = (metrics.descent) as i32;

        self.is_modified = true;

        GlyphRect {
            x,
            y,
            w: p_w,
            h: p_h,
            left: placement.left as i32,
            top: placement.top as i32 + descent,
        }
    }

    pub(crate) fn flush(
        &mut self, 
        queue: &wgpu::Queue, 
        texture: &TextTexture,
    ) {
        if self.is_modified {
            self.is_modified = false;

            texture.write_data(queue, &self.store.data);
        }
    }
}

impl Index<FontId> for TextCache {
    type Output = Font;

    #[inline]
    fn index(&self, id: FontId) -> &Self::Output {
        &self.fonts[id.0]
    }
}

fn load_font(path: &str) -> Font {
    if let Ok(font_data) = fs::read(path) {
        Font::from_data(font_data.as_slice()).unwrap()
    } else {
        let font_data = include_bytes!(
            "../../assets/fonts/DejaVuSans.ttf"
        );

        Font::from_data(font_data).unwrap()
    }
}

pub struct Font {
    data: Vec<u8>,
    offset: u32,
    key: CacheKey,
}

impl Font {
    fn from_data(data: &[u8]) -> Option<Self> {
        let index = 0;

        let font = FontRef::from_index(data, index)?;
        let (offset, key) = (font.offset, font.key);

        Some(Self { data: data.to_vec(), offset, key })
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
                if width <= self.cursors[i].width - self.cursors[i].x {
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
pub struct FontId(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct GlyphId {
    font: FontId,
    size: u16,
    glyph: char,
}

impl GlyphId {
    fn new(font: FontId, size: u16, glyph: char) -> Self {
        Self {
            font,
            size,
            glyph,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextRect {
    pub tx_min: f32,
    pub ty_min: f32,

    pub tx_max: f32,
    pub ty_max: f32,

    pub w: f32,
    pub h: f32,

    pub dx: f32,
    pub dy: f32,
}


impl TextRect {
    fn new(glyph: &GlyphRect, width: usize, height: usize) -> Self {
        Self {
            tx_min: glyph.x as f32 / width as f32,
            tx_max: (glyph.x + glyph.w) as f32 / width as f32,

            ty_max: glyph.y as f32 / height as f32,
            ty_min: (glyph.y + glyph.h) as f32 / height as f32,

            w: glyph.w as f32,
            h: glyph.h as f32,

            dx: glyph.left as f32,
            dy: glyph.top as f32 - glyph.h as f32,
        }
    }

    pub(crate) fn is_none(&self) -> bool {
        self.w == 0.
    }
}

#[derive(Clone, Debug)]
struct GlyphRect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,

    left: i32,
    top: i32,
}
