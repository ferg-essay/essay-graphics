use essay_graphics_api::{renderer::Renderer, Margin, Point, Rectangle, Shapes, Size};

use crate::{ui::{Response, ResponseValue, Ui}, widget2::{Element, Shell, Widget, WidgetFrame}};

pub fn tooltip<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    tooltip: impl Into<Element<'a, Message>>,
) -> Tooltip<'a, Message> {
    Tooltip::new(content, tooltip)
}

pub struct Tooltip<'a, Message> {
    content: Element<'a, Message>,   
    tooltip: Element<'a, Message>,   
}

impl<'a, Message> Tooltip<'a, Message> {
    pub fn new(
        content: impl Into<Element<'a, Message>>,
        tooltip: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            content: content.into(),
            tooltip: tooltip.into(),
        }
    }
}

impl<'a, Message> Widget<Message>
    for Tooltip<'a, Message>
where
    Message: Clone + 'a
{
    fn draw(
        &mut self,
        ui: &mut Ui,
        bounds: &Rectangle,
        shell: &mut Shell<Message>,
    ) -> Response {
        let response = self.content.draw(ui, bounds, shell);

        response.on_hover_ui(|ui| {
            self.tooltip.draw(ui, bounds, shell);
        });

        response
    }
}

impl<'a, Message> From<Tooltip<'a, Message>>
    for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(column: Tooltip<'a, Message>) -> Self {
        Self::new(column)
    }
}

impl<'a, Message: Clone + 'a> WidgetFrame<'a, Message> for Tooltip<'a, Message> {}
