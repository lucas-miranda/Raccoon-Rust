use std::{
    error::{
        Error
    },
    fmt::{
        self,
        Display,
        Formatter
    },
    path::{
        PathBuf
    }
};

use image_handler;

use crate::{
    rendering::backend::error::{
        TextureBindingsError
    }
};

#[derive(Debug)]
pub enum TextureError {
    Loading(TextureBindingsError),
}

impl Display for TextureError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TextureError::Loading(err) => {
                write!(fmt, "Failed to load texture: {}", err)
            }
        }
    }
}

impl Error for TextureError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TextureError::Loading(err) => Some(err)
        }
    }
}
