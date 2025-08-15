use crate::ui::{Response, popup::Popup, ui2::Ui};

pub struct Tooltip {
    popup: Popup,
}

impl Tooltip {
    pub fn for_enabled(response: &Response) -> Self {
        Self {
            popup: Popup::from_response(response)
                .open(Self::should_show_tooltip(response)),
        }
    }
    
    pub fn show(self, add_content: impl FnOnce(&mut Ui)) {
        self.popup.show(add_content);
    }

    pub fn should_show_tooltip(response: &Response) -> bool {
        response.ctx.viewport(|viewport| {
            let last_move = viewport.interact.since_cursor_move();

            if last_move > 1. {
                true
            } else {
                response.ctx.request_redraw_when(1. - last_move);
                false
            }
        })
    }
}