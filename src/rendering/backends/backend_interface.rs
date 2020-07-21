use crate::graphics::Graphic;

pub trait BackendInterface {
    fn has_texture_available(&self) -> bool;
    fn draw<T: Graphic>(&self, graphic: &T);
}
