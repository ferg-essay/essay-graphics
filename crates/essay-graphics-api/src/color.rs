use core::fmt;
use std::str::FromStr;

use crate::color_data::lookup_color_name;

#[derive(Clone, Copy, PartialEq)]
pub struct Color(pub u32);

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color({:8x})", self.0)
    }
}

impl Color {
    #[inline]
    pub fn black() -> Color {
        Color(0x000000ff)
    }

    #[inline]
    pub fn white() -> Color {
        Color(0xffffffff)
    }

    #[inline]
    pub fn none() -> Color {
        Color(0x0)
    }
    
    #[inline]
    pub fn from_rgb(red: f32, green: f32, blue: f32) -> Color {
        Color::from_rgba(red, green, blue, 1.0)
    }

    #[inline]
    pub fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        let r = (red * 255.).clamp(0., 255.) as u32;
        let g = (green * 255.).clamp(0., 255.) as u32;
        let b = (blue * 255.).clamp(0., 255.) as u32;
        let a = (alpha * 255.).clamp(0., 255.) as u32;

        Color((r << 24) | (g << 16) | (b << 8) | a)
    }
    
    #[inline]
    pub fn from_grey(value: f32) -> Color {
        Self::from_rgb(value, value, value)
    }

    #[inline]
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Color {
        Color::from(Hsva(h, s, v, 1.))
    }

    #[inline]
    pub fn red(&self) -> f32 {
        ((self.0 >> 24) & 0xff) as f32 / 255.
    }

    #[inline]
    pub fn green(&self) -> f32 {
        ((self.0 >> 16) & 0xff) as f32 / 255.
    }

    #[inline]
    pub fn blue(&self) -> f32 {
        ((self.0 >> 8) & 0xff) as f32 / 255.
    }

    #[inline]
    pub fn alpha(&self) -> f32 {
        (self.0 & 0xff) as f32 / 255.
    }

    #[inline]
    #[must_use]
    pub fn with_alpha(self, arg: f32) -> Color {
        Color::from_rgba(
            self.red(), 
            self.green(), 
            self.blue(),
            arg
        )
    }

    #[inline]
    pub fn r8(&self) -> u8 {
        ((self.0 >> 24) & 0xff) as u8
    }

    #[inline]
    pub fn g8(&self) -> u8 {
        ((self.0 >> 16) & 0xff) as u8
    }

    #[inline]
    pub fn b8(&self) -> u8 {
        ((self.0 >> 8) & 0xff) as u8
    }

    #[inline]
    pub fn a8(&self) -> u8 {
        (self.0 & 0xff) as u8
    }

    #[inline]
    pub fn to_rgb(&self) -> u32 {
        self.0 >> 8
    }

    #[inline]
    pub fn to_rgba(&self) -> u32 {
        self.0
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        self.0 == 0
    }

    pub fn nearest_name(&self) -> String {
        lookup_color_name(self)
    }

    #[inline]
    pub fn to_rgba_vec(&self) -> [u8; 4] {
        [self.r8(), self.g8(), self.b8(), self.a8() ]
    }

    #[inline]
    pub fn srgb_to_lrgb(srgb: f32) -> f32 {
        if srgb > 0.04045 {
            ((srgb + 0.055) / 1.055).powf(2.4)
        } else {
            srgb / 12.92
        }
    }

    #[inline]
    pub fn lrgb_to_srgb(lrgb: f32) -> f32 {
        if lrgb > 0.0031308 {
            1.055 * lrgb.powf(1./2.4) - 0.055
        } else {
            lrgb * 12.92
        }
    }

    #[inline]
    pub fn to_lrgb(&self) -> [f32; 4] {
        [
            Self::srgb_to_lrgb(self.red()),
            Self::srgb_to_lrgb(self.green()),
            Self::srgb_to_lrgb(self.blue()),
            self.alpha(),
        ]
    }

    #[inline]
    pub fn from_lrgb(&self) -> [f32; 4] {
        [
            Self::srgb_to_lrgb(self.red()),
            Self::srgb_to_lrgb(self.green()),
            Self::srgb_to_lrgb(self.blue()),
            1.
        ]
    }

    /*
    /// Morland MSH in Diverging Color Maps for Scientific Visualization
    #[inline]
    pub fn to_msh(&self) -> [f32; 3] {
        let [l, a, b] = self.to_lab();

        lab_to_msh(l, a, b)
    }

    #[inline]
    pub fn from_msh(m: f32, s: f32, h: f32) -> Color {
        let [l, a, b] = msh_to_lab(m, s, h);

        Self::from_lab(l, a, b)
    }
    */
}

/*
// msh in Morland, Diverging Color Maps for Scientific Visualization
fn lab_to_msh(l: f32, a: f32, b: f32) -> [f32; 3] {
    let m = (l * l + a * a + b * b).sqrt();
    let a = if a != 0. { a } else { f32::EPSILON };
    [
        m,
        (l / m.max(f32::EPSILON)).acos(),
        b.atan2(a)
    ]
}

fn msh_to_lab(m: f32, s: f32, h: f32) -> [f32; 3] {
    [
        m * s.cos(),
        m * s.sin() * h.cos(),
        m * s.sin() * h.sin(),
    ]
}
*/

impl From<u32> for Color {
    #[inline]
    fn from(value: u32) -> Self {
        Color((value & 0xffffff) * 256 + 0xff)
    }
}

impl From<[u32; 3]> for Color {
    #[inline]
    fn from(rgb: [u32; 3]) -> Self {
        Color(
            (rgb[0] & 0xff) << 24
            | (rgb[1] & 0xff) << 16
            | (rgb[2] & 0xff) << 8
            | 0xff
        )
    }
}

impl From<Color> for [u32; 3] {
    #[inline]
    fn from(color: Color) -> Self {
        [
            color.r8() as u32,
            color.g8() as u32,
            color.b8() as u32,
        ]
    }
}

impl From<[u32; 4]> for Color {
    #[inline]
    fn from(rgba: [u32; 4]) -> Self {
        Color(
            (rgba[0] & 0xff) << 24
            | (rgba[1] & 0xff) << 16
            | (rgba[2] & 0xff) << 8
            | (rgba[3] & 0xff)
        )
    }
}

impl From<Color> for [u32; 4] {
    #[inline]
    fn from(color: Color) -> Self {
        [
            color.r8() as u32,
            color.g8() as u32,
            color.b8() as u32,
            color.a8() as u32,
        ]
    }
}

impl From<[f32; 3]> for Color {
    #[inline]
    fn from(value: [f32; 3]) -> Self {
        Color::from_rgb(value[0], value[1], value[2])
    }
}

impl From<Color> for [f32; 3] {
    #[inline]
    fn from(value: Color) -> Self {
        [
            value.red(),
            value.green(),
            value.blue(),
        ]
    }
}

impl From<[f32; 4]> for Color {
    fn from(value: [f32; 4]) -> Self {
        Color::from_rgba(value[0], value[1], value[2], value[3])
    }
}

impl From<&str> for Color {
    fn from(name: &str) -> Self {
        name.parse::<Color>().unwrap()
    }
}

impl FromStr for Color {
    type Err = ColorErr;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        if let Some(color) = super::color_data::lookup_color(name) {
            return Ok(color);
        }

        if name.starts_with("#") {
            let mut value = 0;
            for ch in name.as_bytes().iter().skip(1) {
                match ch {
                    b'0'..=b'9' => { value = 16 * value + *ch as u32 - b'0' as u32; }
                    b'a'..=b'f' => { value = 16 * value + *ch as u32 - b'a' as u32 + 10; }
                    b'A'..=b'F' => { value = 16 * value + *ch as u32 - b'A' as u32 + 10; }
                    _ => {
                        return Err(ColorErr(format!("Invalid rgb color spec {:?}", name)));
                    }
                }
            }

            return Ok(Color::from(value))
        }

        return Err(ColorErr(format!("'{}' is an unknown color name", name)));
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Grey(pub f32);

impl Grey {
    #[inline]
    pub fn grey(&self) -> f32 {
        self.0
    }
}

impl From<Grey> for Color {
    #[inline]
    fn from(value: Grey) -> Self {
        Color::from_rgb(value.0, value.0, value.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb(pub f32, pub f32, pub f32);

impl Rgb {
    #[inline]
    pub fn red(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn green(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn blue(&self) -> f32 {
        self.2
    }
}

impl From<Rgb> for Color {
    #[inline]
    fn from(value: Rgb) -> Self {
        Color::from_rgb(value.0, value.1, value.2)
    }
}

impl From<Color> for Rgb {
    #[inline]
    fn from(value: Color) -> Self {
        Self(
            value.red(),
            value.green(),
            value.blue(),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba(pub f32, pub f32, pub f32, pub f32);

impl Rgba {
    #[inline]
    pub fn red(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn green(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn blue(&self) -> f32 {
        self.2
    }

    #[inline]
    pub fn alpha(&self) -> f32 {
        self.3
    }
}

impl From<Rgba> for Color {
    #[inline]
    fn from(value: Rgba) -> Self {
        Color::from_rgba(value.0, value.1, value.2, value.3)
    }
}

impl From<Color> for Rgba {
    #[inline]
    fn from(value: Color) -> Self {
        Self(
            value.red(),
            value.green(),
            value.blue(),
            value.alpha(),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl From<Hsv> for Color {
    fn from(Hsv(h, s, v): Hsv) -> Self {
        Color::from(Hsva(h, s, v, 1.))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
        self.2
    }
}

impl From<Hsva> for Color {
    #[inline]
    fn from(value: Hsva) -> Self {
        let Hsva(h, s, v, a) = value;

        let h = h.clamp(0., 360.);
        let h = h / 360.;
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
    
        Self::from_rgba(r, g, b, a)
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

        Hsva(360. * h, s, max, a)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Xyz(pub f32, pub f32, pub f32);

impl Xyz {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.2
    }
}

impl From<Xyz> for Color {
    #[inline]
    fn from(value: Xyz) -> Self {
        let Xyz(x, y, z) = value;

        Color::from_rgb(
            3.240479 * x - 1.537150 * y - 0.498535 * z,
            -0.969256 * x + 1.875992 * y + 0.041556 * z,
            0.055648 * x - 0.204043 * y + 1.057311 * z,
        )
    }
}

impl From<Color> for Xyz {
    #[inline]
    fn from(value: Color) -> Self {
        let [r, g, b]: [f32; 3] = value.into();

        Self(
            0.412453 * r + 0.357580 * g + 0.180423 * b,
            0.212671 * r + 0.715160 * g + 0.072169 * b,
            0.019334 * r + 0.119193 * g + 0.950227 * b,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lab(pub f32, pub f32, pub f32);

impl Lab {
    pub const D50 : [f32; 3] = [0.964212, 1.0, 0.825188];
    pub const D65 : [f32; 3] = [0.950489, 1.0, 1.088840];

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

impl From<Lab> for Color {
    #[inline]
    fn from(value: Lab) -> Self {
        let Lab(l, a, b) = value;
        let [xn, yn, zn] = Lab::D65;

        fn f(x: f32) -> f32 {
            let x3 = x * x * x;

            if x3 > 0.008856 {
                x3
            } else {
                (116. * x - 16.) / 903.3
            }
        }

        let f_y = (l + 16.) / 116.;

        Color::from(Xyz(
            xn * f(f_y + a / 500.),
            yn * f(f_y),
            zn * f(f_y - b / 200.),
        ))
    }
}

impl From<Color> for Lab {
    #[inline]
    fn from(value: Color) -> Self {
        let Xyz(x, y, z) = value.into();
        let [xn, yn, zn] = Self::D65;

        fn f(x: f32) -> f32 {
            if x > 0.008856 {
                x.powf(1. / 3.)
            } else {
                7.787 * x + 16. / 116.
            }
        }

        Self(
            116. * f(y / yn) - 16.,
            500. * (f(x / xn) - f(y / yn)),
            200. * (f(y / yn) - f(z / zn)),
        )
    }
}

#[derive(Clone)]
pub struct Colors {
    pub colors: Vec<Color>,
}

impl Colors {
    fn new(colors: Vec<Color>) -> Self {
        Self {
            colors
        }
    }

    pub fn into(self) -> Vec<Color> {
        self.colors
    }
}

impl<const N: usize> From<[&str; N]> for Colors {
    fn from(value: [&str; N]) -> Self {
        let mut vec = Vec::new();

        for name in value {
            vec.push(Color::from(name));
        }

        Colors::new(vec)
    }
}

#[derive(Clone, Debug)]
pub struct ColorErr(pub String);

#[cfg(test)]
mod test {
    use crate::{color::{Lab, Xyz}, Color};

    /// CIE xyz color
    #[test]
    fn test_xyz() {
        assert_eq!(Xyz::from(Color(0x0000_00ff)), Xyz(0., 0., 0.));
        assert_eq!(Xyz::from(Color(0xffff_ffff)), Xyz(0.950456, 1.0, 1.088754));

        assert_eq!(Xyz::from(Color(0xff00_00ff)), Xyz(0.412453, 0.212671, 0.019334));
        assert_eq!(Xyz::from(Color(0x00ff_00ff)), Xyz(0.35758, 0.71516, 0.119193));
        assert_eq!(Xyz::from(Color(0x0000_ffff)), Xyz(0.180423, 0.072169, 0.950227));

        assert_eq!(Color::from(Xyz(0., 0., 0.)), Color(0x0000_00ff));
        assert_eq!(Color::from(Xyz(1., 0., 0.)), Color(0xff00_0eff));
        assert_eq!(Color::from(Xyz(0., 1., 0.)), Color(0x00ff_00ff));
        assert_eq!(Color::from(Xyz(0., 0., 1.)), Color(0x000a_ffff));
        assert_eq!(Color::from(Xyz(1., 1., 1.)), Color(0xfff1_e7ff));

        assert_eq!(Color::from(Xyz(0.1, 0.0, 0.0)), Color(0x5200_01ff));
        assert_eq!(Color::from(Xyz(0.0, 0.1, 0.0)), Color(0x002f_00ff));
        assert_eq!(Color::from(Xyz(0.0, 0.0, 0.1)), Color(0x0001_1aff));
        assert_eq!(Color::from(Xyz(0.1, 0.1, 0.1)), Color(0x1e18_17ff));

        // full colors
        let Xyz(x, y, z) = Xyz::from(Color(0xff00_00ff));
        assert_eq!(Color::from(Xyz(x, y, z)), Color(0xfe00_00ff));

        let Xyz(x, y, z) = Xyz::from(Color(0x00ff_00ff));
        assert_eq!(Color::from(Xyz(x, y, z)), Color(0x00ff_00ff));

        let Xyz(x, y, z) = Xyz::from(Color(0x0000_ffff));
        assert_eq!(Color::from(Xyz(x, y, z)), Color(0x0000_ffff));

        let Xyz(x, y, z) = Xyz::from(Color(0xffff_ffff));
        assert_eq!(Color::from(Xyz(x, y, z)), Color(0xffff_feff));

        // low colors
        let Xyz(x, y, z) = Xyz::from(Color(0x0400_00ff));
        assert_eq!(Color::from(Xyz(x, y, z)), Color(0x0300_00ff));

        let Xyz(x, y, z) = Xyz::from(Color(0x0004_00ff));
        assert_eq!(Color::from(Xyz(x, y, z)), Color(0x0004_00ff));

        let Xyz(x, y, z) = Xyz::from(Color(0x0000_04ff));
        assert_eq!(Color::from(Xyz(x, y, z)), Color(0x0000_04ff));

        let Xyz(x, y, z) = Xyz::from(Color(0x0404_04ff));
        assert_eq!(Color::from(Xyz(x, y, z)), Color(0x0404_03ff));
    }

    /// CIE lab color
    #[test]
    fn test_lab() {
        assert_eq!(Lab::from(Color(0x0000_00ff)), Lab(0., 0., 0.));
        assert_eq!(Lab::from(Color(0xffff_ffff)), Lab(100.0, -0.0057816505, 0.0052571297));

        assert_eq!(Lab::from(Color(0xff00_00ff)), Lab(53.240585, 80.089806, 67.20291));
        assert_eq!(Lab::from(Color(0x00ff_00ff)), Lab(87.7351, -86.185425, 83.18));
        assert_eq!(Lab::from(Color(0x0000_ffff)), Lab(32.29567, 79.183685, -107.85673));

        assert_eq!(Color::from(Lab(53.24, 80.09, 67.20)), Color(0xfe00_00ff));

        // full colors
        let Lab(l, a, b) = Lab::from(Color(0xff00_00ff));
        assert_eq!(Color::from(Lab(l, a, b)), Color(0xfe00_00ff));

        let Lab(l, a, b) = Lab::from(Color(0x00ff_00ff));
        assert_eq!(Color::from(Lab(l, a, b)), Color(0x00ff_00ff));

        let Lab(l, a, b) = Lab::from(Color(0x0000_ffff));
        assert_eq!(Color::from(Lab(l, a, b)), Color(0x0000_ffff));

        let Lab(l, a, b) = Lab::from(Color(0xffff_ffff));
        assert_eq!(Color::from(Lab(l, a, b)), Color(0xffff_feff));

        // low colors
        let Lab(l, a, b) = Lab::from(Color(0x0400_00ff));
        assert_eq!(Color::from(Lab(l, a, b)), Color(0x0300_00ff));

        let Lab(l, a, b) = Lab::from(Color(0x0004_00ff));
        assert_eq!(Color::from(Lab(l, a, b)), Color(0x0004_00ff));

        let Lab(l, a, b) = Lab::from(Color(0x0000_04ff));
        assert_eq!(Color::from(Lab(l, a, b)), Color(0x0000_04ff));

        let Lab(l, a, b) = Lab::from(Color(0x0404_04ff));
        assert_eq!(Color::from(Lab(l, a, b)), Color(0x0404_03ff));
    }

    /*
    /// MSH 
    #[test]
    fn test_msh() {
        assert_eq!(Color(0x0000_00ff).to_msh(), [0., 1.5707964, 0.]);
        //assert_eq!(Color(0xffff_ffff).to_msh(), [100.0, 0.0, -0.73791766]);

        //assert_eq!(Color(0xff00_00ff).to_lab(), [53.240585, 80.089806, 67.20291]);
        //assert_eq!(Color(0x00ff_00ff).to_msh(), [148.47319, 0.9386032, -0.7676548]);
        //assert_eq!(Color(0x0000_ffff).to_lab(), [32.29567, 79.183685, -107.85673]);

        //assert_eq!(Color::from_lab(53.24, 80.09, 67.20), Color(0xfe00_00ff));

        // full colors
        let [m, s, h] = Color(0xff00_00ff).to_msh();
        assert_eq!(Color::from_msh(m, s, h), Color(0xff00_00ff));

        let [m, s, h] = Color(0x00ff_00ff).to_msh();
        assert_eq!(Color::from_msh(m, s, h), Color(0x00ff_00ff));

        let [m, s, h] = Color(0x0000_ffff).to_msh();
        assert_eq!(Color::from_msh(m, s, h), Color(0x0000_ffff));

        let [m, s, h] = Color(0xffff_ffff).to_msh();
        assert_eq!(Color::from_msh(m, s, h), Color(0xfffe_ffff));

        // low colors
        let [m, s, h] = Color(0x0400_00ff).to_msh();
        assert_eq!(Color::from_msh(m, s, h), Color(0x0300_00ff));

        let [m, s, h] = Color(0x0004_00ff).to_msh();
        assert_eq!(Color::from_msh(m, s, h), Color(0x0004_00ff));

        let [m, s, h] = Color(0x0000_04ff).to_msh();
        assert_eq!(Color::from_msh(m, s, h), Color(0x0000_03ff));

        let [m, s, h] = Color(0x0404_04ff).to_msh();
        assert_eq!(Color::from_msh(m, s, h), Color(0x0403_04ff));

        // unsat colors
        let [m, s, h] = Color(0xfcff_ffff).to_msh();
        assert_eq!(Color::from_msh(m, s, h), Color(0xfbff_ffff));

        let [m, s, h] = Color(0xfffc_ffff).to_msh();
        assert_eq!(Color::from_msh(m, s, h), Color(0xfefc_feff));

        let [m, s, h] = Color(0xffff_fcff).to_msh();
        assert_eq!(Color::from_msh(m, s, h), Color(0xffff_fbff));
    }
    */
}
