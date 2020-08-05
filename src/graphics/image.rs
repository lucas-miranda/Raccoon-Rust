use crate::{
    graphics::Renderable,
    rendering::{
        Backend,
        BackendInterface,
        Renderer,
        RenderingRequirements
    }
};

pub struct Image {
}

impl Renderable for Image {
    fn render(&self, renderer: &Renderer) {
    }

    fn dispose(&mut self) {
    }
}

impl Image {
    pub fn new() -> Image {
        verify_backend_requirements!(RenderingRequirements::Texture, "Can't create Image.");

        Image {
        }
    }
}
