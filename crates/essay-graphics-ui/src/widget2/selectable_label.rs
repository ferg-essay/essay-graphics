use std::marker::PhantomData;

use essay_graphics_api::{renderer::{Renderer}, Padding, PathStyle, Point, Shapes, Size};

use crate::{
    style::{UiStyle}, 
    ui::{Response, ResponseValue, Shell, Ui, Widget}, 
    widget2::{OffStyle, OnStyle, Text}
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

        let style = OnStyle;

        let corner = style.corner_radius(ui);
        let pad = 4.;
        let margin = pad + corner;

        let size = Size::new(size.width + 2. * margin, size.height + 2. * margin);

        let ResponseValue {
            value: bounds,
            response
        } = ui.allocate(size);

        //let bounds = bounds.round_ui();

        let pos = Point::new(bounds.xmin() + margin, bounds.ymin() + margin);

        let inner = bounds - Padding::from_pair(corner, corner);
        // println!("Size {:?} Bounds {:?} {:?}", size, bounds, inner);

        // let mut style = ui.style().button.clone();

        // let ui_style = ui.style();
        if response.clicked(ui) {
            ui.close_popup();

            match &self.on_press {
                OnPress::None => {},
                OnPress::Message(message) => {
                    shell.publish(message.clone());
                },
            }
        }

        let (background, foreground) = {
            let is_active = self.is_selected;

            if is_active {
                if response.is_hover(ui) {
                    (OnStyle.hover_background(ui), OnStyle.hover_foreground(ui))
                } else {
                    (OnStyle.background(ui), OnStyle.foreground(ui))
                }
            } else {
                if response.is_hover(ui) {
                    (OffStyle.hover_background(ui), OffStyle.hover_foreground(ui))
                } else {
                    (OffStyle.background(ui), OffStyle.foreground(ui))
                }
            }
        };
        
        //style.edge_color(ui.style()[state].edge);

        let label = String::from(self.label.value());

        let border = background;
        let corner = OnStyle.corner_radius(ui);

        ui.painter().add(move |ui: &mut dyn Renderer| {
            let mut style = PathStyle::new();

            let sz = 0.;
            let r = corner;
            if sz > 0. { // border
                ui.draw_shape(&Shapes::rect(
                    inner.p0() - Point::new(sz, sz), inner.size() + Size::new(2. * sz, 2. * sz), r + 1., 
                    border,
                ))?;
            }

            ui.draw_shape(&Shapes::rect(
                inner.p0(), inner.size(), r, background,
            ))?;
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