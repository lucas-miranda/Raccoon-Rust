use crate::rendering::Renderer;

pub trait Renderable {
    fn render(&self, renderer: &Renderer);
    fn dispose(&mut self);
}
