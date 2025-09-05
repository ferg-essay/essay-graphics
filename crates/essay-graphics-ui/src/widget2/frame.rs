use essay_graphics_api::{renderer::Renderer, Color, Padding, Point, Shapes};

use crate::{ui::{ui::UiBuilder, Response, Shell, Ui, Widget}, widget2::Element};

pub fn frame<'a, Message>(
    content: impl Into<Element<'a, Message>>
) -> Frame<'a, Message> {
    Frame::new(content)
}

pub struct Frame<'a, Message> {
    content: Element<'a, Message>,   

    pub padding: Padding,
    pub margin: Padding,

    pub background: Option<Color>,
    pub is_shadow: bool,
}

impl<'a, Message> Frame<'a, Message> {
    pub fn new(
        content: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            content: content.into(),

            padding: Padding::from_all(6.),
            margin: Padding::from_all(0.),
            background: None,
            is_shadow: false,
        }
    }

    #[must_use]
    pub fn background(mut self, color: impl Into<Color>) -> Self {
        self.background = Some(color.into());

        self
    }

    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();

        self
    }

    #[inline]
    pub fn total_margin(&self) -> Padding {
        self.padding + self.margin
    }
}

impl<'a, Message> Widget<Message> for Frame<'a, Message>
where
    Message: Clone + 'a
{
    fn ui(
        &mut self,
        ui: &mut Ui,
        shell: &mut Shell<Message>,
    ) -> Response {
        let corner_margin = Padding::from_all(ui.style().corner_radius);

        let index = ui.painter_mut().add(Shapes::None);

        let margin = self.total_margin() + corner_margin;

        let builder = UiBuilder::default()
            .margin(margin);

        let response = ui.child(builder, |ui| {
            self.content.ui(ui, shell);
        }).response;

        let rect = ui.pass().widgets().get(response.id()).unwrap();

        let pos = rect.pos; //  + self.inner_margin + corner_margin;

        let background = self.background.unwrap_or(Color(0));
        let corner = ui.style().corner_radius;
        let border = ui.style().border;
        let is_shadow = self.is_shadow;
        let shadow = ui.style().shadow;

        ui.painter_mut().set(index, move |ui: &mut dyn Renderer| {
            let pos = pos.snap();

            if is_shadow {
                // cheap shadow
                let px = 5.;

                ui.draw_shape(&Shapes::Rectangle(
                    pos.p0() + Point::new(px, px), pos.size(), corner, shadow,
                ))?;
            }
            /*
            ui.draw_shape(&Shapes::Rectangle(
                pos.p0() - Point(1., 1.), pos.size() + Size(2., 2.), corner, border,
            ))?;
            */

            ui.draw_shape(&Shapes::Rectangle(
                pos.p0(), pos.size(), corner, background,
            ))
        });

        response
    }
}

impl<'a, Message> From<Frame<'a, Message>>
    for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(frame: Frame<'a, Message>) -> Self {
        Self::new(frame)
    }
}
