use essay_graphics_api::{renderer::Renderer, Margin, Point, Rectangle, Shapes, Size};

use crate::{ui::{Response, ResponseValue, Ui}, widget2::{Element, Shell, Widget, WidgetFrame}};

pub fn column<'a, Message>(
    content: impl IntoIterator<Item=Element<'a, Message>>
) -> Column<'a, Message> {
    Column::new(content)
}

pub struct Column<'a, Message> {
    content: Vec<Element<'a, Message>>,   
}

impl<'a, Message> Column<'a, Message>
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

impl<'a, Message> Widget<Message>
    for Column<'a, Message>
where
    Message: Clone + 'a
{
    fn draw(
        &mut self,
        ui: &mut Ui,
        bounds: &Rectangle,
        shell: &mut Shell<Message>,
    ) -> Response {
        ui.column(|ui| {
            for item in &mut self.content {
                item.draw(ui, bounds, shell);
            }
        }).response
    }
}

impl<'a, Message> From<Column<'a, Message>>
    for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(column: Column<'a, Message>) -> Self {
        Self::new(column)
    }
}

impl<'a, Message: Clone + 'a> WidgetFrame<'a, Message> for Column<'a, Message> {}
