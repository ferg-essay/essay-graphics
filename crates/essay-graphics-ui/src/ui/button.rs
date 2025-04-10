use essay_graphics_api::{renderer::Canvas, Path, Point, Size};

use crate::ui::{ui::Response, Ui};

use super::{style::State, ui::Widget};

pub(crate) struct Button {
    label: String,
    press: bool,
}

impl Button {
    pub(crate) fn new(label: &str, press: bool) -> Self {
        Self {
            label: String::from(label),
            press,
        }
    }
}

impl Widget for Button {
    fn ui(&mut self, ui: &mut Ui) -> Response {
        let button_text = ui.style().button_text.clone();
        let size = ui.text_size(&self.label, &button_text);

        let margin = 10.;
        let size = Size(size.0 + 2. * margin, size.1 + 2. * margin);

        let bounds = ui.allocate_rect(size);
        let pos = Point(bounds.xmin() + margin, bounds.ymin() + margin);

        let mut style = ui.style().button.clone();

        let background = Path::<Canvas>::from(bounds);
        //let border = Path::<Canvas>::from(bounds.with_margin(m2));

        //let press = ui.input().left_press && ui.input().cursor_in(&bounds);
        let press_one = ui.input().left_click && ui.input().cursor_in(&bounds);

        let state = if ui.input().cursor
            .map_or(false, |p| bounds.contains(p)) {
            State::Hover
        } else if self.press ^ press_one { 
            State::Active
        } else {
            State::Inactive
        };

        style.edge_color(ui.style()[state].edge);
        style.color(ui.style()[state].background);

        ui.renderer().draw_path(&background, &style).unwrap();

        if self.press ^ press_one { 
            style.edge_color(ui.style()[State::Active].foreground);
            style.face_color(ui.style()[State::Active].foreground);
        } else {
            style.edge_color(ui.style()[State::Inactive].foreground);
            style.face_color(ui.style()[State::Inactive].foreground);
        }

        ui.renderer().draw_text(pos, &self.label, 0., &style, &button_text).unwrap();

        Response::default().with_onclick(press_one)
    }
}
