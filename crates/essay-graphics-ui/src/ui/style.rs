use essay_graphics_api::{HorizAlign, PathStyle, TextStyle};

#[derive(Clone)]
pub struct UiStyle {
    pub label: PathStyle,
    pub label_text: TextStyle,

    pub button: PathStyle,
    pub button_press: PathStyle,
    pub button_text: TextStyle,
}

impl UiStyle {
    pub(crate) fn new()->Self {
        let mut text = TextStyle::new();
        text.halign(HorizAlign::Left);
        
        Self {
            label: PathStyle::new(),
            label_text: text.clone(),

            button: PathStyle::new(),
            button_press: PathStyle::new(),
            button_text: text.clone(),
        }
    }
}