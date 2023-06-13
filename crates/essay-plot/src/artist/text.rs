use essay_plot_base::{
    Affine2d, 
    Bounds, Point, Canvas,
    Style, StyleOpt,
    driver::Renderer, 
    style::Chain, TextStyle,
};

use super::{ArtistTrait};

pub struct Text {
    pos: Bounds<Canvas>,
    extent: Bounds<Canvas>,

    text: Option<String>,

    style: Style,
    text_style: TextStyle,

    angle: f32,
}

impl Text {
    pub fn new() -> Self {
        Self {
            pos: Bounds::none(),
            extent: Bounds::zero(),
            text: None,

            style: Style::new(),
            text_style: TextStyle::new(),

            angle: 0.
        }
    }

    pub(crate) fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.pos = pos;
    }

    pub fn text(&mut self, text: &str) -> &mut Self {
        if text.len() > 0 {
            self.text = Some(text.to_string());
        } else {
            self.text = None;
        }

        self
    }

    pub fn height(&self) -> f32 {
        2. * 14.
    }

    pub fn font(&mut self) -> &mut TextStyle {
        &mut self.text_style
    }

    pub fn style(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn angle(&mut self, angle: f32) -> &mut Self {
        self.angle = angle;

        self
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }
}

impl ArtistTrait<Canvas> for Text {
    fn get_extent(&mut self) -> Bounds<Canvas> {
        self.extent.clone()
    }

    fn update_extent(&mut self, canvas: &Canvas) {
        self.extent = match &self.text {
            None => Bounds::zero(),
            Some(text) => {
                let height = match self.text_style.get_size() {
                    Some(size) => *size,
                    None => 12.,
                };

                let width = text.len() as f32 * height as f32 * 0.5;

                Bounds::extent(width * canvas.scale_factor(), height * canvas.scale_factor())
            }
        }
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        _to_canvas: &Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn StyleOpt,
    ) {
        if let Some(text) = &self.text {
            let style = Chain::new(style, &self.style);

            if ! self.pos.is_none() {
                renderer.draw_text(
                    Point(self.pos.xmid(), self.pos.ymid()),
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
