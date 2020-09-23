use std::any::Any;

use crate::{
    core::ecs::{
        components::Updatable,
        Component
    },
    math::{
        Vector2
    },
    rendering::{
        GraphicsDevice,
        ResourceDisposable
    }
};

pub struct TransformComponent {
    pub position: Vector2<f32>,
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
        /*
        self.position += Vector2::try_with(1i16, 2i16).unwrap();
        println!("pos: {:?}", self.position);
        */
    }

    fn late_update(&mut self) {
    }
}

impl ResourceDisposable for TransformComponent {
    fn is_disposed(&self) -> bool {
        false
    }

    fn dispose(&mut self, device: &GraphicsDevice) {
    }
}

impl Drop for TransformComponent {
    fn drop(&mut self) {
    }
}

impl TransformComponent {
    pub fn new() -> TransformComponent {
        TransformComponent {
            position: Vector2::new(),
            rotation: 0f32
        }
    }
}
