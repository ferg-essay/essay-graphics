use std::collections::hash_map;

use essay_graphics_api::{input::Input, Rectangle};

use crate::{ui::{Layer, Response, Ui}, util::{Id, IdMap}};

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
    pub layer: Layer,

    pub pos: Rectangle,
}

#[derive(Clone)]
pub struct WidgetRects {
    widgets: Vec<Vec<WidgetPos>>,

    by_id: IdMap<(usize, WidgetPos)>,
}

impl Default for WidgetRects {
    fn default() -> Self {
        let mut widgets = Vec::new();
        widgets.resize_with(Layer::ORDER.len(), Default::default);

        Self { 
            widgets,
            by_id: Default::default() 
        }
    }
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

    pub fn insert(&mut self, id: Id, layer: Layer, pos: impl Into<Rectangle>) -> WidgetPos {
        let widget_pos = WidgetPos { id, layer, pos: pos.into() };

        let Self {
            widgets,
            by_id
        } = self;

        match by_id.entry(id) {
            hash_map::Entry::Vacant(entry) => {
                let index = widgets[layer.index()].len();
                entry.insert((index, widget_pos));
                widgets[layer.index()].push(widget_pos);
            },
            hash_map::Entry::Occupied(mut entry) => {
                let (index, widget) = entry.get_mut();

                let layer = widget.layer;
                widget.pos = widget_pos.pos;
                self.widgets[layer.index()][*index].pos = widget_pos.pos;
            },
        }

        widget_pos
    }

    pub fn iter(&self) -> impl Iterator<Item=&WidgetPos> {
        self.widgets.iter().map(|layer| layer.iter()).flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut WidgetPos> {
        self.widgets.iter_mut().map(|layer| layer.iter_mut()).flatten()
    }

    pub fn clear(&mut self) {
        self.by_id.clear();
        self.widgets.clear();
    }
}