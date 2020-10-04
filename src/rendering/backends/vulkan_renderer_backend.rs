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
        GraphicsDevice,
        RendererBackend,
        RendererBackendInterface,
        RenderingRequirements,
        VertexPosition,
        VertexUV
    },
    window::Window
};

use super::{
    hal
};

pub struct VulkanRendererBackend {
    hal_state: hal::State
}

impl VulkanRendererBackend {
    pub fn new<L: 'static + GameLoopInterface>(window: Option<&Window<L>>) -> Result<Self, &'static str> {
        let hal_state = match window {
            Some(w) => {
                match hal::State::new(w) {
                    Ok(state) => state,
                    Err(e) => {
                        panic!(format!("Can't create hal state.\n{}", e));
                        return Err("Can't create hal state.");
                    }
                }
            },
            None => return Err("Missing window reference.")
        };

        Ok(Self { 
            hal_state
        })
    }
}

impl RendererBackendInterface for VulkanRendererBackend {
    type InternalBackend = gfx_backend_vulkan::Backend;
    type TextureBindings = hal::TextureBindings;
    type ShaderBindings = hal::ShaderBindings;
    type DeviceAdapterBackend = hal::DeviceAdapterBackend;

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

    fn draw_texture_with_vertices<V, P, U>(&mut self, vertices: &[V], texture: &mut Texture, shader: &Shader) where 
        V: VertexPosition<P> + VertexUV<U>
    {
        self.hal_state.draw_texture_with_vertices(vertices, texture, shader)
    }
}