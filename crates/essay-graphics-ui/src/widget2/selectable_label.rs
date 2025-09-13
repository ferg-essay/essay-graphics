use std::marker::PhantomData;

use essay_graphics_api::{renderer::{Renderer}, Padding, PathStyle, Point, Shapes, Size};

use crate::{
    style::{Style, UiStyle}, 
    ui::{Response, ResponseValue, Shell, Ui, Widget}, 
    widget2::Text
};

pub fn selectable_label<'a, Message>(
    content: impl Into<Text>
) -> SelectableLabel<'a, Message> {
    SelectableLabel::new(content.into())
}

pub struct SelectableLabel<'a, Message> {
    label: Text,

    is_selected: bool,

    on_press: OnPress<Message>,

    marker: PhantomData<&'a Message>,
}

impl<'a, Message> SelectableLabel<'a, Message> {
    pub fn new(label: impl Into<Text>) -> Self {
        Self {
            label: label.into(),
            is_selected: false,
            on_press: OnPress::None,
            marker: Default::default(),
        }
    }

    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = OnPress::Message(message);

        self
    }
}

impl<'a, Message> Widget<Message> for SelectableLabel<'a, Message>
where
    Message: Clone
{
    fn ui(
        &mut self, 
        ui: &mut Ui,
        shell: &mut Shell<Message>,
    ) -> Response {
        let text_style = ui.theme().button_text.clone();
        let size = ui.text_size(&self.label.value(), &text_style);

        let style = Style::ButtonOn;

        let corner = style.corner_radius(ui);
        let pad = style.padding(ui);
        let margin = pad + corner;
        let border_width = style.border_width(ui);

        let size = Size::new(
            size.width + margin.width(), 
            size.height + margin.height(),
        );

        let ResponseValue {
            value: bounds,
            response
        } = ui.allocate(size);

        let pos = Point::new(bounds.xmin() + margin.left, bounds.ymin() + margin.top);

        let inner = bounds - Padding::from_pair(corner, corner);

        if response.clicked(ui) {
            ui.close_popup();

            match &self.on_press {
                OnPress::None => {},
                OnPress::Message(message) => {
                    shell.publish(message.clone());
                },
            }
        }

        let style = if self.is_selected { Style::ButtonOn } else { Style::ButtonOff };

        let is_hover = response.is_hover(ui);

        let background = style.background_on_hover(ui, is_hover);
        let foreground = style.foreground_on_hover(ui, is_hover);
        let border = style.border_on_hover(ui, is_hover);
        
        let label = String::from(self.label.value());

        let border = background;
        let corner = style.corner_radius(ui);

        ui.painter().add(move |ui: &mut dyn Renderer| {
            let mut style = PathStyle::new();

            let sz = border_width;
            let r = corner;

            ui.draw_shape(&Shapes::quad(
                inner.p0() - Point::new(sz, sz), 
                inner.size() + Size::new(2. * sz, 2. * sz), 
                r + border_width, 
                border,
                r,
                background,
            ))?;
            /*
            if sz > 0. { // border
                ui.draw_shape(&Shapes::rect(
                    inner.p0() - Point::new(sz, sz), 
                    inner.size() + Size::new(2. * sz, 2. * sz), 
                    r + 1., 
                    border,
                ))?;
            }

            ui.draw_shape(&Shapes::rect(
                inner.p0(), inner.size(), r, background,
            ))?;
            */
            // ui.draw_path(&background, &style)?;
            style.color(foreground);
            ui.draw_text(pos, &label, 0., &style, &text_style)
        });

        response
    }
}

enum OnPress<Message> {
    None,
    Message(Message),
}