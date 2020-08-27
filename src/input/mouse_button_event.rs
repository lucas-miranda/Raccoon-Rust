use super::{
    ButtonState,
    MouseButton,
    KeyModifiers
};

pub struct MouseButtonEvent {
    pub state: ButtonState,
    pub button: MouseButton,
    pub modifiers: KeyModifiers
}

impl MouseButtonEvent {
    pub fn new(state: ButtonState, button: MouseButton, modifiers: KeyModifiers) -> Self {
        Self {
            state,
            button,
            modifiers
        }
    }
}
