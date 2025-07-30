use essay_graphics_api::{input::{Event, Input, Key}, Point, Size};
use winit::{
    event::{self, ElementState, MouseButton, WindowEvent},
    keyboard::{self, NamedKey},
};

pub fn input_event(input: &mut Input, event: &event::Event<()>) -> bool {
    let mut is_update = true;

    match event {
        event::Event::WindowEvent {
            event: WindowEvent::Resized(new_size),
            ..
        } => {
            input.size = Size(new_size.width as f32, new_size.height as f32);
        }
        event::Event::WindowEvent {
            event: WindowEvent::ScaleFactorChanged { scale_factor, .. },
            ..
        } => {
            input.event(Event::ScaleFactorChanged(*scale_factor as f32));
        }
        event::Event::WindowEvent {
            event: WindowEvent::MouseInput { state, button, .. },
            ..
        } => {
            mouse_input(input, state, button);
        }
        event::Event::WindowEvent {
            event: WindowEvent::CursorMoved { position, .. },
            ..
        } => {
            let pos = Point(position.x as f32, input.size.height() - position.y as f32);

            input.cursor = Some(pos);
        }
        event::Event::WindowEvent {
            event: WindowEvent::KeyboardInput { event, .. },
            ..
        } => {
            if let Some(key) = map_key(&event.logical_key) {
                if event.state == ElementState::Pressed {
                    input.event(Event::KeyPress(key));
                } else if event.state == ElementState::Released {
                    input.event(Event::KeyRelease(key));
                }
            }
        }
        event::Event::WindowEvent {
            event: WindowEvent::CursorEntered { .. },
            ..
        } => {}
        event::Event::WindowEvent {
            event: WindowEvent::CursorLeft { .. },
            ..
        } => {}
        event::Event::WindowEvent {
            event: WindowEvent::Focused(is_focus),
            ..
        } => {
            input.is_focus = *is_focus;

            if !is_focus {
                input.cursor = None;
            }
        }
        event::Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            input.event(Event::RedrawRequested);
        },
        _ => {
            is_update = false;
        }
    }

    is_update
}

fn mouse_input(input: &mut Input, state: &ElementState, button: &MouseButton) {
    match button {
        MouseButton::Left => match state {
            ElementState::Pressed => {
                input.left_press = true;
                input.left_click = true;
            }
            ElementState::Released => {
                input.left_release = true;
                input.left_press = false;
            }
        },
        _ => {}
    }
}

fn map_key(key: &keyboard::Key) -> Option<Key> {
    match key {
        keyboard::Key::Named(named_key) => match named_key {
            NamedKey::Space => Some(Key::Space),
            _ => None,
        },
        keyboard::Key::Character(key) => match key.as_str() {
            " " => Some(Key::Space),

            "a" => Some(Key::A),
            "b" => Some(Key::B),
            "c" => Some(Key::C),
            "d" => Some(Key::D),
            "e" => Some(Key::E),
            "f" => Some(Key::F),
            "g" => Some(Key::G),
            "h" => Some(Key::H),
            "i" => Some(Key::I),
            "j" => Some(Key::J),
            "k" => Some(Key::K),
            "l" => Some(Key::L),
            "m" => Some(Key::M),
            "n" => Some(Key::N),
            "o" => Some(Key::O),
            "p" => Some(Key::P),
            "q" => Some(Key::Q),
            "r" => Some(Key::R),
            "s" => Some(Key::S),
            "t" => Some(Key::T),
            "u" => Some(Key::U),
            "v" => Some(Key::V),
            "w" => Some(Key::W),
            "x" => Some(Key::X),
            "y" => Some(Key::Y),
            "z" => Some(Key::Z),

            "0" => Some(Key::N0),
            "1" => Some(Key::N1),
            "2" => Some(Key::N2),
            "3" => Some(Key::N3),
            "4" => Some(Key::N4),
            "5" => Some(Key::N5),
            "6" => Some(Key::N6),
            "7" => Some(Key::N7),
            "8" => Some(Key::N8),
            "9" => Some(Key::N9),

            _ => None,
        },
        keyboard::Key::Unidentified(_) => None,
        keyboard::Key::Dead(_) => None,
    }
}
