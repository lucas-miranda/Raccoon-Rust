use crate::{
    core::ecs::{
        System,
        SystemDataContainer
    },
    graphics::Graphic,
    rendering::Renderer
};

pub struct RenderingSystem<'a> {
    renderer: Renderer
}

impl<'a> System for RenderingSystem<'a> {
    type DataType = SystemDataContainer<'a, dyn Graphic>;

    fn run(&mut self, component_type: Self::DataType) {
        self.renderer.render(component_type.get(0));
    }
}

impl<'a> RenderingSystem<'a> {
    pub fn new() -> RenderingSystem<'a> {
        RenderingSystem {
            renderer: Renderer::new().expect("Can't initialize Renderer.")
        }
    }
}
