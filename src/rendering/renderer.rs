use crate::{
    core::GameLoopInterface,
    graphics::{
        shaders::{
            Shader,
            ShaderBuilder
        },
        Texture,
    },
    rendering::{
        GraphicsDevice,
        RendererBackend,
        RendererBackendInterface,
        VertexPosition,
        VertexUV
    },
    window::Window
};

pub struct Renderer {
    backend: RendererBackend,
    shader_builder: ShaderBuilder,
    default_shader: Shader
}

impl Renderer {
    pub fn new<L: 'static + GameLoopInterface>(window: Option<&Window<L>>) -> Result<Self, &'static str> {
        let backend = if cfg!(feature = "no-backend") {
            RendererBackend::new::<L>(None)?
        } else {
            match window {
                Some(w) => RendererBackend::new(Some(w))?,
                None => return Err("Window reference not found")
            }
        };

        let mut shader_builder = ShaderBuilder::new()?;
        let default_shader = shader_builder.shader_from_files("../../src/resources/shaders/basic_shader.vert", "../../src/resources/shaders/basic_shader.frag", backend.graphics_device())?;

        Ok(Self {
            backend: backend,
            shader_builder,
            default_shader
        })
    }

    pub fn get_backend(&self) -> &RendererBackend {
        &self.backend
    }

    pub fn graphics_device(&self) -> &GraphicsDevice {
        self.backend.graphics_device()
    }

    pub fn mut_graphics_device(&mut self) -> &mut GraphicsDevice {
        self.backend.mut_graphics_device()
    }

    pub fn draw_clear_frame(&mut self, color: [f32; 4]) {
        self.backend.draw_clear_frame(color)
    }

    pub fn draw_texture<V, P, U>(&mut self, vertices: &[V], texture: &mut Texture, shader: Option<&Shader>) where 
        V: VertexPosition<P> + VertexUV<U>
    {
        match shader {
            Some(s) => self.backend.draw_texture_with_vertices(vertices, texture, s),
            None => self.backend.draw_texture_with_vertices(vertices, texture, &self.default_shader)
        }
    }

    /*
    pub fn draw_triangle_frame(&mut self, triangle: Triangle) -> Result<(), &'static str> {
        self._hal_state.draw_triangle_frame(triangle)
    }
    */
}
