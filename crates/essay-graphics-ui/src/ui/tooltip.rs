use crate::ui::{context::Response, popup::Popup, ui2::Ui};

pub struct Tooltip {
    popup: Popup,
}

impl Tooltip {
    pub fn for_enabled(response: &Response) -> Self {
        Self {
            popup: Popup::from_response(response),
        }
    }
    
    pub fn show(&self, add_content: impl FnOnce(&mut Ui)) {
        self.popup.show(add_content);
    }
}