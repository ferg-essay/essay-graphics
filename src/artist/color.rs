use core::fmt;
use std::ops::Index;

use essay_graphics_api::Color;


pub struct ColorCycle {
    colors: Vec<(String, Color)>,
}

impl ColorCycle {
    pub fn new(colors: &Vec<(&str, Color)>) -> Self {
        let mut vec = Vec::<(String, Color)>::new();

        for (name, color) in colors {
            vec.push((name.to_string(), *color));
        }

        Self {
            colors: vec
        }
    }
}

impl Index<usize> for ColorCycle {
    type Output = Color;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let len = self.colors.len();

        &self.colors[index % len].1
    }
}

impl ColorCycle {
    pub fn tableau() -> ColorCycle {
        Self::new(&vec![
            ("tab:blue", Color(0x1f77b4ff)),
            ("tab:orange", Color(0xff7f0eff)),
            ("tab:green", Color(0x2ca02cff)),
            ("tab:red", Color(0xd62728ff)),
            ("tab:purple", Color(0x9467bdff)),
            ("tab:brown", Color(0x8c564bff)),
            ("tab:pink", Color(0xe377c2ff)),
            ("tab:gray", Color(0x7f7f7fff)),
            ("tab:olive", Color(0xbcbd22ff)),
            ("tab:cyan", Color(0xf17becff)),
        ])
    }
}

impl Default for ColorCycle {
    fn default() -> Self {
        ColorCycle::tableau()
    }
}

pub struct Lab(f32, f32, f32);

impl Lab {
    #[inline]
    pub fn l(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn a(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.2
    }
}

impl fmt::Debug for Lab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Lab").field(&self.0).field(&self.1).field(&self.2).finish()
    }
}


impl From<Lab> for Color {
    fn from(value: Lab) -> Self {
        Color::from_lab(value.l(), value.a(), value.b())
    }
}

impl From<Color> for Lab {
    fn from(color: Color) -> Self {
        let [l, a, b] = color.to_lab();

        Self(l, a, b)
    }
}

///
/// Hue, saturation, value.
/// 
/// Note: hue is unit circle based: [0, 1], not degree based.
/// 
pub struct Hsv(pub f32, pub f32, pub f32);

impl Hsv {
    #[inline]
    pub fn h(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn s(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn v(&self) -> f32 {
        self.2
    }
}

impl fmt::Debug for Hsv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Hsv").field(&self.0).field(&self.1).field(&self.2).finish()
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Hsva(pub f32, pub f32, pub f32, pub f32);

impl Hsva {
    #[inline]
    pub fn h(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn s(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn v(&self) -> f32 {
        self.2
    }

    #[inline]
    pub fn a(&self) -> f32 {
        self.3
    }
}

impl fmt::Debug for Hsva {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Hsva")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .field(&self.3)
            .finish()
    }
}

impl From<Hsva> for Color {
    #[inline]
    fn from(value: Hsva) -> Color {
        Color::from(&value)
    }
}

impl From<&Hsva> for Color {
    fn from(value: &Hsva) -> Color {
        let Hsva(h, s, v, a) = value;

        let h = h.clamp(0., 1.);
        let s = s.clamp(0., 1.);
        let v = v.clamp(0., 1.);
        let a = a.clamp(0., 1.);

        let i = (h * 6.) as u32;
        let ff = h * 6. - i as f32;
        let p = v * (1. - s);
        let q = v * (1. - (s * ff));
        let t = v * (1. - (s * (1. - ff)));

        let (r, g, b) = match i {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };

        Color::from_rgba(r, g, b, a)
    }
}

impl From<Color> for Hsva {
    fn from(color: Color) -> Hsva {
        let (r, g, b, a) = (color.r8(), color.g8(), color.b8(), color.a8());

        let r = r as f32 / 255.;
        let g = g as f32 / 255.;
        let b = b as f32 / 255.;
        let a = a as f32 / 255.;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);

        let c = max - min;
        let s = c / max;

        let r_s = (max - r) / c;
        let g_s = (max - g) / c;
        let b_s = (max - b) / c;

        let h = if min == max {
            0.
        } else if max == r {
            b_s - g_s
        } else if max == g {
            2. + r_s - b_s
        } else {
            4. + g_s - r_s
        };

        let h = (h / 6. + 1.) % 1.;

        Hsva(h, s, max, a)
    }
}

impl From<Hsv> for Color {
    #[inline]
    fn from(value: Hsv) -> Color {
        Color::from(&value)
    }
}

impl From<&Hsv> for Color {
    #[inline]
    fn from(value: &Hsv) -> Color {
        let Hsv(h, s, v) = value;

        Color::from(&Hsva(*h, *s, *v, 1.))
    }
}

#[cfg(test)]
mod test {
    use essay_graphics_api::Color;

    use crate::artist::color::Hsva;

    #[test]
    fn hsva_to_color() {
        assert_eq!(Color::from(Hsva(0., 1., 1., 1.,)), Color(0xff00_00ff));
        assert_eq!(Color::from(Hsva(1./6., 1., 1., 1.,)), Color(0xffff_00ff));
        assert_eq!(Color::from(Hsva(2./6., 1., 1., 1.,)), Color(0x00ff_00ff));
        assert_eq!(Color::from(Hsva(3./6., 1., 1., 1.,)), Color(0x00ff_ffff));
        assert_eq!(Color::from(Hsva(4./6., 1., 1., 1.,)), Color(0x0000_ffff));
        assert_eq!(Color::from(Hsva(5./6., 1., 1., 1.,)), Color(0xff00_ffff));

        assert_eq!(Color::from(Hsva(0./6., 0.5, 1., 1.,)), Color(0xff7f_7fff));

        assert_eq!(Color::from(Hsva(0./6., 1., 0.5, 1.,)), Color(0x7f00_00ff));
    }

    #[test]
    fn color_to_hsva() {
        assert_eq!(Hsva::from(Color(0xff00_00ff)), Hsva(0., 1., 1., 1.));
        assert_eq!(Hsva::from(Color(0xffff_00ff)), Hsva(0.16666663, 1., 1., 1.));
        assert_eq!(Hsva::from(Color(0x00ff_00ff)), Hsva(0.33333337, 1., 1., 1.));
        assert_eq!(Hsva::from(Color(0x00ff_ffff)), Hsva(0.5, 1., 1., 1.));
        assert_eq!(Hsva::from(Color(0x0000_ffff)), Hsva(0.66666675, 1., 1., 1.));
        assert_eq!(Hsva::from(Color(0xff00_ffff)), Hsva(5./6., 1., 1., 1.));

        assert_eq!(Hsva::from(Color(0x8000_00ff)), Hsva(0., 1., 0.5019608, 1.));
        assert_eq!(Hsva::from(Color(0xff7f_7fff)), Hsva(0., 0.50196075, 1., 1.));
    }
}