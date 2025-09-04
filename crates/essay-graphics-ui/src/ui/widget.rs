use std::collections::hash_map;

use essay_graphics_api::{input::Input, Rectangle};

use crate::{ui::{Response, Ui}, util::{Id, IdMap}};

pub trait Widget<Message> {
    fn ui(&mut self, ui: &mut Ui, shell: &mut Shell<Message>) -> Response;

    #[allow(unused_variables)]
    fn update(
        &mut self,
        input: &Input,
        shell: &mut Shell<'_, Message>,
    ) {
    }
}

///
/// DrawWidget is for non-updating elements such as text, which don't
/// send update messages.
/// 
/// DrawWidgets can be used outside of an application view, as part of the
/// UI frame.
/// 
pub trait DrawWidget {
    fn draw(&mut self, ui: &mut Ui) -> Response;
}

pub struct Shell<'a, Message> {
    messages: &'a mut Vec<Message>,
}

impl<'a, Message> Shell<'a, Message> {
    pub fn new(messages: &'a mut Vec<Message>) -> Self {
        Self {
            messages,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn publish(&mut self, message: Message) {
        self.messages.push(message);
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WidgetPos {
    pub id: Id,

    pub pos: Rectangle,
}

#[derive(Default, Clone)]
pub struct WidgetRects {
    widgets: Vec<WidgetPos>,

    by_id: IdMap<(usize, WidgetPos)>,
}

impl WidgetRects {
    #[inline]
    pub fn get(&self, id: Id) -> Option<&WidgetPos> {
        self.by_id.get(&id).map(|(_, w)| w)
    }

    #[inline]
    pub fn contains(&self, id: Id) -> bool {
        self.by_id.contains_key(&id)
    }

    pub fn insert(&mut self, id: Id, pos: impl Into<Rectangle>) -> WidgetPos {
        let widget_pos = WidgetPos { id, pos: pos.into() };

        let Self {
            widgets,
            by_id
        } = self;

        match by_id.entry(id) {
            hash_map::Entry::Vacant(entry) => {
                let index = widgets.len();
                entry.insert((index, widget_pos));
                widgets.push(widget_pos);
            },
            hash_map::Entry::Occupied(mut entry) => {
                let (index, widget) = entry.get_mut();

                widget.pos = widget_pos.pos;
                self.widgets[*index].pos = widget_pos.pos;
            },
        }

        widget_pos
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item=&WidgetPos> {
        self.widgets.iter()
    }

    pub fn iter_mut(&mut self) -> impl ExactSizeIterator<Item=&mut WidgetPos> {
        self.widgets.iter_mut()
    }

    pub fn clear(&mut self) {
        self.by_id.clear();
        self.widgets.clear();
    }
}