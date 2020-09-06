use crate::rendering::Renderer;

pub trait Drawable : Drop {
    fn draw(&mut self, renderer: &mut Renderer);
}
