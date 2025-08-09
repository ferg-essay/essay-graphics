use essay_graphics_api::{renderer::Canvas, Bounds, HorizAlign, Path, PathStyle, Point, Size};

use crate::ui::Ui;

use super::style::State;

pub struct Tabs<'a, T: PartialEq + Into<String>> {
    select: T,
    items: Vec<Item<'a, T>>,
}

impl<'a, T: PartialEq + Into<String>> Tabs<'a, T> {
    pub fn new(value: T) -> Self {
        Self {
            select: value,
            items: Vec::new(),
        }
    }

    pub fn item(&mut self, key: T, add_content: impl FnOnce(&mut Ui) + 'a) {
        self.items.push({Item {
            key: key,
            add_content: Some(Box::new(add_content)),
        }});
    }
}

impl<T: PartialEq + Clone + Into<String>> Tabs<'_, T> {
    pub fn show(mut self, ui: &mut Ui) -> Option<T> {
        let style_text = ui.style().label_text.clone();
        let margin = 10.;

        let style = PathStyle::new();
        let mut tab_style = PathStyle::new();
        let mut add_content: Option<Box<dyn FnOnce(&mut Ui)>> = None;
        let mut selected: Option<T> = Some(self.select.clone());

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let remaining_size = ui.remaining_size();
                let text_size = ui.text_size("M", &style_text);
                let text_size = Size(text_size.0 + 2. * margin, text_size.1 + 2. * margin);

                let size = Size(remaining_size.width(), text_size.height());
                let pos = ui.allocate_rect(size);

                let tab_width = pos.width() / self.items.len().max(1) as f32;

                let mut tabs = Vec::<Bounds<Canvas>>::new();

                for (i, item) in self.items.iter().enumerate() {
                    let pos = Bounds::from((
                        [pos.xmin() + i as f32 * tab_width, pos.ymin()],
                        [tab_width, pos.height()]
                    ));

                    if ui.input().left.click && ui.input().cursor_in(&pos) {
                        selected = Some(item.key.clone());
                    }

                    tabs.push(pos);
                }

                for (mut item, pos) in self.items.drain(..).zip(tabs) {
                    let is_selected = selected.as_ref().map_or(false, |key| key.clone() == item.key);
                    
                    let label: String = item.label();

                    if is_selected {
                        tab_style.edge_color(ui.style()[State::Active].edge);
                        tab_style.face_color(ui.style()[State::Active].background);

                        let path = Path::move_to(pos.xmin(), pos.ymin())
                            .line_to(pos.xmin(), pos.ymax())
                            .to_path();

                        ui.renderer().draw_path(&path, &tab_style).unwrap();

                        let path = Path::move_to(pos.xmax(), pos.ymin())
                            .line_to(pos.xmax(), pos.ymax())
                            .to_path();
                        
                        ui.renderer().draw_path(&path, &tab_style).unwrap();

                        let path = Path::move_to(pos.xmin(), pos.ymax())
                            .line_to(pos.xmax(), pos.ymax())
                            .to_path();
                        
                        tab_style.edge_color(ui.style()[State::Active].accent);
                        let mut style = tab_style.clone();
                        style.line_width(3.);
                        ui.renderer().draw_path(&path, &style).unwrap();

                        add_content = item.add_content.take();
                        selected = Some(item.key);
                    } else if ui.input().cursor_in(&pos) {
                        tab_style.edge_color(ui.style()[State::Hover].edge);
                        tab_style.face_color(ui.style()[State::Hover].background);

                        let path = Path::from(pos);
                        ui.renderer().draw_path(&path, &tab_style).unwrap();
                    } else {
                        tab_style.edge_color(ui.style()[State::Inactive].edge);
                        tab_style.face_color(ui.style()[State::Inactive].background);

                        let path = Path::from(pos);
                        ui.renderer().draw_path(&path, &tab_style).unwrap();
                    }
            
                    let pos = pos.with_margin(margin);
                    let mut style_text = style_text.clone();
                    style_text.halign(HorizAlign::Center);
                    
                    ui.renderer().draw_text(
                        Point(pos.xmid(), pos.ymin()),
                        &label, 
                        0., 
                        &style,
                        &style_text
                    ).unwrap();
                }
            });

            if let Some(add_content) = add_content.take() {
                (add_content)(ui);
            }
        });

        selected
    }
}

struct Item<'a, T> {
    key: T,
    add_content: Option<Box<dyn FnOnce(&mut Ui) + 'a>>,
}

impl<'a, T: Clone + Into<String>> Item<'a, T> {
    fn label(&self) -> String {
        self.key.clone().into()
    }
}