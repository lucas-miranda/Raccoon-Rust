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

use gfx_hal::{
    device,
    pso
};

use crate::{
    rendering::{
        ShaderStage
    }
};

#[derive(Debug)]
pub enum HalGraphicsPipelineError {
    ShaderModuleCreationFailed {
        stage: ShaderStage,
        shader_error: device::ShaderError
    },
    PipelineLayoutCreationFailed(device::OutOfMemory),
    CreationError(pso::CreationError)
}

impl Display for HalGraphicsPipelineError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HalGraphicsPipelineError::ShaderModuleCreationFailed { stage, shader_error } => {
                write!(fmt, "{} shader module creation has failed => {}", stage, shader_error)
            },
            HalGraphicsPipelineError::PipelineLayoutCreationFailed(err) => {
                write!(fmt, "Pipeline layout creation has failed: {}", err)
            },
            HalGraphicsPipelineError::CreationError(err) => {
                write!(fmt, "Graphic Pipeline creation has failed => {}", err)
            }
        }
    }
}

impl Error for HalGraphicsPipelineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HalGraphicsPipelineError::ShaderModuleCreationFailed { stage, shader_error } => Some(shader_error),
            HalGraphicsPipelineError::PipelineLayoutCreationFailed(err) => Some(err),
            HalGraphicsPipelineError::CreationError(err) => Some(err)
        }
    }
}
