use essay_graphics_api::{renderer::{Canvas, Renderer}, Color, Path, Point, Shapes, Size};

use crate::{style::State, ui::{ui::Widget, Response, ResponseValue, Ui}};

pub struct Button {
    label: String,
    press: bool,
}

impl Button {
    pub fn new(label: &str, press: bool) -> Self {
        Self {
            label: String::from(label),
            press,
        }
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> Response {
        let button_text = ui.style().button_text.clone();
        let size = ui.text_size(&self.label, &button_text);

        let margin = 10.;
        let size = Size(size.0 + 2. * margin, size.1 + 2. * margin);

        let ResponseValue {
            value: bounds,
            response
        } = ui.allocate_rect(size);

        let bounds = bounds.round_ui();

        let pos = Point(bounds.xmin() + margin, bounds.ymin() + margin);

        let mut style = ui.style().button.clone();

        let background = Path::<Canvas>::from(bounds);

        //let press_one = ui.input().left.click && ui.input().cursor_in(&bounds);

        let press_one = response.clicked();

        let state = ui.input(|input| {
            if input.cursor
                .map_or(false, |p| bounds.contains(p)) {
                State::Hover
            } else if self.press ^ press_one { 
                State::Active
            } else {
                State::Inactive
            }
        });

        style.edge_color(ui.style()[state].edge);
        style.color(ui.style()[state].background);

        let label = self.label.clone();

        let text_color = if self.press ^ press_one { 
            ui.style()[State::Active].foreground
        } else {
            ui.style()[State::Inactive].foreground
        };

        let background_color = ui.style()[state].background;

        ui.painter_mut().add(move |ui: &mut dyn Renderer| {
            let sz = 2.;
            let r = 10.;
            ui.draw_shape(&Shapes::Rectangle(
                bounds.p0() - Point(sz, sz), bounds.size() + Size(2. * sz, 2. * sz), r + 1., 
                Color(0x404040ff)
            ))?;

            ui.draw_shape(&Shapes::Rectangle(
                bounds.p0(), bounds.size(), r, background_color,
            ))?;
            // ui.draw_path(&background, &style)?;
            style.edge_color(text_color);
            style.face_color(text_color);
            ui.draw_text(pos, &label, 0., &style, &button_text)
        });

        /*
        if self.press ^ press_one { 
            style.edge_color(ui.style()[State::Active].foreground);
            style.face_color(ui.style()[State::Active].foreground);
        } else {
            style.edge_color(ui.style()[State::Inactive].foreground);
            style.face_color(ui.style()[State::Inactive].foreground);
        }
        */

        //ui.painter_mut().add(|ui: &mut dyn Renderer| {
        //    ui.draw_text(pos, &self.label, 0., &style, &button_text)
        //});

        //ui.renderer().draw_text(pos, &self.label, 0., &style, &button_text).unwrap();

        //Response::default().with_onclick(press_one)
        response
    }
}
