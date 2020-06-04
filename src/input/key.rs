
pub struct Key {
}

impl Key {
    pub fn is_pressed(&self) -> bool {
        false
    }

    pub fn is_released(&self) -> bool {
        false
    }

    pub fn is_down(&self) -> bool {
        false
    }

    pub fn is_up(&self) -> bool {
        true
    }

    pub(super) fn new() -> Key {
        Key {
        }
    }
}
