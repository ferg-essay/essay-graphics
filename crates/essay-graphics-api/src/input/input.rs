use std::{collections::HashSet, slice::Iter};

use crate::{renderer::Canvas, Bounds, Point, Size};

use super::Key;

#[derive(Clone)]
pub struct Input {
    pub size: Size,

    pub cursor: Option<Point>,

    pub is_focus: bool,


    pub left_press: bool,
    pub left_release: bool,
    pub left_click: bool,

    key_down: HashSet<Key>,
    events: Vec<Event>,
}

impl Input {
    pub fn cursor_in(&self, bounds: &Bounds<Canvas>) -> bool {
        self.cursor.map_or(false, |pt| bounds.contains(pt))
    }

    pub fn key_pressed(&self, key: Key) -> bool {
        self.events.iter().any(|event| {
            if let Event::KeyPress(event_key) = event {
                *event_key == key
            } else {
                false
            }
        })
    }

    pub fn key_released(&self, key: Key) -> bool {
        self.events.iter().any(|event| {
            if let Event::KeyRelease(event_key) = event {
                *event_key == key
            } else {
                false
            }
        })
    }

    pub fn key_down(&self, key: Key) -> bool {
        self.key_down.contains(&key)
    }

    pub fn update_after_draw(&mut self) {
        self.left_release = false;
        self.left_click = false;
        self.events.drain(..);
    }

    pub fn event(&mut self, event: Event) {
        match event {
            Event::KeyPress(key) => {
                self.key_down.remove(&key);
            },
            Event::KeyRelease(key) => {
                self.key_down.insert(key);
            },
            _ => {}
        }

        self.events.push(event);
    }
    
    pub fn events(&self) -> Iter<Event> {
        self.events.iter()
    }
}

impl Default for Input {
    fn default() -> Self {
        Self { 
            size: Default::default(),

            cursor: Default::default(),
            is_focus: Default::default(),
            left_press: Default::default(),
            left_release: Default::default(),
            left_click: Default::default(),

            key_down: Default::default(),
            events: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Event {
    KeyPress(Key),
    KeyRelease(Key),
    RedrawRequested,
    Resized,
    ScaleFactorChanged(f32),
}

