use std::{any::{Any, TypeId}};

use crate::util::{Id, IdMap};

pub struct Memory {
    popup: Option<OpenPopup>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {  
            popup: Default::default()
        }
    }
}

impl Memory {
    pub fn popup_open(&self, id: Id) -> bool {
        self.popup.as_ref().is_some_and(|popup| popup.id == id)
    }

    pub fn popup_toggle(&mut self, id: Id) {
        match &mut self.popup {
            Some(popup) => {
                if popup.id == id {
                    self.popup = None;
                } else {
                    self.popup = Some(OpenPopup::new(id));
                }
            }
            None => {
                self.popup = Some(OpenPopup::new(id));
            }
        }
    }
}

struct OpenPopup {
    id: Id,
}

impl OpenPopup {
    fn new(id: Id) -> Self {
        Self {
            id,
        }
    }
}

#[derive(Default)]
pub struct MemoryData {
    map: IdMap<Item>,
}

impl MemoryData {
    pub fn insert<T: MemoryItem + 'static>(&mut self, id: Id, value: T) {
        let id_type = id.with(TypeId::of::<T>());

        self.map.insert(id_type, Item::new(value));
    }

    pub fn get<T: MemoryItem>(&self, id: Id) -> Option<&T> {
        let id_type = id.with(TypeId::of::<T>());

        self.map.get(&id_type).map(|item| item.deref())
    }

    pub fn get_mut<T: MemoryItem>(&mut self, id: Id) -> Option<&mut T> {
        let id_type = id.with(TypeId::of::<T>());

        self.map.get_mut(&id_type).map(|item| item.deref_mut())
    }
}

struct Item {
    value: Box<dyn Any + Send + Sync>,
}

impl Item {
    fn new<T: MemoryItem>(value: T) -> Self {
        Self {
            value: Box::new(value),
        }

    }

    fn deref<T: MemoryItem>(&self) -> &T {
        self.value.downcast_ref().expect("Unexpected type for deref")
    }

    fn deref_mut<T: MemoryItem>(&mut self) -> &mut T {
        self.value.downcast_mut().expect("Unexpected type for deref_mut")
    }
} 

pub trait MemoryItem: Send + Sync + 'static {
}
