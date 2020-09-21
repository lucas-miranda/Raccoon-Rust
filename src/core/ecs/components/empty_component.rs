use std::any::Any;

use crate::{
    core::ecs::{
        components::Updatable,
        Component
    },
    rendering::backends::{
        ResourceDisposable,
        GraphicsDevice
    }
};

pub struct EmptyComponent {
}

impl Component for EmptyComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Updatable for EmptyComponent {
    fn before_update(&mut self) {
    }

    fn update(&mut self) {
    }

    fn late_update(&mut self) {
    }
}

impl ResourceDisposable for EmptyComponent {
    fn is_disposed(&self) -> bool {
        false
    }

    fn dispose(&mut self, device: &GraphicsDevice) {
    }
}

impl Drop for EmptyComponent {
    fn drop(&mut self) {
    }
}
