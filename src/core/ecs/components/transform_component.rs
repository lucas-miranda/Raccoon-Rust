use std::any::Any;
use crate::core::ecs::{
    components::Updatable,
    Component
};

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

impl Updatable for TransformComponent {
    fn before_update(&mut self) {
    }

    fn update(&mut self) {
    }

    fn late_update(&mut self) {
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
