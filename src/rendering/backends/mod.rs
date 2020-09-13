mod backend_interface;
pub use backend_interface::BackendInterface;

#[cfg(feature = "no-backend")]
mod no_backend;
#[cfg(feature = "no-backend")]
pub use no_backend::NoBackend as Backend;

#[cfg(not(feature = "no-backend"))]
mod halstate;
#[cfg(not(feature = "no-backend"))] pub use halstate::HalState;

#[cfg(feature = "vulkan")]
mod vulkan_backend;
#[cfg(feature = "vulkan")]
pub use vulkan_backend::VulkanBackend as Backend;

#[macro_use]
mod resource_disposable;
pub use resource_disposable::ResourceDisposable;
pub use panic_if_resource_isnt_disposed;

mod shader_bindings;
pub use shader_bindings::ShaderBindings;

mod texture_bindings;
pub use texture_bindings::TextureBindings;

mod vertex;
pub use vertex::*;

mod graphics_device;
pub use graphics_device::GraphicsDevice;
