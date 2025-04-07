use std::{collections::HashMap, fmt, hash::{Hash, Hasher}};
use essay_graphics_api::renderer::{Drawable, Renderer, Result};

use crate::util::DynLabel;

use super::Page;

pub struct Pages {
    pages: Vec<Page>,

    _page_label: HashMap<Box<dyn PageLabel>, usize>,

    current: Option<usize>,
}

impl Pages {
    pub fn _new() -> Self {
        Self {
            pages: Vec::new(),
            _page_label: HashMap::new(),
            current: None,
        }
    }

    pub fn _add(&mut self, label: impl PageLabel, page: Page) {
        let id = self.pages.len();

        self.pages.push(page);

        self._page_label.insert(label._box_clone(), id);

        if self.current.is_none() {
            self.current = Some(id);
        }
    }

    pub fn _activate(&mut self, label: impl PageLabel) {
        if let Some(id) = self._page_label.get(&label._box_clone()) {
            self.current = Some(*id);
        } else {
            self.current = None;
        }
    }
}

impl Drawable for Pages {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        if let Some(current) = self.current {
            self.pages[current].draw(renderer)
        } else {
            Ok(())
        }
    }
}

pub trait PageLabel : Send + DynLabel + fmt::Debug {
    fn _box_clone(&self) -> Box<dyn PageLabel>;
}

impl PartialEq for dyn PageLabel {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_dyn_eq())
    }
}

impl Eq for dyn PageLabel {}

impl Hash for dyn PageLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state);
    }
}

impl AsRef<dyn PageLabel> for dyn PageLabel {
    fn as_ref(&self) -> &dyn PageLabel {
        self
    }
}
