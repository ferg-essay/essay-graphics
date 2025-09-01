use essay_graphics_api::input::Input;

use crate::ui::{Response, Shell, Ui, Widget};


pub struct Element<'a, Message> {
    widget: Box<dyn Widget<Message> + 'a>,
}

impl<'a, Message> Element<'a, Message> {
    pub fn new(widget: impl Widget<Message> + 'a) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }

    pub fn as_widget(&self) -> &dyn Widget<Message> {
        self.widget.as_ref()
    }

    pub fn as_widget_mut(&mut self) -> &dyn Widget<Message> {
        self.widget.as_mut()
    }
}

impl<'a, Message> Widget<Message> for Element<'a, Message> {
    fn update(
        &mut self,
        input: &Input,
        shell: &mut Shell<'_, Message>,
    ) {
        self.widget.update(input, shell)
    }

    fn ui(
        &mut self,
        ui: &mut Ui,
        shell: &mut Shell<Message>,
    ) -> Response {
        self.widget.ui(ui, shell)
    }
}
