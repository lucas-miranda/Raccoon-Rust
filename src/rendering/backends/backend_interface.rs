use crate::rendering::RenderingRequirements;

pub trait BackendInterface {
    fn name() -> &'static str;
    fn has_requirements(requirements: RenderingRequirements) -> bool;
    //fn draw<T: Graphic>(&self, graphic: &T);
}

