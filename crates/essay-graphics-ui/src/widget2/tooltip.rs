use crate::{ui::{Response, Shell, Ui, Widget}, widget2::{Element, WidgetFrame}};

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

impl<'a, Message> Widget<Message> for Tooltip<'a, Message>
where
    Message: Clone + 'a
{
    fn ui(
        &mut self,
        ui: &mut Ui,
        shell: &mut Shell<Message>,
    ) -> Response {
        let response = self.content.ui(ui, shell);

        response.on_hover_ui(ui, |ui| {
            self.tooltip.ui(ui, shell);
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
