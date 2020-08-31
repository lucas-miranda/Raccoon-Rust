mod backend_interface;
pub use backend_interface::BackendInterface;

#[cfg(feature = "no-backend")]
mod no_backend;
#[cfg(feature = "no-backend")]
pub use no_backend::NoBackend as Backend;

#[cfg(not(feature = "no-backend"))]
mod halstate;
#[cfg(not(feature = "no-backend"))]
pub use halstate::HalState;

#[cfg(feature = "vulkan")]
mod vulkan_backend;
#[cfg(feature = "vulkan")]
pub use vulkan_backend::VulkanBackend as Backend;
