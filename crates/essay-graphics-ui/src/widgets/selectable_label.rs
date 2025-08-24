use essay_graphics_api::{renderer::{Canvas, Renderer}, Color, Margin, Path, Point, Shapes, Size};

use crate::{style::State, ui::{ui::Widget, Response, ResponseValue, Ui}};

pub struct SelectableLabel {
    label: String,
    is_selected: bool,
}

impl SelectableLabel {
    pub fn new(label: &str, is_selected: bool) -> Self {
        Self {
            label: String::from(label),
            is_selected,
        }
    }

    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn select(&mut self, is_press: bool) {
        self.is_selected = is_press
    }
}

impl Widget for SelectableLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let text_style = ui.style().button_text.clone();
        let size = ui.text_size(&self.label, &text_style);

        let corner = ui.style().corner_radius;
        let pad = 4.;
        let margin = pad + corner;

        let size = Size(size.0 + 2. * margin, size.1 + 2. * margin);

        let ResponseValue {
            value: bounds,
            response
        } = ui.allocate_rect(size);

        //let bounds = bounds.round_ui();

        let pos = Point(bounds.xmin() + margin, bounds.ymin() + margin);

        let inner = bounds - Margin::from_pair(corner, corner);
        // println!("Size {:?} Bounds {:?} {:?}", size, bounds, inner);

        let mut style = ui.style().button.clone();

        let ui_style = ui.style();

        let (background, foreground) = ui.input(|input| {
            let is_active = self.is_selected;

            if input.cursor
                .map_or(false, |p| bounds.contains(p)) {
                if is_active {
                    (ui_style.button2_on.hover_background, ui_style.button2_on.hover_foreground)
                } else {
                    (ui_style.button2_off.hover_background, ui_style.button2_off.hover_foreground)
                }
            } else {
                if is_active {
                    (ui_style.button2_on.background, ui_style.button2_on.foreground)
                } else {
                    (ui_style.button2_off.background, ui_style.button2_off.foreground)
                }
            }
        });
        
        //style.edge_color(ui.style()[state].edge);
        style.color(background);

        let label = self.label.clone();

        let border = background;
        let corner = ui_style.corner_radius;

        ui.painter_mut().add(move |ui: &mut dyn Renderer| {
            let sz = 0.;
            let r = corner;
            if sz > 0. { // border
                ui.draw_shape(&Shapes::Rectangle(
                    inner.p0() - Point(sz, sz), inner.size() + Size(2. * sz, 2. * sz), r + 1., 
                    border,
                ))?;
            }

            ui.draw_shape(&Shapes::Rectangle(
                inner.p0(), inner.size(), r, background,
            ))?;
            // ui.draw_path(&background, &style)?;
            style.edge_color(foreground);
            style.face_color(foreground);
            ui.draw_text(pos, &label, 0., &style, &text_style)
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
