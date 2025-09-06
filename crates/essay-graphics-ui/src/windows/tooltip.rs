use std::time::{Duration, Instant};

use crate::{ui::{ui::Ui, Response}, windows::Popup};

pub struct Tooltip {
    popup: Popup,
}

impl Tooltip {
    pub fn for_enabled(ui: &mut Ui, response: &Response) -> Self {
        Self {
            popup: Popup::from_response(ui, response)
                .open(Self::should_show_tooltip(ui, response)),
        }
    }
    
    pub fn show(self, ui: &mut Ui, add_content: impl FnOnce(&mut Ui)) {
        self.popup.show(ui, add_content);
    }

    pub fn should_show_tooltip(ui: &mut Ui, response: &Response) -> bool {
        let last_move = ui.render().state.interact.since_cursor_move();

        if last_move > 1. {
            true
        } else {
            let time = Instant::now() + Duration::from_millis(1000 - (last_move * 1000.).ceil() as u64);
            ui.output_mut().redraw_after_delay(time);
            false
        }
    }
}