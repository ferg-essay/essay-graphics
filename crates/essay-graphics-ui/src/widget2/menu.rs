use essay_graphics_api::{renderer::Renderer, Padding, Point, Shapes, Size};

use crate::{style::UiStyle, ui::{ui::UiBuilder, Response, ResponseValue, Shell, Ui, Widget}, widget2::{Element, OffStyle, OnStyle, SelectableLabel, Text, WidgetFrame}};

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

        let mut style = ui.theme().button.clone();

        if response.clicked(ui) {
            ui.context().memory_mut(|mem| {
                mem.popup_toggle(ui.stable_id())
            });
            /*
            match &self.on_press {
                Some(OnPress::Direct(message)) => {
                    shell.publish(message.clone());
                },
                Some(OnPress::Closure(fun)) => {
                    shell.publish(fun());
                },
                None => {}
            }
            */
        }

        let is_press = ui.context().memory_mut(|mem| {
            mem.popup_open(ui.stable_id())
        });

        // let ui_style = ui.theme();

        let (background, foreground) = {
            let is_active = is_press; // self.press ^ press_one;

            if response.is_hover(ui) {
                if is_active {
                    (OnStyle.hover_background(ui), OnStyle.hover_foreground(ui))
                } else {
                    (OffStyle.hover_background(ui), OffStyle.hover_foreground(ui))
                }
            } else {
                if is_active {
                    (OnStyle.background(ui), OnStyle.foreground(ui))
                } else {
                    (OffStyle.background(ui), OffStyle.foreground(ui))
                }
            }
        };
        
        //style.edge_color(ui.style()[state].edge);
        style.color(background);

        //let label = self.label.clone();

        let border = background;
        let corner = OnStyle.corner_radius(ui);
        let label = String::from(self.title.value());

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
            style.edge_color(foreground);
            style.face_color(foreground);
            ui.draw_text(pos, &label, 0., &style, &button_text)
        });

        if is_press {
            let id = ui.stable_id().with("popup");

            let rect = response.rect(ui);
            let pos = Point::new(rect.xmin(), rect.ymax());

            ui.popup(id, UiBuilder::default().max_bounds(pos), |ui| {
                for child in &mut self.values {
                    child.ui(ui, shell);
                }    
            });
        }
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

impl<'a, Message> From<MenuButton<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(menu: MenuButton<'a, Message>) -> Self {
        Self::new(menu)
    }
}

impl<'a, Message: Clone + 'a> WidgetFrame<'a, Message> for MenuButton<'a, Message> {}