#[cfg(feature = "no-backend")]
mod renderer_no_backend_error;
#[cfg(feature = "no-backend")]
pub use renderer_no_backend_error::RendererNoBackendError as RendererBackendError;

#[cfg(not(feature = "no-backend"))]
mod renderer_backend_error;
#[cfg(not(feature = "no-backend"))]
pub use renderer_backend_error::RendererBackendError;
