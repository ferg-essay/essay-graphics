use crate::widget2::widget::Widget;


pub struct Element<'a, Message, Theme, Renderer> {
    widget: Box<dyn Widget<Message, Theme, Renderer> + 'a>,
}

impl<'a, Message, Theme, Renderer> Element<'a, Message, Theme, Renderer> {
    pub fn new(widget: impl Widget<Message, Theme, Renderer> + 'a) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }

    pub fn as_widget(&self) -> &dyn Widget<Message, Theme, Renderer> {
        self.widget.as_ref()
    }

    pub fn as_widget_mut(&mut self) -> &dyn Widget<Message, Theme, Renderer> {
        self.widget.as_mut()
    }
}