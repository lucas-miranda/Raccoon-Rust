mod backends;
pub use backends::{
    RendererBackend,
    RendererBackendInterface
};

mod graphics_device;
pub use graphics_device::GraphicsDevice;

mod renderer;
pub use renderer::Renderer;

#[macro_use]
mod resource_disposable;
pub use resource_disposable::ResourceDisposable;
pub use panic_if_resource_isnt_disposed;
pub use panic_if_resources_isnt_disposed;

mod rendering_requirements;
pub use rendering_requirements::RenderingRequirements;

mod vertex;
pub use vertex::*;

// macros
#[macro_export]
macro_rules! verify_backend_requirements {
    ($requirements:expr) => {
        if !RendererBackend::has_requirements($requirements) {
            panic!(
                "For selected backend: {}\nIt doesn't met one or more requirements: {}", 
                RendererBackend::name(),
                $requirements
            );
        }
    };

    ($requirements:expr, $additional_message:literal) => {
        if !RendererBackend::has_requirements($requirements) {
            panic!(
                "{}\nFor selected backend: {}\nIt doesn't met one or more requirements: {}", 
                $additional_message,
                RendererBackend::name(),
                $requirements
            );
        }
    };
}

pub use verify_backend_requirements;

