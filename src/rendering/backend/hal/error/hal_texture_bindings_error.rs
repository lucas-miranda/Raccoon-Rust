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
    buffer
};

use image_handler;

#[derive(Debug)]
pub enum HalTextureBindingsError {
    ImageLoading(image_handler::ImageError),
    BufferCreation(buffer::CreationError)
}

impl Display for HalTextureBindingsError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HalTextureBindingsError::ImageLoading(err) => {
                write!(fmt, "Image loader raised an error: {}", err)
            },
            HalTextureBindingsError::BufferCreation(err) =>{
                write!(fmt, "Error when creating a buffer: {}", err)
            }
        }
    }
}

impl Error for HalTextureBindingsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HalTextureBindingsError::ImageLoading(err) => Some(err),
            HalTextureBindingsError::BufferCreation(err) => Some(err)
        }
    }
}
