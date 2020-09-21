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
            GraphicsDevice,
            ResourceDisposable,
            StandardVertex,
            panic_if_resource_isnt_disposed
        },
        Renderer,
        RenderingRequirements
    }
};

pub struct Image {
    texture: Texture,
    vertices: [StandardVertex; 6],
    disposed: bool
}

impl Drawable for Image {
    fn draw(&mut self, renderer: &mut Renderer) {
        renderer.draw_texture(&self.vertices, &mut self.texture, None);
    }
}

impl Graphic for Image {
}

impl ResourceDisposable for Image {
    fn is_disposed(&self) -> bool {
        self.disposed
    }

    fn dispose(&mut self, device: &GraphicsDevice) {
        if self.disposed {
            return;
        }

        self.disposed = true;
        self.texture.dispose(device)
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        panic_if_resource_isnt_disposed!(self);
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
            ],
            disposed: false
        }
    }
}
