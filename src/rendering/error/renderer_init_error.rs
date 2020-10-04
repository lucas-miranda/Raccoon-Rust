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
    graphics::shaders::{
        ShaderBuilderInitError,
        ShaderBuildError
    },
    rendering::backend::error::{
        RendererBackendError
    }
};

#[derive(Debug)]
pub enum RendererInitError {
    BackendCreation(RendererBackendError),
    ShaderBuilderCreation(ShaderBuilderInitError),
    DefaultShaderCreation(ShaderBuildError)
}

impl Display for RendererInitError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RendererInitError::BackendCreation(err) => {
                write!(fmt, "Internal backend raised an error => {}", err)
            },
            RendererInitError::ShaderBuilderCreation(err) => {
                write!(fmt, "Shader builder creation raised an error => {}", err)
            },
            RendererInitError::DefaultShaderCreation(err) => {
                write!(fmt, "Shader builder raised an error when creating default shader => {}", err)
            }
        }
    }
}

impl Error for RendererInitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RendererInitError::BackendCreation(err) => Some(err),
            RendererInitError::ShaderBuilderCreation(err) => Some(err),
            RendererInitError::DefaultShaderCreation(err) => Some(err)
        }
    }
}

impl From<RendererBackendError> for RendererInitError {
    fn from(backend_error: RendererBackendError) -> RendererInitError {
        RendererInitError::BackendCreation(backend_error)
    }
}

impl From<ShaderBuilderInitError> for RendererInitError {
    fn from(shader_builder_init_error: ShaderBuilderInitError) -> RendererInitError {
        RendererInitError::ShaderBuilderCreation(shader_builder_init_error)
    }
}

impl From<ShaderBuildError> for RendererInitError {
    fn from(shader_build_error: ShaderBuildError) -> RendererInitError {
        RendererInitError::DefaultShaderCreation(shader_build_error)
    }
}
