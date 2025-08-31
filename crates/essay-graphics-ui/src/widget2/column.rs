use essay_graphics_api::{Rectangle};

use crate::{
    ui::{Response, Ui}, 
    widget2::{Element, Shell, Widget, WidgetFrame}
};

pub fn column<'a, Message>(
    children: impl IntoIterator<Item=Element<'a, Message>>
) -> Column<'a, Message> {
    Column::with_children(children)
}

#[macro_export]
macro_rules! column {
    () => (
        $crate::widget2::Column::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::widget2::Column::with_children([$($crate::widget2::Element::from($x)),+])
    )
}

pub struct Column<'a, Message> {
    children: Vec<Element<'a, Message>>,   
}

impl<'a, Message> Column<'a, Message> {
    pub fn new() -> Self {
        Self {
            children: Default::default(),
        }
    }

    pub fn with_children(
        children: impl IntoIterator<Item=Element<'a, Message>>,
    ) -> Self {
        Self {
            children: children.into_iter().collect(),
        }
    }

    #[must_use]
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());

        self
    }

    #[must_use]
    pub fn extend(
        self,
        children: impl IntoIterator<Item=Element<'a, Message>>,
    ) -> Self {
        children.into_iter().fold(self, Self::push)
    }
}

impl<'a, Message> Widget<Message> for Column<'a, Message>
where
    Message: 'a
{
    fn draw(
        &mut self,
        ui: &mut Ui,
        shell: &mut Shell<Message>,
    ) -> Response {
        ui.column(|ui| {
            for item in &mut self.children {
                item.draw(ui, shell);
            }
        }).response
    }
}

impl<'a, Message> From<Column<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(column: Column<'a, Message>) -> Self {
        Self::new(column)
    }
}

impl<'a, Message: Clone + 'a> WidgetFrame<'a, Message> for Column<'a, Message> {}
