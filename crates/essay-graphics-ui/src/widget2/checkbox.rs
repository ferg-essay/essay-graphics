use std::marker::PhantomData;

use essay_graphics_api::Color;

use crate::widget2::Element;

pub trait Catalog: Sized {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}
pub struct Style {
    pub background: Color,
    pub icon_color: Color,
    pub text_color: Option<Color>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active {
        is_checked: bool,
    },
    Hovered {
        is_checked: bool,
    },
    Disabled {
        is_checked: bool,
    }
}

struct Menu<'a, M, V> {
    values: Vec<MenuItem<'a, M, V>>,
}

struct MenuItem<'a, M, V> {
    value: V,
    content: Element<'a, M>,
}

fn menu<'a, M, V>(
    items: impl IntoIterator<Item=MenuItem<'a, M, V>>,
    value: &V,
    selected: impl Fn(V) -> M,
) -> Menu<'a, M, V> {
    Menu {
        values: Vec::from_iter(items)
    }
}

impl<'a, M, V> Into<MenuItem<'a, M, V>> for (Element<'a, M>, V) {
    fn into(self) -> MenuItem<'a, M, V> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::widget2::checkbox::menu;

    fn test() {
        menu([
            ("test".into(), Test::A).into(),
        ], &Test::A, Msg::Selected);
    }
    enum Test {
        A,
        B,
    }

    enum Msg {
        Selected(Test),
    }
}