use crate::{ui::{Response, Ui}, widget2::{Element, Shell, Widget, WidgetFrame}};

pub fn row<'a, Message>(
    content: impl IntoIterator<Item=Element<'a, Message>>
) -> Row<'a, Message> {
    Row::new(content)
}

#[macro_export]
macro_rules! row {
    () => (
        $crate::widget2::Row::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::widget2::Row::with_children([$($crate::widget2::Element::from($x)),+])
    )
}

pub struct Row<'a, Message> {
    children: Vec<Element<'a, Message>>,   
}

impl<'a, Message> Row<'a, Message> {
    pub fn new(
        children: impl IntoIterator<Item=Element<'a, Message>>,
    ) -> Self {
        Self {
            children: children.into_iter().collect(),
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

impl<'a, Message> Widget<Message> for Row<'a, Message> {
    fn draw(
        &mut self,
        ui: &mut Ui,
        shell: &mut Shell<Message>,
    ) -> Response {
        ui.row(|ui| {
            for item in &mut self.children {
                item.draw(ui, shell);
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
