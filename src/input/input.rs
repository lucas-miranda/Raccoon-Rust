use std::collections::HashMap;

use crate::{
    input::{
        Key,
        KeyCode
    }
};

pub struct Input {
    _keys: HashMap<KeyCode, Key>
}

impl Input {
    pub fn key(&self, key: KeyCode) -> Option<&Key> {
        self._keys.get(&key)
    }

    pub fn key_mut(&mut self, key: KeyCode) -> Option<&mut Key> {
        self._keys.get_mut(&key)
    }

    pub(crate) fn new() -> Input {
        let mut keys = HashMap::new();

        keys.insert(KeyCode::D0, Key::new());
        keys.insert(KeyCode::D1, Key::new());
        keys.insert(KeyCode::D2, Key::new());
        keys.insert(KeyCode::D3, Key::new());
        keys.insert(KeyCode::D4, Key::new());
        keys.insert(KeyCode::D5, Key::new());
        keys.insert(KeyCode::D6, Key::new());
        keys.insert(KeyCode::D7, Key::new());
        keys.insert(KeyCode::D8, Key::new());
        keys.insert(KeyCode::D9, Key::new());
        keys.insert(KeyCode::A,  Key::new());
        keys.insert(KeyCode::B,  Key::new());
        keys.insert(KeyCode::C,  Key::new());
        keys.insert(KeyCode::D,  Key::new());
        keys.insert(KeyCode::E,  Key::new());
        keys.insert(KeyCode::F,  Key::new());
        keys.insert(KeyCode::G,  Key::new());
        keys.insert(KeyCode::H,  Key::new());
        keys.insert(KeyCode::I,  Key::new());
        keys.insert(KeyCode::J,  Key::new());
        keys.insert(KeyCode::K,  Key::new());
        keys.insert(KeyCode::L,  Key::new());
        keys.insert(KeyCode::M,  Key::new());
        keys.insert(KeyCode::N,  Key::new());
        keys.insert(KeyCode::O,  Key::new());
        keys.insert(KeyCode::P,  Key::new());
        keys.insert(KeyCode::Q,  Key::new());
        keys.insert(KeyCode::R,  Key::new());
        keys.insert(KeyCode::S,  Key::new());
        keys.insert(KeyCode::T,  Key::new());
        keys.insert(KeyCode::U,  Key::new());
        keys.insert(KeyCode::V,  Key::new());
        keys.insert(KeyCode::W,  Key::new());
        keys.insert(KeyCode::X,  Key::new());
        keys.insert(KeyCode::Y,  Key::new());
        keys.insert(KeyCode::Z,  Key::new());

        Input {
            _keys: keys
        }
    }
}
