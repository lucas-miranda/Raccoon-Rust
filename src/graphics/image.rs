use crate::{
    graphics::{
        Drawable,
        Graphic
    },
    rendering::{
        Backend,
        BackendInterface,
        Renderer,
        RenderingRequirements
    }
};

pub struct Image {
    //texture: Texture
}

impl Drawable for Image {
    fn draw(&mut self, renderer: &mut Renderer) {
        // TODO  draw from texture data
    }

    fn dispose(&mut self) {
        //texture.dispose();
    }
}

impl Graphic for Image {
}

impl Image {
    pub fn new() -> Image {
        verify_backend_requirements!(RenderingRequirements::Texture, "Can't create Image.");

        Image {
        }
    }
}
