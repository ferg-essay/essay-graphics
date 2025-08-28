use std::borrow::Cow;

use essay_graphics_api::{Color, Point, Size};

use crate::widget2::{self, Element, Shell, Widget};

pub struct Text<Content = String> {
    pub content: Content,
    // pub font: Font,

    // pub bounds: Size,
}

impl<'a, Message, C: 'static> Widget<Message> for Text<C> {
    fn draw(
        &mut self,
        ui: &mut crate::ui::Ui,
        bounds: &essay_graphics_api::Rectangle,
        shell: &mut Shell<Message>,
    ) -> crate::ui::Response {
        todo!()
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self {
            content: String::from(value),
        }
    }
}

impl<'a, M> From<&str> for Element<'a, M> {
    fn from(value: &str) -> Self {
        Text::from(value).into()
    }
}

impl<'a, C: 'static, M> From<Text<C>> for Element<'a, M> {
    fn from(value: Text<C>) -> Self {
        Element::new(value)
    }
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
