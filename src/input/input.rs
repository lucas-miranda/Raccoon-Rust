use std::collections::HashMap;

use crate::{
    events::Event,
    input::{
        Button,
        ButtonState,
        InputEvent,
        KeyCode,
        MouseButton
    },
    math::Vector2
};

pub struct Input {
    // keyboard
    // TODO  maybe it should be using scancode instead keycode
    keys: HashMap<KeyCode, Button>,

    // mouse
    mouse_position: Vector2<f64>,
    mouse_buttons: HashMap<MouseButton, Button>,
    mouse_extra_buttons: HashMap<u8, Button>
}

macro_rules! update_state {
    ($button:expr, $button_current_state:expr) => {
        if let ButtonState::Pressed = $button_current_state {
            $button.press();
        } else {
            $button.release();
        }

        let state = $button.state();
        if *$button_current_state != *state {
            *$button_current_state = *state;
        }
    }
}

/*
impl InputEventsIndirectHandler<Box<&mut dyn InputEventListener>> for Input {
    fn handle(&mut self, listener: &mut Box<&mut dyn InputEventListener>, events: &mut Vec<Event<InputEvent>>) {
        for event in events.iter_mut() {
            listener.handle(event);
        }
    }

    fn handle_multiple(&mut self, mut listeners: Vec<Box<&mut dyn InputEventListener>>, events: &mut Vec<Event<InputEvent>>) {
        for event in events.iter_mut() {
            match &mut event.kind_mut() {
                InputEvent::Keyboard(ref mut e) => {
                    let key_state = &mut e.state;

                    match &e.key {
                        Some(keycode) => {
                            match self.keys.get_mut(&keycode) {
                                Some(ref mut key) => {
                                    update_state!(key, key_state);
                                },
                                None => {
                                    println!("Keyboard's key {:?} not registered.", &keycode);
                                }
                            };
                        },
                        None => ()
                    };
                },
                InputEvent::MouseButton(ref mut e) => {
                    let button_state = &mut e.state;

                    if let MouseButton::Other(id) = e.button {
                        match self.mouse_extra_buttons.get_mut(&id) {
                            Some(ref mut button) => {
                                update_state!(button, button_state);
                            },
                            None => {
                                let mut button = Button::new();
                                update_state!(button, button_state);
                                self.mouse_extra_buttons.insert(id, button);
                            }
                        }
                    } else {
                        match self.mouse_buttons.get_mut(&e.button) {
                            Some(ref mut button) => { 
                                update_state!(button, button_state);
                            },
                            None => {
                                println!("Mouse button {:?} not registered.", &e.button);
                            }
                        }
                    }
                },
                InputEvent::CursorEntered => {
                    // TODO  enable mouse interaction
                },
                InputEvent::CursorLeft => {
                    // TODO  disable mouse interaction
                },
                InputEvent::CursorMoved { position, modifiers } => {
                    self.mouse_position = *position;
                },
                _ => (),
            };

            for listener in listeners.iter_mut() {
                listener.handle(event);
            }
        }
    }
}
*/

impl Input {
    pub fn key(&self, key: KeyCode) -> Option<&Button> {
        self.keys.get(&key)
    }

    pub fn key_mut(&mut self, key: KeyCode) -> Option<&mut Button> {
        self.keys.get_mut(&key)
    }

    pub(crate) fn new() -> Input {
        let mut keys = HashMap::with_capacity(161);

        keys.insert(KeyCode::D0, Button::new());
        keys.insert(KeyCode::D1, Button::new());
        keys.insert(KeyCode::D2, Button::new());
        keys.insert(KeyCode::D3, Button::new());
        keys.insert(KeyCode::D4, Button::new());
        keys.insert(KeyCode::D5, Button::new());
        keys.insert(KeyCode::D6, Button::new());
        keys.insert(KeyCode::D7, Button::new());
        keys.insert(KeyCode::D8, Button::new());
        keys.insert(KeyCode::D9, Button::new());
        keys.insert(KeyCode::A, Button::new());
        keys.insert(KeyCode::B, Button::new());
        keys.insert(KeyCode::C, Button::new());
        keys.insert(KeyCode::D, Button::new());
        keys.insert(KeyCode::E, Button::new());
        keys.insert(KeyCode::F, Button::new());
        keys.insert(KeyCode::G, Button::new());
        keys.insert(KeyCode::H, Button::new());
        keys.insert(KeyCode::I, Button::new());
        keys.insert(KeyCode::J, Button::new());
        keys.insert(KeyCode::K, Button::new());
        keys.insert(KeyCode::L, Button::new());
        keys.insert(KeyCode::M, Button::new());
        keys.insert(KeyCode::N, Button::new());
        keys.insert(KeyCode::O, Button::new());
        keys.insert(KeyCode::P, Button::new());
        keys.insert(KeyCode::Q, Button::new());
        keys.insert(KeyCode::R, Button::new());
        keys.insert(KeyCode::S, Button::new());
        keys.insert(KeyCode::T, Button::new());
        keys.insert(KeyCode::U, Button::new());
        keys.insert(KeyCode::V, Button::new());
        keys.insert(KeyCode::W, Button::new());
        keys.insert(KeyCode::X, Button::new());
        keys.insert(KeyCode::Y, Button::new());
        keys.insert(KeyCode::Z, Button::new());
        keys.insert(KeyCode::Escape, Button::new());
        keys.insert(KeyCode::F1, Button::new());
        keys.insert(KeyCode::F2, Button::new());
        keys.insert(KeyCode::F3, Button::new());
        keys.insert(KeyCode::F4, Button::new());
        keys.insert(KeyCode::F5, Button::new());
        keys.insert(KeyCode::F6, Button::new());
        keys.insert(KeyCode::F7, Button::new());
        keys.insert(KeyCode::F8, Button::new());
        keys.insert(KeyCode::F9, Button::new());
        keys.insert(KeyCode::F10, Button::new());
        keys.insert(KeyCode::F11, Button::new());
        keys.insert(KeyCode::F12, Button::new());
        keys.insert(KeyCode::F13, Button::new());
        keys.insert(KeyCode::F14, Button::new());
        keys.insert(KeyCode::F15, Button::new());
        keys.insert(KeyCode::F16, Button::new());
        keys.insert(KeyCode::F17, Button::new());
        keys.insert(KeyCode::F18, Button::new());
        keys.insert(KeyCode::F19, Button::new());
        keys.insert(KeyCode::F20, Button::new());
        keys.insert(KeyCode::F21, Button::new());
        keys.insert(KeyCode::F22, Button::new());
        keys.insert(KeyCode::F23, Button::new());
        keys.insert(KeyCode::F24, Button::new());
        keys.insert(KeyCode::PrintScreen, Button::new());
        keys.insert(KeyCode::ScrollLock, Button::new());
        keys.insert(KeyCode::PauseBreak, Button::new());
        keys.insert(KeyCode::Insert, Button::new());
        keys.insert(KeyCode::Home, Button::new());
        keys.insert(KeyCode::Delete, Button::new());
        keys.insert(KeyCode::End, Button::new());
        keys.insert(KeyCode::PageDown, Button::new());
        keys.insert(KeyCode::PageUp, Button::new());
        keys.insert(KeyCode::Left, Button::new());
        keys.insert(KeyCode::Up, Button::new());
        keys.insert(KeyCode::Right, Button::new());
        keys.insert(KeyCode::Down, Button::new());
        keys.insert(KeyCode::Backspace, Button::new());
        keys.insert(KeyCode::Return, Button::new());
        keys.insert(KeyCode::Space, Button::new());
        keys.insert(KeyCode::Compose, Button::new());
        keys.insert(KeyCode::Caret, Button::new());
        keys.insert(KeyCode::Numlock, Button::new());
        keys.insert(KeyCode::Numpad0, Button::new());
        keys.insert(KeyCode::Numpad1, Button::new());
        keys.insert(KeyCode::Numpad2, Button::new());
        keys.insert(KeyCode::Numpad3, Button::new());
        keys.insert(KeyCode::Numpad4, Button::new());
        keys.insert(KeyCode::Numpad5, Button::new());
        keys.insert(KeyCode::Numpad6, Button::new());
        keys.insert(KeyCode::Numpad7, Button::new());
        keys.insert(KeyCode::Numpad8, Button::new());
        keys.insert(KeyCode::Numpad9, Button::new());
        keys.insert(KeyCode::AbntC1, Button::new());
        keys.insert(KeyCode::AbntC2, Button::new());
        keys.insert(KeyCode::Add, Button::new());
        keys.insert(KeyCode::Apostrophe, Button::new());
        keys.insert(KeyCode::Apps, Button::new());
        keys.insert(KeyCode::At, Button::new());
        keys.insert(KeyCode::Ax, Button::new());
        keys.insert(KeyCode::Backslash, Button::new());
        keys.insert(KeyCode::Calculator, Button::new());
        keys.insert(KeyCode::Capital, Button::new());
        keys.insert(KeyCode::Colon, Button::new());
        keys.insert(KeyCode::Comma, Button::new());
        keys.insert(KeyCode::Convert, Button::new());
        keys.insert(KeyCode::Decimal, Button::new());
        keys.insert(KeyCode::Divide, Button::new());
        keys.insert(KeyCode::Equals, Button::new());
        keys.insert(KeyCode::Grave, Button::new());
        keys.insert(KeyCode::Kana, Button::new());
        keys.insert(KeyCode::Kanji, Button::new());
        keys.insert(KeyCode::LeftAlt, Button::new());
        keys.insert(KeyCode::LeftBracket, Button::new());
        keys.insert(KeyCode::LeftControl, Button::new());
        keys.insert(KeyCode::LeftShift, Button::new());
        keys.insert(KeyCode::LeftCommand, Button::new());
        keys.insert(KeyCode::Mail, Button::new());
        keys.insert(KeyCode::MediaSelect, Button::new());
        keys.insert(KeyCode::MediaStop, Button::new());
        keys.insert(KeyCode::Minus, Button::new());
        keys.insert(KeyCode::Multiply, Button::new());
        keys.insert(KeyCode::Mute, Button::new());
        keys.insert(KeyCode::MyComputer, Button::new());
        keys.insert(KeyCode::NavigateForward, Button::new());
        keys.insert(KeyCode::NavigateBackward, Button::new());
        keys.insert(KeyCode::NextTrack, Button::new());
        keys.insert(KeyCode::NoConvert, Button::new());
        keys.insert(KeyCode::NumpadComma, Button::new());
        keys.insert(KeyCode::NumpadEnter, Button::new());
        keys.insert(KeyCode::NumpadEquals, Button::new());
        keys.insert(KeyCode::OEM102, Button::new());
        keys.insert(KeyCode::Period, Button::new());
        keys.insert(KeyCode::PlayPause, Button::new());
        keys.insert(KeyCode::Power, Button::new());
        keys.insert(KeyCode::PrevTrack, Button::new());
        keys.insert(KeyCode::RightAlt, Button::new());
        keys.insert(KeyCode::RightBracket, Button::new());
        keys.insert(KeyCode::RightControl, Button::new());
        keys.insert(KeyCode::RightShift, Button::new());
        keys.insert(KeyCode::RightCommand, Button::new());
        keys.insert(KeyCode::Semicolon, Button::new());
        keys.insert(KeyCode::Slash, Button::new());
        keys.insert(KeyCode::Sleep, Button::new());
        keys.insert(KeyCode::Stop, Button::new());
        keys.insert(KeyCode::Subtract, Button::new());
        keys.insert(KeyCode::Sysrq, Button::new());
        keys.insert(KeyCode::Tab, Button::new());
        keys.insert(KeyCode::Underline, Button::new());
        keys.insert(KeyCode::Unlabeled, Button::new());
        keys.insert(KeyCode::VolumeDown, Button::new());
        keys.insert(KeyCode::VolumeUp, Button::new());
        keys.insert(KeyCode::Wake, Button::new());
        keys.insert(KeyCode::WebBack, Button::new());
        keys.insert(KeyCode::WebFavorites, Button::new());
        keys.insert(KeyCode::WebForward, Button::new());
        keys.insert(KeyCode::WebHome, Button::new());
        keys.insert(KeyCode::WebRefresh, Button::new());
        keys.insert(KeyCode::WebSearch, Button::new());
        keys.insert(KeyCode::WebStop, Button::new());
        keys.insert(KeyCode::Yen, Button::new());
        keys.insert(KeyCode::Copy, Button::new());
        keys.insert(KeyCode::Paste, Button::new());
        keys.insert(KeyCode::Cut, Button::new());

        let mut mouse_buttons = HashMap::with_capacity(3);

        mouse_buttons.insert(MouseButton::Left, Button::new());
        mouse_buttons.insert(MouseButton::Right, Button::new());
        mouse_buttons.insert(MouseButton::Middle, Button::new());

        Input {
            keys,
            mouse_position: Vector2::default(),
            mouse_buttons,
            mouse_extra_buttons: HashMap::new()
        }
    }
}
