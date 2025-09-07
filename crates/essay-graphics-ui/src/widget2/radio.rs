use std::marker::PhantomData;

use essay_graphics_api::{color::Grey, renderer::{Canvas, Renderer}, Bounds, Shapes, Size};

use crate::{ui::{Response, ResponseValue, Shell, Ui, Widget}, widget2::{Element, Text}};

pub fn radio_value<'a, Message, V: PartialEq>(
    label: impl Into<Text>,
    var: V,
    value: V,
) -> Radio<'a, Message, V> {
    Radio {
        label: label.into(),

        is_selected: var == value,
        value: Some(value),

        on_press: OnPress::None,

        marker: Default::default(),
    }
}

pub fn radio<'a, Message>(
    content: impl Into<Text>,
    is_selected: bool,
) -> Radio<'a, Message, Message> {
    Radio::new(content, is_selected)
}

pub struct Radio<'a, Message, V> {
    label: Text,

    is_selected: bool,
    value: Option<V>,

    on_press: OnPress<'a, Message, V>,

    marker: PhantomData<&'a Message>,
}

impl<'a, Message> Radio<'a, Message, Message> {
    pub fn new(label: impl Into<Text>, is_selected: bool) -> Self {
        Self {
            label: label.into(),

            is_selected,
            value: None,

            on_press: OnPress::None,

            marker: Default::default(),
        }
    }
}

impl<'a, Message, V> Radio<'a, Message, V> {
    #[must_use]
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = OnPress::Direct(on_press);

        self
    }

    #[must_use]
    pub fn on_press_with(mut self, on_press: impl Fn(V) -> Message + 'a) -> Self {
        self.on_press = OnPress::Closure(Box::new(on_press));

        self
    }
}

impl<'a, Message: Clone, V: Clone> Widget<Message> for Radio<'a, Message, V> {
    fn ui(
        &mut self, 
        ui: &mut Ui,
        shell: &mut Shell<Message>,
    ) -> Response {
        let text_style = ui.style().button_text.clone();
        let size = ui.text_size(self.label.value(), &text_style);

        let height = size.height;
        let pad = 10.;

        let size = Size::new(size.width + pad + height, size.height);

        let ResponseValue {
            value: _bounds,
            response
        } = ui.allocate(size);

        let pos = response.rect(ui);

        let ui_style = ui.style();

        let is_active = self.is_selected;

        let (background, foreground) = {
            if is_active {
                (ui_style.button2_on.background, ui_style.button2_on.foreground)
            } else {
                (Grey(1.0).into(), Grey(0.90).into())
            }
        };
        
        let label = String::from(self.label.value());
        let style = ui.style().button.clone();

        // let is_active = self.is_selected;

        let radio_pos = Bounds::<Canvas>::from([
            [pos.xmax() - height, pos.ymin()],
            [pos.xmax(), pos.ymax()],
        ]);

        // style.color(foreground);
        ui.painter().add(move |ui: &mut dyn Renderer| {
            ui.draw_text(pos.p0(), &label, 0., &style, &text_style)?;

            if is_active {
                let r0 = radio_pos.height() * 0.5;

                ui.draw_shape(&Shapes::quad(
                    radio_pos.p0(),
                    radio_pos.size(),
                    r0,
                    background,
                    r0 - 6.,
                    foreground,
                ))?;

            } else {
                let r0 = radio_pos.height() * 0.5;

                ui.draw_shape(&Shapes::quad(
                    radio_pos.p0(),
                    radio_pos.size(),
                    r0,
                    foreground,
                    r0 - 2.,
                    background,
                ))?;
            }

            Ok(())
        });

        if response.clicked(ui) {
            self.on_press.select(&self.value, shell);
        }

        response
    }
}

impl<'a, Message, V> From<Radio<'a, Message, V>> for Element<'a, Message>
where
    Message: Clone + 'a,
    V: Clone + 'a
{
    fn from(radio: Radio<'a, Message, V>) -> Self {
        Self::new(radio)
    }
}

enum OnPress<'a, Message, V> {
    None,
    Direct(Message),
    Closure(Box<dyn Fn(V) -> Message + 'a>),
}

impl<'a, Message: Clone, V: Clone> OnPress<'a, Message, V> {
    fn select(
        &self, 
        value: &Option<V>, 
        shell: &mut Shell<Message>
    ) {
        match self {
            OnPress::None => {},
            OnPress::Direct(message) => {
                shell.publish(message.clone())
            },
            OnPress::Closure(on_press) => {
                if let Some(value) = value {
                    shell.publish(on_press(value.clone()))
                }
            },
        }
    }
}