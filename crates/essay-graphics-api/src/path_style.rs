use core::fmt;

use crate::{Affine2d, CapStyle, Color, Hatch, JoinStyle, LineStyle, PathOpt, TextureId};

#[derive(Clone)]
pub struct PathStyle {
    color: Option<Color>,
    face_color: Option<Color>,
    edge_color: Option<Color>,

    line_width: Option<f32>,
    join_style: Option<JoinStyle>,
    cap_style: Option<CapStyle>,

    line_style: Option<LineStyle>,
    alpha: Option<f32>,
    texture: Option<TextureId>,
    hatch: Option<Hatch>,

    gap_color: Option<Color>,
}

impl PathStyle {
    pub fn new() -> PathStyle {
        PathStyle::default()
    }

    pub fn color(&mut self, color: impl Into<Color>) -> &mut Self {
        // TODO: is color a default or an assignment?
        self.color = Some(color.into());

        self
    }

    pub fn face_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.face_color = Some(color.into());

        self
    }

    pub fn edge_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.edge_color = Some(color.into());

        self
    }

    pub fn line_style(&mut self, line_style: impl Into<LineStyle>) -> &mut Self {
        self.line_style = Some(line_style.into());

        self
    }

    pub fn get_line_width(&mut self) -> Option<f32> {
        self.line_width.clone()
    }

    pub fn line_width(&mut self, linewidth: f32) -> &mut Self {
        // assert!(linewidth > 0.);

        self.line_width = Some(linewidth);

        self
    }

    pub fn join_style(&mut self, joinstyle: impl Into<JoinStyle>) -> &mut Self {
        self.join_style = Some(joinstyle.into());

        self
    }

    pub fn cap_style(&mut self, capstyle: impl Into<CapStyle>) -> &mut Self {
        self.cap_style = Some(capstyle.into());

        self
    }

    pub fn alpha(&mut self, alpha: f32) -> &mut Self {
        self.alpha = Some(alpha);

        self
    }

    pub fn texture(&mut self, texture: TextureId) -> &mut Self {
        self.texture = Some(texture);

        self
    }

    pub fn hatch(&mut self, hatch: impl Into<Hatch>) -> &mut Self {
        self.hatch = Some(hatch.into());

        self
    }
}

impl fmt::Debug for PathStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt = f.debug_struct("PathStyle");

        if let Some(color) = &self.color {
            fmt.field("color", color);
        }

        if let Some(face_color) = &self.face_color {
            fmt.field("face_color", face_color);
        }

        if let Some(edge_color) = &self.edge_color {
            fmt.field("edge_color", edge_color);
        }
        
        if let Some(line_width) = &self.line_width {
            fmt.field("line_width", line_width);
        }
        
        if let Some(join_style) = &self.join_style {
            fmt.field("join_style", join_style);
        }

        if let Some(cap_style) = &self.cap_style {
            fmt.field("cap_style", cap_style);
        }
        
        if let Some(line_style) = &self.line_style {
            fmt.field("line_style", line_style);
        }
        
        if let Some(alpha) = &self.alpha {
            fmt.field("alpha", alpha);
        }
        
        if let Some(gap_color) = &self.gap_color {
            fmt.field("gap_color", gap_color);
        }
        
        fmt.finish()
    }
}

impl PathOpt for PathStyle {
    #[inline]
    fn get_face_color(&self) -> Option<Color> {
        self.face_color.or(self.color)
    }

    #[inline]
    fn get_edge_color(&self) -> Option<Color> {
        self.edge_color.or(self.color)
    }

    #[inline]
    fn get_line_width(&self) -> Option<f32> {
        self.line_width
    }

    #[inline]
    fn get_join_style(&self) -> Option<JoinStyle> {
        self.join_style
    }

    #[inline]
    fn get_cap_style(&self) -> Option<CapStyle> {
        self.cap_style
    }

    #[inline]
    fn get_line_style(&self) -> Option<LineStyle> {
        self.line_style.as_ref().map(|style| style.clone())
    }

    #[inline]
    fn get_alpha(&self) -> Option<f32> {
        self.alpha
    }

    #[inline]
    fn get_texture(&self) -> Option<TextureId> {
        self.texture
    }

    #[inline]
    fn get_hatch(&self) -> Option<Hatch> {
        self.hatch
    }
}

impl Default for PathStyle {
    fn default() -> Self {
        Self { 
            color: None,
            face_color: None,
            edge_color: None,
            line_width: None,
            join_style: None,
            cap_style: None,
            line_style: None,
            gap_color: None,
            alpha: None,
            texture: None,
            hatch: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MarkerStyle {
    pub color: Color,
    pub affine: Affine2d,
}

impl From<Color> for MarkerStyle {
    fn from(color: Color) -> Self {
        Self {
            color,
            affine: Affine2d::eye(),
        }
    }
}

impl From<(Color, Affine2d)> for MarkerStyle {
    fn from((color, affine): (Color, Affine2d)) -> Self {
        Self {
            color,
            affine,
        }
    }
}

impl From<(Color, &Affine2d)> for MarkerStyle {
    fn from((color, affine): (Color, &Affine2d)) -> Self {
        Self {
            color,
            affine: affine.clone(),
        }
    }
}

