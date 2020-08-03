use std::any::Any;
use crate::core::ecs::{
    components::Updatable,
    Component
};

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

impl Updatable for GraphicRendererComponent {
    fn before_update(&mut self) {
    }

    fn update(&mut self) {
    }

    fn late_update(&mut self) {
    }
}

impl GraphicRendererComponent {
    pub fn new() -> GraphicRendererComponent {
        GraphicRendererComponent {
        }
    }
}
