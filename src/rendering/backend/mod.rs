mod renderer_backend_interface;
pub use renderer_backend_interface::RendererBackendInterface;

#[cfg(feature = "no-backend")]
mod no_renderer_backend;
#[cfg(feature = "no-backend")]
pub use {
    no_renderer_backend::NoRendererBackend as RendererBackend
};

#[cfg(not(feature = "no-backend"))]
pub mod hal;

#[cfg(not(feature = "no-backend"))]
pub use {
    hal::DeviceAdapterBackend   as DeviceAdapterBackend,
    hal::RendererBackend        as RendererBackend,
    hal::ShaderBindings         as ShaderBindings,
    hal::TextureBindings        as TextureBindings,
};

pub mod error;

