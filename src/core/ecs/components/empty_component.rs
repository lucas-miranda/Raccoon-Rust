use std::any::Any;
use crate::core::ecs::Component;

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
