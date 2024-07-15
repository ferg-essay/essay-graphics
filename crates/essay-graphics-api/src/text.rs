
#[derive(Clone)]
pub struct TextStyle {
    font: Option<FontTypeId>,
    size: Option<f32>,

    vert_align: Option<VertAlign>,
    horiz_align: Option<HorizAlign>,
}

impl TextStyle {
    pub const SIZE_DEFAULT : f32 = 10.;
    pub const HALIGN_DEFAULT : HorizAlign = HorizAlign::Center;
    pub const VALIGN_DEFAULT : VertAlign = VertAlign::BaselineBottom;

    pub fn new() -> Self {
        Self {
            font: None,
            size: None,

            vert_align: None,
            horiz_align: None,
        }
    }

    #[inline]
    pub fn get_font(&self) -> &Option<FontTypeId> {
        &self.font
    }

    pub fn font(&mut self, id: FontTypeId) -> &mut Self {
        self.font = Some(id);

        self
    }

    #[inline]
    pub fn get_size(&self) -> &Option<f32> {
        &self.size
    }

    pub fn size(&mut self, size: f32) -> &mut Self {
        self.size = Some(size);

        self
    }

    #[inline]
    pub fn get_height_align(&self) -> &Option<VertAlign> {
        &self.vert_align
    }

    pub fn valign(&mut self, align: VertAlign) {
        self.vert_align = Some(align);
    }

    #[inline]
    pub fn get_width_align(&self) -> &Option<HorizAlign> {
        &self.horiz_align
    }

    pub fn halign(&mut self, align: HorizAlign) {
        self.horiz_align = Some(align);
    }

}

#[derive(Clone, Copy, Debug)]
pub enum VertAlign {
    Bottom,
    BaselineBottom,
    Center,
    Top,
}

#[derive(Clone, Copy, Debug)]
pub enum HorizAlign {
    Left,
    Center,
    Right,
}

pub struct FontStyle {
    family: Option<String>,
}

impl FontStyle {
    pub fn new() -> Self {
        FontStyle {
            family: None,
        }
    }

    pub fn family(&mut self, family: &str) -> &mut Self {
        self.family = Some(family.to_string());

        self
    }

    pub fn get_family(&self) -> &Option<String> {
        &self.family
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontTypeId(pub usize);

impl FontTypeId {
    #[inline]
    pub fn i(&self) -> usize {
        self.0
    }
}

#[derive(Clone)]
pub struct FontFamily {
    path: String,
}

impl FontFamily {
    pub fn new(family: &str) -> Self {
        Self {
            path: family.to_string(),
        }
    }

    #[inline]
    pub fn get_path(&self) -> &str {
        &self.path
    }
}

impl Into<FontFamily> for &str {
    fn into(self) -> FontFamily {
        FontFamily::new(self)
    }
}
