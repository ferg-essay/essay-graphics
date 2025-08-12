use std::collections::hash_map;

use essay_graphics_api::{renderer::Canvas, Bounds};

use crate::ui::{Id, IdMap};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WidgetRect {
    pub id: Id,

    pub rect: Bounds<Canvas>,
}

#[derive(Default, Clone)]
pub struct WidgetRects {
    widgets: Vec<WidgetRect>,

    by_id: IdMap<(usize, WidgetRect)>,
}

impl WidgetRects {
    #[inline]
    pub fn get(&self, id: Id) -> Option<&WidgetRect> {
        self.by_id.get(&id).map(|(_, w)| w)
    }

    #[inline]
    pub fn contains(&self, id: Id) -> bool {
        self.by_id.contains_key(&id)
    }

    pub fn insert(&mut self, widget_rect: WidgetRect) {
        let Self {
            widgets,
            by_id
        } = self;

        match by_id.entry(widget_rect.id) {
            hash_map::Entry::Vacant(entry) => {
                let index = widgets.len();
                entry.insert((index, widget_rect));
                widgets.push(widget_rect);
            },
            hash_map::Entry::Occupied(mut entry) => {
                let (_index, widget) = entry.get_mut();

                widget.rect = widget_rect.rect;
            },
        }
    }

    pub fn clear(&mut self) {
        self.by_id.clear();
        self.widgets.clear();
    }
}