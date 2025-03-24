use essay_graphics_api::{renderer::{self, Canvas}, Color, Path, Point, Size};

use crate::Ui;

use super::ui::Widget;

pub(crate) struct UiButton {
    label: String,
    _press: bool,
}

impl UiButton {
    pub(crate) fn new(label: &str) -> Self {
        Self {
            label: String::from(label),
            _press: false,
        }
    }
}

impl Widget for UiButton {
    fn ui(&mut self, ui: &mut Ui) -> renderer::Result<()> {
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

        let over = ui.input().cursor.map_or(false, |p| bounds.contains(p));
        let press = ui.input().left_press.map_or(false, |p| bounds.contains(p));

        let mut style = ui.style().button.clone();
        style.face_color(Color::none());
        if over {
            style.edge_color("red");
        } else {
            style.edge_color("black");
        }

        ui.renderer().draw_path(&border, &style).unwrap();

        if press { 
            let button_press = ui.style().button_press.clone();
            ui.renderer().draw_text(pos, &self.label, 0., &button_press, &button_text)
        } else {
            let button = ui.style().button.clone();

            ui.renderer().draw_text(pos, &self.label, 0., &button, &button_text)
        }
    }
}
