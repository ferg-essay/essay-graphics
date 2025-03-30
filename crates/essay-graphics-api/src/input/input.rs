use crate::{renderer::Canvas, Bounds, Point};

#[derive(Clone)]
pub struct Input {
    pub cursor: Option<Point>,

    pub is_focus: bool,


    pub left_press: bool,
    pub left_release: bool,
    pub left_click: bool,
}
impl Input {
    pub fn cursor_in(&self, bounds: &Bounds<Canvas>) -> bool {
        self.cursor.map_or(false, |pt| bounds.contains(pt))
    }

    pub fn update_after_draw(&mut self) {
        self.left_release = false;
        self.left_click = false;
    }
}

impl Default for Input {
    fn default() -> Self {
        Self { 
            cursor: Default::default(),
            is_focus: Default::default(),
            left_press: Default::default(),
            left_release: Default::default(),
            left_click: Default::default(),
        }
    }
}
