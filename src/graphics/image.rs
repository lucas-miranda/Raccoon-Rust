use crate::{
    graphics::{
        Drawable,
        Graphic,
        Texture
    },
    rendering::{
        backends::{
            Backend,
            BackendInterface,
            StandardVertex
        },
        Renderer,
        RenderingRequirements
    }
};

pub struct Image {
    texture: Texture,
    vertices: [StandardVertex; 6]
}

impl Drawable for Image {
    fn draw(&mut self, renderer: &mut Renderer) {
        renderer.draw_texture(&self.vertices, &mut self.texture, None);
    }
}

impl Graphic for Image {
}

impl Drop for Image {
    fn drop(&mut self) {
    }
}

impl Image {
    pub fn new(texture: Texture) -> Image {
        verify_backend_requirements!(RenderingRequirements::Texture, "Can't create Image.");

        Image {
            texture,
            vertices: [
                StandardVertex { position: [ -0.5,  0.33 ], uv: [0.0, 1.0] },
                StandardVertex { position: [  0.5,  0.33 ], uv: [1.0, 1.0] },
                StandardVertex { position: [  0.5, -0.33 ], uv: [1.0, 0.0] },

                StandardVertex { position: [ -0.5,  0.33 ], uv: [0.0, 1.0] },
                StandardVertex { position: [  0.5, -0.33 ], uv: [1.0, 0.0] },
                StandardVertex { position: [ -0.5, -0.33 ], uv: [0.0, 0.0] }
            ]
        }
    }
}
