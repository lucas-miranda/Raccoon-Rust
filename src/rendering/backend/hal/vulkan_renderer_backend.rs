use enumflags2::BitFlags;

use gfx_backend_vulkan;

use crate::{
    core::GameLoopInterface,
    graphics::{
        shaders::{
            Shader
        },
        Texture
    },
    rendering::{
        backend::{
            error::{
                RendererBackendError
            },
            RendererBackend,
            RendererBackendInterface
        },
        GraphicsDevice,
        RenderingRequirements,
        VertexPosition,
        VertexUV
    },
    window::Window
};

use super::{
    error,
    DeviceAdapterBackend,
    ShaderBindings,
    State,
    TextureBindings
};

pub struct VulkanRendererBackend {
    hal_state: State
}

impl VulkanRendererBackend {
    pub fn new<L: 'static + GameLoopInterface>(window: Option<&Window<L>>) -> Result<Self, RendererBackendError> {
        let hal_state = match window {
            Some(w) => {
                match State::new(w) {
                    Ok(state) => Ok(state),
                    Err(e) => Err(RendererBackendError::InternalBackend(e.into()))
                }
            },
            None => Err(RendererBackendError::InvalidWindow)
        }?;

        Ok(Self { 
            hal_state
        })
    }
}

impl RendererBackendInterface for VulkanRendererBackend {
    type InternalBackend = gfx_backend_vulkan::Backend;
    type TextureBindings = TextureBindings;
    type ShaderBindings = ShaderBindings;
    type DeviceAdapterBackend = DeviceAdapterBackend;
    type InternalBackendError = error::HalError;

    fn name() -> &'static str {
        "Vulkan"
    }

    fn has_requirements(requirements: RenderingRequirements) -> bool {
        true
    }

    fn graphics_device(&self) -> &GraphicsDevice {
        &self.hal_state.graphics_device
    }

    fn mut_graphics_device(&mut self) -> &mut GraphicsDevice {
        &mut self.hal_state.graphics_device
    }

    fn draw_clear_frame(&mut self, color: [f32; 4]) {
        self.hal_state.draw_clear_frame(color)
    }

    fn draw_texture_with_vertices<V, P, U>(&mut self, vertices: &[V], texture: &mut Texture, shader: &Shader) -> Result<(), RendererBackendError> where 
        V: VertexPosition<P> + VertexUV<U>
    {
        self.hal_state.draw_texture_with_vertices(vertices, texture, shader)
                      .map_err(|e| RendererBackendError::InternalBackend(e.into()))
    }
}
