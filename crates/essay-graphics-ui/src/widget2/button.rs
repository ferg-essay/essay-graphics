use essay_graphics_api::{
    renderer::{Canvas, Renderer}, 
    Bounds, Color, HorizAlign, Padding, PathStyle, Point, Shapes, Size, TextStyle};

use crate::{
    style::{UiStyle},
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
        let (background, foreground) = {
            if is_hover {
                (style.hover_background(ui), style.hover_foreground(ui))
            } else {
                (style.background(ui), style.foreground(ui))
            }
        };

        //let label = self.label.clone();

        let border = background;
        let corner = style.corner_radius(ui);
        let label = String::from(self.content.value());

        ui.painter().add(move |ui: &mut dyn Renderer| {
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
            let mut style = PathStyle::new();
            let mut button_text = TextStyle::new();
            button_text.halign(HorizAlign::Left);
            //button_text.valign(VertAlign::Top);
            style.edge_color(foreground);
            style.face_color(foreground);
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

        let corner = OnStyle.corner_radius(ui);
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
            self.draw(ui, pos, inner, OnStyle, is_hover);
        } else {
            self.draw(ui, pos, inner, OffStyle, is_hover);
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

pub struct OnStyle;

impl UiStyle for OnStyle {
    fn background(&self, ui: &Ui) -> Color {
        ui.theme().button2_on.background
    }

    fn foreground(&self, ui: &Ui) -> Color {
        ui.theme().button2_on.foreground
    }

    fn border(&self, _ui: &Ui) -> Color {
        Color(0)
    }

    fn border_width(&self, _ui: &Ui) -> f32 {
        0.
    }

    fn corner_radius(&self, ui: &Ui) -> f32 {
        ui.theme().corner_radius
    }

    fn hover_background(&self, ui: &Ui) -> Color {
        ui.theme().button2_on.hover_background
    }

    fn hover_foreground(&self, ui: &Ui) -> Color {
        ui.theme().button2_on.hover_foreground
    }

    fn hover_border(&self, _ui: &Ui) -> Color {
        Color(0)
    }
}


pub struct OffStyle;

impl UiStyle for OffStyle {
    fn background(&self, ui: &Ui) -> Color {
        ui.theme().button2_off.background
    }

    fn foreground(&self, ui: &Ui) -> Color {
        ui.theme().button2_off.foreground
    }

    fn border(&self, _ui: &Ui) -> Color {
        Color(0)
    }

    fn border_width(&self, _ui: &Ui) -> f32 {
        0.
    }

    fn corner_radius(&self, ui: &Ui) -> f32 {
        ui.theme().corner_radius
    }

    fn hover_background(&self, ui: &Ui) -> Color {
        ui.theme().button2_off.hover_background
    }

    fn hover_foreground(&self, ui: &Ui) -> Color {
        ui.theme().button2_off.hover_foreground
    }

    fn hover_border(&self, _ui: &Ui) -> Color {
        Color(0)
    }
}