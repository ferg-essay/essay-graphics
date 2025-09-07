use crate::{ui::{Response, ResponseValue, Ui}, widget2::Text};

pub fn menu_button<'a, Message>(
    title: impl Into<Text>
) -> MenuButton<'a, Message> {
    MenuButton {
        title: title.into(),
        on_press: None,
        press: false,
    }
}

pub struct MenuButton<'a, Message> {
    title: Text,
    on_press: Option<OnPress<'a, Message>>,
    press: bool,
}

impl<'a, Message> MenuButton<'a, Message> {
    pub fn new(
        content: impl Into<Text>,
    ) -> Self {
        let content = content.into();

        Self {
            title: content,
            on_press: None,
            press: false,
        }
    }

    /*
    #[inline]
    pub fn from_button(button: Button) -> Self {
        Self {
            button
        }
    }
    */

    pub fn ui<R>(
        mut self,
        ui: &mut Ui,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> (Response, Option<ResponseValue<R>>) {
        let next_id = ui.next_id().with("popup");
        let press = ui.context().memory(|memory| {
            memory.popup_open(next_id)
        });
        //self.button.set_press(press);

        todo!();
        /*
        let button_response = self.button.ui(ui);

        let popup_response = Popup::menu(ui, &button_response)
            .show(add_content);

        (button_response, popup_response)
        */
    }
}