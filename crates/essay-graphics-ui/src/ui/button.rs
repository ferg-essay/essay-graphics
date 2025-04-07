use essay_graphics_api::{renderer::Canvas, Color, Path, Point, Size};

use crate::ui::{ui::Response, Ui};

use super::ui::Widget;

pub(crate) struct UiButton {
    label: String,
    press: bool,
}

impl UiButton {
    pub(crate) fn new(label: &str, press: bool) -> Self {
        Self {
            label: String::from(label),
            press,
        }
    }
}

impl Widget for UiButton {
    fn ui(&mut self, ui: &mut Ui) -> Response {
        let button_text = ui.style().button_text.clone();
        let size = ui.text_size(&self.label, &button_text);

        let margin = 10.;
        let size = Size(size.0 + 2. * margin, size.1 + 2. * margin);

        let bounds = ui.allocate_rect(size);
        let pos = Point(bounds.xmin() + margin, bounds.ymin() + margin);
        let m2 = margin * 0.5;

        let border = Path::<Canvas>::move_to(bounds.xmin() + m2, bounds.ymin() + m2)
            .line_to(bounds.xmax() - m2, bounds.ymin() + m2)
            .line_to(bounds.xmax() - m2, bounds.ymax() - m2)
            .close_poly(bounds.xmin() + m2, bounds.ymax() - m2)
            .to_path();

        let mut style = ui.style().button.clone();

        if ui.input().cursor.map_or(false, |p| bounds.contains(p)) {
            style.color(Color(0xf0f0f0ff));
            ui.renderer().draw_path(&border, &style).unwrap();
        }

        let press = ui.input().left_press && ui.input().cursor_in(&bounds);
        let press_one = ui.input().left_click && ui.input().cursor_in(&bounds);
        
        let mut style = ui.style().button.clone();
        style.face_color(Color::none());

        if press {
            style.edge_color(Color::from("red").with_alpha(0.25));
        }

        ui.renderer().draw_path(&border, &style).unwrap();

        if self.press ^ press_one { 
            let button_press = ui.style().button_press.clone();
            ui.renderer().draw_text(pos, &self.label, 0., &button_press, &button_text).unwrap();
        } else {
            let button = ui.style().button.clone();

            ui.renderer().draw_text(pos, &self.label, 0., &button, &button_text).unwrap();
        }

        Response::default().with_onclick(press_one)
    }
}
