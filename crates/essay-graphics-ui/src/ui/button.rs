use essay_graphics_api::{renderer::{Canvas, Renderer}, Path, Point, Size};

use crate::ui::{context, ui::Response, ui2::{ResponseValue, Ui2, Widget2}, Ui};

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

        let press_one = ui.input().left.click && ui.input().cursor_in(&bounds);

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

pub(crate) struct Button2 {
    label: String,
    press: bool,
}

impl Button2 {
    pub(crate) fn new(label: &str, press: bool) -> Self {
        Self {
            label: String::from(label),
            press,
        }
    }
}

impl Widget2 for Button2 {
    fn ui(&mut self, ui: &mut Ui2) -> context::Response {
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

        /*
        let press_one = ui.input().left.click && ui.input().cursor_in(&bounds);

        let state = if ui.input().cursor
            .map_or(false, |p| bounds.contains(p)) {
            State::Hover
        } else if self.press ^ press_one { 
            State::Active
        } else {
            State::Inactive
        };
        */
        let state = State::Inactive;

        style.edge_color(ui.style()[state].edge);
        style.color(ui.style()[state].background);

        let label = self.label.clone();

        let text_color = if false { // if self.press ^ press_one { 
            ui.style()[State::Active].foreground
        } else {
            ui.style()[State::Inactive].foreground
        };

        ui.painter_mut().add(move |ui: &mut dyn Renderer| {
            ui.draw_path(&background, &style)?;
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
