use essay_graphics_api::{color::Grey, renderer::{Canvas, Renderer}, Bounds, Padding, Shapes, Size};

use crate::ui::{ui::MessageBase, DrawWidget, Response, ResponseValue, Shell, Ui, Widget};

pub struct Radio {
    label: String,
    is_selected: bool,
}

impl Radio {
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

impl DrawWidget for Radio {
    fn draw(&mut self, ui: &mut Ui) -> Response {
        let text_style = ui.style().button_text.clone();
        let size = ui.text_size(&self.label, &text_style);

        let height = size.height;
        let pad = 10.;

        let size = Size::new(size.width + pad + height, size.height);

        let ResponseValue {
            value: _bounds,
            response
        } = ui.allocate(size);

        let pos = response.rect(ui);

        let ui_style = ui.style();

        let (background, foreground) = {
            let is_active = self.is_selected;

            if is_active {
                (ui_style.button2_on.background, ui_style.button2_on.foreground)
            } else {
                (Grey(0.90).into(), Grey(0.90).into())
            }
        };
        
        let label = self.label.clone();
        let style = ui.style().button.clone();

        let is_active = self.is_selected;

        let radio_pos = Bounds::<Canvas>::from([
            [pos.xmax() - height, pos.ymin()],
            [pos.xmax(), pos.ymax()],
        ]);

        // style.color(foreground);
        ui.painter().add(move |ui: &mut dyn Renderer| {
            ui.draw_text(pos.p0(), &label, 0., &style, &text_style)?;

            let pos_center = radio_pos - Padding::from_all(6.);

            ui.draw_shape(&Shapes::Rectangle(
                radio_pos.p0(),
                radio_pos.size(),
                radio_pos.height() * 0.5,
                background,
            ))?;

            if is_active {
                ui.draw_shape(&Shapes::Rectangle(
                    pos_center.p0(),
                    pos_center.size(),
                    pos_center.height() * 0.5,
                    foreground,
                ))?;
            }

            Ok(())
        });

        response
    }
}
