use essay_graphics_api::{
    Bounds, Point, Canvas,
    PathOpt,
    driver::Renderer, 
    TextStyle, Clip, FontFamily, FontStyle,
};

use crate::{frame::Data, graph::ConfigArc, data_artist_option_struct, path_style_options};

use super::{Artist, PathStyle, PlotArtist, PlotId, artist::ToCanvas};

pub struct TextCanvas {
    pos: Bounds<Canvas>,
    extent: Bounds<Canvas>,

    text: Option<String>,

    path_style: PathStyle,
    text_style: TextStyle,

    angle: f32,
}

impl TextCanvas {
    pub const DESC : f32 = 0.3;

    pub fn new() -> Self {
        Self {
            pos: Bounds::none(),
            extent: Bounds::zero(),
            text: None,

            path_style: PathStyle::new(),
            text_style: TextStyle::new(),

            angle: 0.
        }
    }

    pub(crate) fn set_pos(&mut self, pos: impl Into<Bounds<Canvas>>) {
        self.pos = pos.into();
    }

    pub fn label(&mut self, text: &str) -> &mut Self {
        if text.len() > 0 {
            self.text = Some(text.to_string());
        } else {
            self.text = None;
        }

        self
    }

    pub fn height(&self) -> f32 {
        self.extent.height()
    }

    pub fn text_style(&self) -> &TextStyle {
        &self.text_style
    }

    pub fn text_style_mut(&mut self) -> &mut TextStyle {
        &mut self.text_style
    }

    pub fn path_style_mut(&mut self) -> &mut PathStyle {
        &mut self.path_style
    }

    pub fn angle(&mut self, angle: f32) -> &mut Self {
        self.angle = angle;

        self
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }
}

impl Artist<Canvas> for TextCanvas {
    fn get_extent(&mut self) -> Bounds<Canvas> {
        self.extent.clone()
    }

    fn update(&mut self, canvas: &Canvas) {
        self.extent = match &self.text {
            None => Bounds::zero(),
            Some(text) => {
                let size = match self.text_style.get_size() {
                    Some(size) => *size,
                    None => TextStyle::SIZE_DEFAULT,
                };

                let width = text.len() as f32 * size as f32; //  * 0.5;

                Bounds::extent(
                    canvas.to_px(width),
                    canvas.to_px(size)
                )
            }
        }
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        _to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        if let Some(text) = &self.text {
            let style = self.path_style.push(style);

            if ! self.pos.is_none() {
                //let desc = Self::DESC * self.extent.height();

                renderer.draw_text(
                    Point(self.pos.xmid(), self.pos.ymin()),
                    text,
                    self.get_angle(),
                    &style,
                    &self.text_style,
                    clip
                ).unwrap();
            }
        }
    }
}

pub struct Text {
    pos: Point,
    coords: TextCoords,

    text: String,

    path_style: PathStyle,
    text_style: TextStyle,

    family: Option<FontFamily>,

    angle: f32,
}

impl Text {
    pub const DESC : f32 = 0.3;

    pub fn new(pos: impl Into<Point>, text: impl AsRef<str>) -> Self {
        Self {
            pos: pos.into(),
            coords: TextCoords::Data,
            text: text.as_ref().to_string(),

            path_style: PathStyle::new(),
            text_style: TextStyle::new(),

            family: None,

            angle: 0.
        }
    }

    pub fn text(&mut self, text: impl AsRef<str>) -> &mut Self {
        self.text = text.as_ref().to_string();

        self
    }

    pub fn height(&self) -> f32 {
        //self.extent.height()
        0.
    }

    pub fn text_style(&self) -> &TextStyle {
        &self.text_style
    }

    pub fn text_style_mut(&mut self) -> &mut TextStyle {
        &mut self.text_style
    }

    pub fn path_style_mut(&mut self) -> &mut PathStyle {
        &mut self.path_style
    }

    pub fn angle(&mut self, angle: f32) -> &mut Self {
        self.angle = angle;

        self
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }
}

impl Artist<Data> for Text {
    fn update(&mut self, _canvas: &Canvas) {
    }
    
    fn get_extent(&mut self) -> Bounds<Data> {
        Bounds::none()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let pos = self.coords.to_canvas(self.pos, &to_canvas);
        let style = self.path_style.push(style);

        if self.text.len() > 0 {
            if let Some(family) = &self.family {
                let mut font_style = FontStyle::new();
                font_style.family(family.get_path());

                let font_id = renderer.font(&font_style).unwrap();
                println!("FontId {:?} {:?}", font_id, family.get_path());
                self.text_style.font(font_id);
            }

            renderer.draw_text(
                pos,
                &self.text,
                0.,
                &style,
                &self.text_style,
                clip
            ).unwrap();
        }
    }
}

impl PlotArtist<Data> for Text {
    type Opt = TextOpt;

    fn config(&mut self, _cfg: &ConfigArc, id: PlotId) -> Self::Opt {
        // self.style = PathStyle::from_config(cfg, "text");

        unsafe { TextOpt::new(id) }
    }

    fn get_legend(&self) -> Option<crate::frame::LegendHandler> {
        None
    }
}

data_artist_option_struct!(TextOpt, Text);

impl TextOpt {
    path_style_options!(path_style);

    pub fn text(&mut self, label: impl AsRef<str>) -> &mut Self {
        self.write(|artist| {
            artist.text = label.as_ref().to_string();
        });

        self
    }

    pub fn pos(&mut self, pos: impl Into<Point>) -> &mut Self {
        self.write(|artist| {
            artist.pos = pos.into();
        });

        self
    }

    pub fn coord(&mut self, coord: impl Into<TextCoords>) -> &mut Self {
        self.write(|artist| {
            artist.coords = coord.into();
        });

        self
    }

    pub fn family(&mut self, family: impl Into<FontFamily>) -> &mut Self {
        self.write(|artist| {
            artist.family = Some(family.into());
        });

        self
    }

    pub fn size(&mut self, size: f32) -> &mut Self {
        self.write(|artist| {
            artist.text_style.size(size);
        });

        self
    }
}

pub enum TextCoords {
    Data,
    FrameFraction,
}

impl TextCoords {
    fn to_canvas(&self, pos: Point, to_canvas: &ToCanvas) -> Point {
        match self {
            TextCoords::Data => to_canvas.transform_point(pos),
            TextCoords::FrameFraction => {
                let bounds = to_canvas.pos();

                Point(
                    bounds.xmin() + pos.x() * bounds.width(),
                    bounds.ymin() + pos.y() * bounds.height(),
                )
            }
        }
    }
}