use crate::{
    input::TouchPhase,
    math::Vector2,
};

pub struct TouchEvent {
    phase: TouchPhase,
    position: Vector2<f64>,
    id: u64
}

impl TouchEvent {
    pub fn new(phase: TouchPhase, position: Vector2<f64>, id: u64) -> Self {
        Self {
            phase,
            position,
            id
        }
    }
}
