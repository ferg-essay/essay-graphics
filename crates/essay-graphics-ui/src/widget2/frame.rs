use essay_graphics_api::{renderer::Renderer, Color, Margin, Point, Rectangle, Shapes, Size};

use crate::{ui::{ui::UiBuilder, Response, ResponseValue, Ui}, widget2::{Element, Shell, Widget}};

pub fn frame<'a, Message>(
    content: impl Into<Element<'a, Message>>
) -> Frame<'a, Message> {
    Frame::new(content)
}

pub struct Frame<'a, Message> {
    content: Element<'a, Message>,   

    pub inner_margin: Margin,
    pub outer_margin: Margin,

    pub background: Option<Color>,
    pub is_shadow: bool,
}

impl<'a, Message> Frame<'a, Message> {
    pub fn new(
        content: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            content: content.into(),

            inner_margin: Margin::from_all(6.),
            outer_margin: Margin::from_all(0.),
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
    pub fn padding(mut self, padding: impl Into<Margin>) -> Self {
        self.inner_margin = padding.into();

        self
    }

    #[inline]
    pub fn total_margin(&self) -> Margin {
        self.inner_margin + self.outer_margin
    }
}

impl<'a, Message> Widget<Message>
    for Frame<'a, Message>
where
    Message: Clone + 'a
{
    fn draw(
        &mut self,
        ui: &mut Ui,
        bounds: &Rectangle,
        shell: &mut Shell<Message>,
    ) -> Response {
        let corner_margin = Margin::from_all(ui.style().corner_radius);

        let max_bounds = ui.available_bounds() - self.total_margin() - corner_margin;

        // todo: negative bounds
        /*
        assert!(max_bounds.x0() < max_bounds.x1());
        assert!(max_bounds.y0() < max_bounds.y1());
        */

        let index = ui.painter_mut().add(Shapes::None);

        let margin = self.total_margin() + corner_margin;

        let builder = UiBuilder::default()
            .margin(margin);

        // let response = self.content.draw(ui, bounds, shell);

        let response = ui.child(builder, |ui| {
            self.content.draw(ui, bounds, shell);
        }).response;

        let rect = ui.pass().widgets().get(response.id()).unwrap();

        // rect.rect = rect.rect + corner_margin;

        // TODO: force allocation
        /*
        let ResponseValue {
            response,
            ..
        } = ui.alloc_response(rect.rect - self.outer_margin - corner_margin);
        */

        // rect.rect = rect.rect + self.total_margin();

        let pos = rect.rect; //  + self.inner_margin + corner_margin;

        let background = self.background.unwrap_or(Color(0));
        let corner = ui.style().corner_radius;
        let border = ui.style().border;
        let is_shadow = self.is_shadow;
        let shadow = ui.style().shadow;

        ui.painter_mut().set(index, move |ui: &mut dyn Renderer| {
            let pos = pos.round_ui();

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
