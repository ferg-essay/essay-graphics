use std::borrow::Cow;

use essay_graphics_api::{Color, Point, Size};

use crate::widget2::{self};

pub struct Text<Content = String> {
    pub content: Content,
    // pub font: Font,

    pub bounds: Size,
}

pub type Fragment<'a> = Cow<'a, str>;

pub trait IntoFragment<'a> {
    fn into_fragment(self) -> Fragment<'a>;
}

impl<'a> IntoFragment<'a> for Fragment<'a> {
    fn into_fragment(self) -> Fragment<'a> {
        self
    }
}