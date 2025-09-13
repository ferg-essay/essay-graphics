use essay_graphics_api::{
    renderer::{Canvas, Renderer}, 
    Bounds, HorizAlign, Padding, PathStyle, Point, Shapes, Size, TextStyle, VertAlign};

use crate::{
    style::{Style, UiStyle},
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

impl<'a, Message> Button<'a, Message> {
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

    fn draw(
        &mut self,
        ui: &mut Ui,
        pos: Point,
        inner: Bounds<Canvas>,
        style: impl UiStyle,
        is_hover: bool,
    ) {
        let background = style.background_on_hover(ui, is_hover);
        let foreground = style.foreground_on_hover(ui, is_hover);
        let border = style.border_on_hover(ui, is_hover);

        let corner = style.corner_radius(ui);
        let border_width = style.border_width(ui);
        let label = String::from(self.content.value());

        ui.painter().add(move |ui: &mut dyn Renderer| {
            let sz = border_width;
            let r = corner;

            ui.draw_shape(&Shapes::quad(
                inner.p0(), 
                inner.size(), 
                r + sz, 
                border,
                r,
                background,
            ))?;

            /*
            if sz > 0. { // border
                ui.draw_shape(&Shapes::rect(
                    inner.p0() - Point::new(sz, sz), inner.size() + Size::new(2. * sz, 2. * sz), r + 1., 
                    border,
                ))?;
            }

            ui.draw_shape(&Shapes::rect(
                inner.p0(), inner.size(), r, background,
            ))?;
            */
            // ui.draw_path(&background, &style)?;
            let mut style = PathStyle::new();
            let mut button_text = TextStyle::new();
            button_text.halign(HorizAlign::Left);
            button_text.valign(VertAlign::Top);
            style.color(foreground);
            ui.draw_text(pos, &label, 0., &style, &button_text)
        });
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
        let button_text = ui.theme().button_text.clone();
        let size = ui.text_size(self.content.value(), &button_text);

        let corner = Style::ButtonOn.corner_radius(ui);
        let pad = 10.;
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

        if response.clicked(ui) {
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

        let is_hover = response.is_hover(ui);

        if self.press {
            self.draw(ui, pos, inner, Style::ButtonOn, is_hover);
        } else {
            self.draw(ui, pos, inner, Style::ButtonOff, is_hover);
        }

        response
    }
}

impl<'a, Message> From<Button<'a, Message>> for Element<'a, Message>
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
