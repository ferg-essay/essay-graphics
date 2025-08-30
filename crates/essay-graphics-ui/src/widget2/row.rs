use essay_graphics_api::{renderer::Renderer, Margin, Point, Rectangle, Shapes, Size};

use crate::{ui::{Response, ResponseValue, Ui}, widget2::{Element, Shell, Widget, WidgetFrame}};

pub fn row<'a, Message>(
    content: impl IntoIterator<Item=Element<'a, Message>>
) -> Row<'a, Message> {
    Row::new(content)
}

pub struct Row<'a, Message> {
    content: Vec<Element<'a, Message>>,   
}

impl<'a, Message> Row<'a, Message>
{
    pub fn new(
        content: impl IntoIterator<Item=Element<'a, Message>>,
    ) -> Self {
        let content = content.into_iter().collect();

        Self {
            content,
        }
    }
}

impl<'a, Message> Widget<Message> for Row<'a, Message> {
    fn draw(
        &mut self,
        ui: &mut Ui,
        bounds: &Rectangle,
        shell: &mut Shell<Message>,
    ) -> Response {
        ui.row(|ui| {
            for item in &mut self.content {
                item.draw(ui, bounds, shell);
            }
        }).response
    }
}

impl<'a, Message> From<Row<'a, Message>>
    for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(row: Row<'a, Message>) -> Self {
        Self::new(row)
    }
}

impl<'a, M: 'a> WidgetFrame<'a, M> for Row<'a, M> {}
