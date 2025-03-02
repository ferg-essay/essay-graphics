use essay_graphics_api::{PathStyleBase, TextStyle};

pub(crate) struct UiStyle {
    pub label: PathStyleBase,
    pub label_text: TextStyle,
}

impl UiStyle {
    pub(crate) fn new()->Self {
        Self {
            label: PathStyleBase::new(),
            label_text: TextStyle::new(),
        }
    }
}