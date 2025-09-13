use essay_graphics_api::{renderer::Renderer, Padding, Point, Shapes, Size};

use crate::{
    style::{Style, UiStyle}, 
    ui::{ui::Props, Response, ResponseValue, Shell, Ui, Widget}, 
    widget2::{Element, SelectableLabel, Text, WidgetFrame}
};

pub fn menu_button<'a, Message>(
    title: impl Into<Text>,
    values: impl IntoIterator<Item = SelectableLabel<'a, Message>>,
) -> MenuButton<'a, Message> {
    MenuButton::new(title, values)
}

pub struct MenuButton<'a, Message> {
    title: Text,
    values: Vec<SelectableLabel<'a, Message>>,
}

impl<'a, Message> MenuButton<'a, Message> {
    pub fn new(
        content: impl Into<Text>,
        values: impl IntoIterator<Item = SelectableLabel<'a, Message>>,
    ) -> Self {
        let content = content.into();

        Self {
            title: content,
            values: values.into_iter().collect(),
        }
    }

    /*
    #[inline]
    pub fn from_button(button: Button) -> Self {
        Self {
            button
        }
    }
    */
}

impl<'a, Message> Widget<Message> for MenuButton<'a, Message>
where
    Message: Clone + 'a
{
    fn ui(
        &mut self,
        ui: &mut Ui,
        shell: &mut Shell<Message>,
    ) -> Response {
        let button_text = ui.theme().button_text.clone();
        let size = ui.text_size(self.title.value(), &button_text);

        let corner = ui.theme().corner_radius;
        let pad = 10.;
        let margin = pad + corner;

        let size = Size::new(size.width + 2. * margin, size.height + 2. * margin);

        let ResponseValue {
            value: bounds,
            response
        } = ui.allocate(size);

        //let bounds = bounds.round_ui();

        let pos = Point::new(bounds.xmin() + margin, bounds.ymin() + margin);

        let inner = bounds - Padding::from_pair(corner, corner);
        // println!("Size {:?} Bounds {:?} {:?}", size, bounds, inner);

        let popup_id = response.id().with("popup");

        if response.clicked(ui) {
            ui.context().memory_mut(|mem| {
                mem.popup_toggle(popup_id)
            });
        }

        let is_press = ui.context().memory_mut(|mem| {
            mem.popup_open(popup_id)
        });

        let is_active = is_press; // self.press ^ press_one;

        let style = if is_active { Style::ButtonOn } else { Style::ButtonOff };

        let (background, foreground) = {
            if response.is_hover(ui) {
                (style.hover_background(ui), style.hover_foreground(ui))
            } else {
                (style.background(ui), style.foreground(ui))
            }
        };
        
        let border = background;
        let corner = style.corner_radius(ui);
        let label = String::from(self.title.value());

        let mut path_style = ui.theme().button.clone();
        path_style.color(background);

        ui.painter().add(move |ui: &mut dyn Renderer| {

            let sz = 0.;
            let r = corner;
            if sz > 0. { // border
                ui.draw_shape(&Shapes::rect(
                    inner.p0() - Point::new(sz, sz), inner.size() + Size::new(2. * sz, 2. * sz), r + 1., 
                    border,
                ))?;
            }

            ui.draw_shape(&Shapes::rect(
                inner.p0(), inner.size(), r, background,
            ))?;
            // ui.draw_path(&background, &style)?;
            path_style.edge_color(foreground);
            path_style.face_color(foreground);
            ui.draw_text(pos, &label, 0., &path_style, &button_text)
        });

        if is_press {
            let rect = response.rect(ui);
            let pos = Point::new(rect.xmin(), rect.ymax());

            ui.popup(popup_id, Props::default().max_bounds(pos), |ui| {
                for child in &mut self.values {
                    child.ui(ui, shell);
                }    
            });
        }

        response
    }
}

impl<'a, Message> From<MenuButton<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(menu: MenuButton<'a, Message>) -> Self {
        Self::new(menu)
    }
}

impl<'a, Message: Clone + 'a> WidgetFrame<'a, Message> for MenuButton<'a, Message> {}