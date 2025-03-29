use crate::Point;

#[derive(Clone)]
pub struct Input {
    pub cursor: Option<Point>,

    pub is_focus: bool,


    pub left_press: bool,
    pub left_release: bool,
}

impl Default for Input {
    fn default() -> Self {
        Self { 
            cursor: Default::default(),
            is_focus: Default::default(),
            left_press: Default::default(),
            left_release: Default::default(),
        }
    }
}
