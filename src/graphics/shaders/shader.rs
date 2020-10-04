use crate::{
    rendering::{
        backend::{
            RendererBackend,
            RendererBackendInterface,
        },
        GraphicsDevice,
        ResourceDisposable,
        panic_if_resource_isnt_disposed
    }
};

type ShaderBindings = <RendererBackend as RendererBackendInterface>::ShaderBindings;

pub struct Shader {
    pub bindings: ShaderBindings,
    vertex_data: Vec<u32>,
    fragment_data: Vec<u32>,
    disposed: bool
}

impl ResourceDisposable for Shader {
    fn is_disposed(&self) -> bool {
        self.disposed
    }

    fn dispose(&mut self, device: &GraphicsDevice) {
        if self.disposed {
            return;
        }

        self.disposed = true;
        self.bindings.dispose(device);
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        panic_if_resource_isnt_disposed!(self);
    }
}

impl Shader {
    pub fn new(vertex_data: Vec<u32>, fragment_data: Vec<u32>, device: &GraphicsDevice) -> Self {
        Self {
            bindings: ShaderBindings::new(device),
            vertex_data,
            fragment_data,
            disposed: false
        }
    }

    pub fn vertex_data(&self) -> &[u32] {
        &self.vertex_data[..]
    }

    pub fn fragment_data(&self) -> &[u32] {
        &self.fragment_data[..]
    }
}
