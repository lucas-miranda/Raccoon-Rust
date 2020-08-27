use crate::{
    input::ButtonState
};

pub struct Button {
    state: ButtonState
}

impl Button {
    pub fn press(&mut self) {
        match &self.state {
            ButtonState::Released | ButtonState::Up => self.state = ButtonState::Pressed,
            ButtonState::Pressed => self.state = ButtonState::Down,
            ButtonState::Down => (),
        }
    }

    pub fn release(&mut self) {
        match &self.state {
            ButtonState::Up => (),
            ButtonState::Pressed | ButtonState::Down => self.state = ButtonState::Released,
            ButtonState::Released => self.state = ButtonState::Up,
        }
    }

    pub fn state(&self) -> &ButtonState {
        &self.state
    }

    pub fn is_pressed(&self) -> bool {
        match &self.state {
            ButtonState::Pressed => true,
            _ => false
        }
    }

    pub fn is_released(&self) -> bool {
        match &self.state {
            ButtonState::Released => true,
            _ => false
        }
    }

    pub fn is_down(&self) -> bool {
        match &self.state {
            ButtonState::Down => true,
            _ => false
        }
    }

    pub fn is_up(&self) -> bool {
        match &self.state {
            ButtonState::Up => true,
            _ => false
        }
    }

    pub(super) fn new() -> Self {
        Self {
            state: ButtonState::Up
        }
    }
}
