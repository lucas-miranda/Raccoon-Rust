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
    rendering::backend::{
        RendererBackend,
        RendererBackendInterface
    }
};

type InternalBackendError = <RendererBackend as RendererBackendInterface>::InternalBackendError;

#[derive(Debug)]
pub enum RendererBackendError {
    InternalBackend(InternalBackendError),
    InvalidWindow
}

impl Display for RendererBackendError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RendererBackendError::InternalBackend(err) => {
                write!(fmt, "Internal backend raised an error => {}", err)
            },
            RendererBackendError::InvalidWindow => {
                write!(fmt, "Missing window reference.")
            }
        }
    }
}

impl Error for RendererBackendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RendererBackendError::InternalBackend(err) => Some(err),
            _ => None
        }
    }
}
