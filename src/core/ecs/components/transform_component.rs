use std::any::Any;
use crate::core::ecs::Component;

pub struct TransformComponent {
    pub x: f32,
    pub y: f32,
    pub rotation: f32
}

impl Component for TransformComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl TransformComponent {
    pub fn new() -> TransformComponent {
        TransformComponent {
            x: 0f32,
            y: 0f32,
            rotation: 0f32
        }
    }
}
