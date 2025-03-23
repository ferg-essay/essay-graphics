use essay_graphics_api::{HorizAlign, PathStyleBase, TextStyle};

pub struct UiStyle {
    pub label: PathStyleBase,
    pub label_text: TextStyle,

    pub button: PathStyleBase,
    pub button_press: PathStyleBase,
    pub button_text: TextStyle,
}

impl UiStyle {
    pub(crate) fn new()->Self {
        let mut text = TextStyle::new();
        text.halign(HorizAlign::Left);
        
        Self {
            label: PathStyleBase::new(),
            label_text: text.clone(),

            button: PathStyleBase::new(),
            button_press: PathStyleBase::new(),
            button_text: text.clone(),
        }
    }
}