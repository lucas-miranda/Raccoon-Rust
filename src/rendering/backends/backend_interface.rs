use crate::{
    graphics::{
        shaders::{
            Shader
        },
        Texture
    },
    rendering::{
        backends::{
            GraphicsDevice,
            VertexPosition,
            VertexUV
        },
        RenderingRequirements
    }
};

pub trait BackendInterface {
    type InternalBackend: gfx_hal::Backend;

    fn name() -> &'static str;
    fn has_requirements(requirements: RenderingRequirements) -> bool;
    fn graphics_device(&self) -> &GraphicsDevice;
    fn mut_graphics_device(&mut self) -> &mut GraphicsDevice;
    //fn draw<T: Graphic>(&self, graphic: &T);
    fn draw_clear_frame(&mut self, color: [f32; 4]);
    fn draw_texture_with_vertices<V, P, U>(&mut self, vertices: &[V], texture: &mut Texture, shader: &Shader) where V: VertexPosition<P> + VertexUV<U>;
}

