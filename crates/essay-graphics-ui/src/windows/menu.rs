use crate::{ui::{ui::{Ui, Widget}, Response, ResponseValue}, widgets::Button, windows::Popup};

pub fn menu_button<R>(
    ui: &mut Ui,
    title: &str,
    add_contents: impl FnOnce(&mut Ui) -> R
) -> ResponseValue<Option<R>> {
    let parent_id = ui.id();
    let menu_id = parent_id.with(title);

    let button = Button::new(title, false);

    let button_response = ui.add(button);

    ResponseValue::new(None, button_response)
}

pub struct MenuButton {
    pub button: Button,
}

impl MenuButton {
    pub fn new(label: &str) -> Self {
        Self::from_button(Button::new(label, false))
    }

    #[inline]
    pub fn from_button(button: Button) -> Self {
        Self {
            button
        }
    }

    pub fn ui<R>(
        mut self,
        ui: &mut Ui,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> (Response, Option<ResponseValue<R>>) {
        let next_id = ui.next_id().with("popup");
        let press = ui.context().memory(|memory| {
            memory.popup_open(next_id)
        });
        self.button.set_press(press);

        let button_response = self.button.ui(ui);

        let popup_response = Popup::menu(ui, &button_response)
            .show(add_content);

        (button_response, popup_response)
    }
}