use crate::{
    input::{
        Key,
        KeyCode
    }
};

pub struct Input {
}

impl Input {
    pub fn key(&self, key: KeyCode) -> Option<Key> {
        None
    }
}
