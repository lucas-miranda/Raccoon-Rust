use std::{
    error::{
        Error
    },
    fmt::{
        self,
        Display,
        Formatter
    }
};

use crate::{
    rendering::backend::error::{
        RendererBackendError
    }
};

#[derive(Debug)]
pub enum RenderError {
    Backend(RendererBackendError)
}

impl Display for RenderError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::Backend(err) => {
                write!(fmt, "Internal backend raised an error => {}", err)
            },
        }
    }
}

impl Error for RenderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RenderError::Backend(err) => Some(err)
        }
    }
}

impl From<RendererBackendError> for RenderError {
    fn from(renderer_backend_error: RendererBackendError) -> RenderError {
        RenderError::Backend(renderer_backend_error)
    }
}
