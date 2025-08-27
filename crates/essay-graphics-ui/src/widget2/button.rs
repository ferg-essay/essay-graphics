use essay_graphics_api::Rectangle;

use crate::{ui::Ui, widget2::{Element, Widget}};

pub fn button<'a, Message>(
    content: impl Into<Element<'a, Message>>
) -> Button<'a, Message> {
    Button {
        content: content.into(),
        on_press: None,
    }
}

pub struct Button<'a, Message> {
    content: Element<'a, Message>,   
    on_press: Option<OnPress<'a, Message>>,
}

impl<'a, Message> Button<'a, Message>
{
    pub fn new(
        content: impl Into<Element<'a, Message>>,
    ) -> Self {
        let content = content.into();

        Self {
            content,
            on_press: None,
        }
    }

    #[must_use]
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(OnPress::Direct(on_press));
        self
    }

    #[must_use]
    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.on_press = on_press.map(OnPress::Direct);
        self
    }

    #[must_use]
    pub fn on_press_with(
        mut self,
        on_press: impl Fn() -> Message + 'a,
    ) -> Self {
        self.on_press = Some(OnPress::Closure(Box::new(on_press)));
        self
    }
}

impl<'a, Message> Widget<Message>
    for Button<'a, Message>
{
    fn draw(
        &mut self,
        ui: &mut Ui,
        bounds: &Rectangle
    ) {
        println!("Draw!");
    }
}

impl<'a, Message> From<Button<'a, Message>>
    for Element<'a, Message>
where
    Message: 'a,
{
    fn from(button: Button<'a, Message>) -> Self {
        Self::new(button)
    }
}

enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

impl<Message: Clone> OnPress<'_, Message> {
    fn get(&self) -> Message {
        match self {
            OnPress::Direct(message) => message.clone(),
            OnPress::Closure(f) => f(),
        }
    }
}

struct State {
    is_pressed: bool,
}