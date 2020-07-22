use std::any::Any;
use crate::core::ecs::Component;

pub struct GraphicRendererComponent {
}

impl Component for GraphicRendererComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl GraphicRendererComponent {
    pub fn new() -> GraphicRendererComponent {
        GraphicRendererComponent {
        }
    }
}
