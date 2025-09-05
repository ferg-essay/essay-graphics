use essay_graphics_api::{renderer::Renderer, Padding, Point, Shapes, Size};

use crate::{
    ui::{Response, ResponseValue, Shell, Ui, Widget}, 
    widget2::{text::Text, Element, WidgetFrame}
};

pub fn button<'a, Message>(
    content: impl Into<Text>
) -> Button<'a, Message> {
    Button {
        content: content.into(),
        on_press: None,
        press: false,
    }
}

pub struct Button<'a, Message> {
    content: Text,
    on_press: Option<OnPress<'a, Message>>,
    press: bool,
}

impl<'a, Message> Button<'a, Message>
{
    pub fn new(
        content: impl Into<Text>,
    ) -> Self {
        let content = content.into();

        Self {
            content,
            on_press: None,
            press: false,
        }
    }

    #[must_use]
    pub fn press(mut self, press: bool) -> Self {
        self.press = press;
        self
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

impl<'a, Message> Widget<Message> for Button<'a, Message>
where
    Message: Clone + 'a
{
    fn ui(
        &mut self,
        ui: &mut Ui,
        shell: &mut Shell<Message>,
    ) -> Response {
        let button_text = ui.style().button_text.clone();
        let size = ui.text_size(self.content.value(), &button_text);

        let corner = ui.style().corner_radius;
        let pad = 10.;
        let margin = pad + corner;

        let size = Size::new(size.width + 2. * margin, size.height + 2. * margin);

        let ResponseValue {
            value: bounds,
            response
        } = ui.allocate_rect(size);

        //let bounds = bounds.round_ui();

        let pos = Point::new(bounds.xmin() + margin, bounds.ymin() + margin);

        let inner = bounds - Padding::from_pair(corner, corner);
        // println!("Size {:?} Bounds {:?} {:?}", size, bounds, inner);

        let mut style = ui.style().button.clone();

        if response.clicked() {
            match &self.on_press {
                Some(OnPress::Direct(message)) => {
                    shell.publish(message.clone());
                },
                Some(OnPress::Closure(fun)) => {
                    shell.publish(fun());
                },
                None => {}
            }
        }

        let ui_style = ui.style();

        let (background, foreground) = ui.input(|input| {
            let is_active = self.press; // self.press ^ press_one;

            if input.cursor
                .map_or(false, |p| bounds.contains(p)) {
                if is_active {
                    (ui_style.button2_on.hover_background, ui_style.button2_on.hover_foreground)
                } else {
                    (ui_style.button2_off.hover_background, ui_style.button2_off.hover_foreground)
                }
            } else {
                if is_active {
                    (ui_style.button2_on.background, ui_style.button2_on.foreground)
                } else {
                    (ui_style.button2_off.background, ui_style.button2_off.foreground)
                }
            }
        });
        
        //style.edge_color(ui.style()[state].edge);
        style.color(background);

        //let label = self.label.clone();

        let border = background;
        let corner = ui_style.corner_radius;
        let label = String::from(self.content.value());

        ui.painter_mut().add(move |ui: &mut dyn Renderer| {
            let sz = 0.;
            let r = corner;
            if sz > 0. { // border
                ui.draw_shape(&Shapes::Rectangle(
                    inner.p0() - Point::new(sz, sz), inner.size() + Size::new(2. * sz, 2. * sz), r + 1., 
                    border,
                ))?;
            }

            ui.draw_shape(&Shapes::Rectangle(
                inner.p0(), inner.size(), r, background,
            ))?;
            // ui.draw_path(&background, &style)?;
            style.edge_color(foreground);
            style.face_color(foreground);
            ui.draw_text(pos, &label, 0., &style, &button_text)
        });

        /*
        if self.press ^ press_one { 
            style.edge_color(ui.style()[State::Active].foreground);
            style.face_color(ui.style()[State::Active].foreground);
        } else {
            style.edge_color(ui.style()[State::Inactive].foreground);
            style.face_color(ui.style()[State::Inactive].foreground);
        }
        */

        //ui.painter_mut().add(|ui: &mut dyn Renderer| {
        //    ui.draw_text(pos, &self.label, 0., &style, &button_text)
        //});

        //ui.renderer().draw_text(pos, &self.label, 0., &style, &button_text).unwrap();

        //Response::default().with_onclick(press_one)
        response
    }
}

impl<'a, Message> From<Button<'a, Message>>
    for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(button: Button<'a, Message>) -> Self {
        Self::new(button)
    }
}

enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

impl<'a, Message: Clone + 'a> WidgetFrame<'a, Message> for Button<'a, Message> {}