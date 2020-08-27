use super::{
    ButtonState,
    KeyCode,
    KeyModifiers
};

type ScanCode = u32;

pub struct KeyboardEvent {
    pub scan_code: ScanCode,
    pub state: ButtonState,
    pub key: Option<KeyCode>,
    pub modifiers: KeyModifiers
}

impl KeyboardEvent {
    pub fn new(scan_code: ScanCode, state: ButtonState, key: Option<KeyCode>, modifiers: KeyModifiers) -> Self {
        Self {
            scan_code,
            state,
            key,
            modifiers
        }
    }
}
