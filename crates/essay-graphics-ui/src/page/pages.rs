use std::{collections::HashMap, fmt, hash::{Hash, Hasher}};
use essay_graphics_api::renderer::{Drawable, Event, Renderer, Result};

use crate::util::DynLabel;

use super::Page;

pub struct Pages {
    pages: Vec<Page>,

    page_label: HashMap<Box<dyn PageLabel>, usize>,

    current: Option<usize>,
}

impl Pages {
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            page_label: HashMap::new(),
            current: None,
        }
    }

    pub fn add(&mut self, label: impl PageLabel, page: Page) {
        let id = self.pages.len();

        self.pages.push(page);

        self.page_label.insert(label.box_clone(), id);

        if self.current.is_none() {
            self.current = Some(id);
        }
    }

    pub fn activate(&mut self, label: impl PageLabel) {
        if let Some(id) = self.page_label.get(&label.box_clone()) {
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

    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) {
        if let Some(current) = self.current {
            self.pages[current].event(renderer, event);
        }
    }
}

pub trait PageLabel : Send + DynLabel + fmt::Debug {
    fn box_clone(&self) -> Box<dyn PageLabel>;
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
