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

use super::{
    HalGraphicsPipelineError
};

#[derive(Debug)]
pub enum HalRenderError {
    GraphicsPipelineCreation(HalGraphicsPipelineError),
    MissingGraphicsPipeline,
    MissingVertexBuffer,
    MissingPipelineLayout
}

impl Display for HalRenderError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HalRenderError::GraphicsPipelineCreation(err) => {
                write!(fmt, "Graphics pipeline creation raised an error: {}", err)
            },
            HalRenderError::MissingGraphicsPipeline => {
                write!(fmt, "Expecting a graphics pipeline, but there is none.")
            },
            HalRenderError::MissingVertexBuffer => {
                write!(fmt, "Expecting a vertex buffer, but there is none")
            },
            HalRenderError::MissingPipelineLayout => {
                write!(fmt, "Expecting a pipeline layout, but there is none")
            }
        }
    }
}

impl Error for HalRenderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HalRenderError::GraphicsPipelineCreation(err) => Some(err),
            _ => None
        }
    }
}

impl From<HalGraphicsPipelineError> for HalRenderError {
    fn from(graphics_pipeline_error: HalGraphicsPipelineError) -> HalRenderError {
        HalRenderError::GraphicsPipelineCreation(graphics_pipeline_error)
    }
}
