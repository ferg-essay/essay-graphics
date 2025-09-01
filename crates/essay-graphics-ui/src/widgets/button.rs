use essay_graphics_api::{renderer::{Renderer}, Margin, Point, Shapes, Size};

use crate::ui::{ui::MessageBase, Response, ResponseValue, Shell, Ui, Widget};

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

    pub fn is_press(&self) -> bool {
        self.press
    }

    pub fn set_press(&mut self, is_press: bool) {
        self.press = is_press
    }
}

impl Widget<MessageBase> for Button {
    fn ui(&mut self, ui: &mut Ui, _shell: &mut Shell<MessageBase>) -> Response {
        let button_text = ui.style().button_text.clone();
        let size = ui.text_size(&self.label, &button_text);

        let corner = ui.style().corner_radius;
        let pad = 10.;
        let margin = pad + corner;

        let size = Size::new(size.width + 2. * margin, size.height + 2. * margin);

        let ResponseValue {
            value: bounds,
            response
        } = ui.allocate_rect(size);

        //let bounds = bounds.round_ui();

        let pos = Point::new(bounds.xmin() + margin, bounds.ymin() + margin);

        let inner = bounds - Margin::from_pair(corner, corner);
        // println!("Size {:?} Bounds {:?} {:?}", size, bounds, inner);

        let mut style = ui.style().button.clone();

        let press_one = response.clicked();

        let ui_style = ui.style();

        let (background, foreground) = ui.input(|input| {
            let is_active = self.press ^ press_one;

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
                    inner.p0() - Point::new(sz, sz), inner.size() + Size::new(2. * sz, 2. * sz), r + 1., 
                    border,
                ))?;
            }

            ui.draw_shape(&Shapes::Rectangle(
                inner.p0(), inner.size(), r, background,
            ))?;
            // ui.draw_path(&background, &style)?;
            style.edge_color(foreground);
            style.face_color(foreground);
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
