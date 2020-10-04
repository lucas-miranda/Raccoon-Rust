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
    rendering::error::{
        RendererInitError
    }
};

#[derive(Debug)]
pub enum GameInitError {
    RendererCreation(RendererInitError),
}

impl Display for GameInitError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameInitError::RendererCreation(err) => {
                write!(fmt, "Can't create renderer: {}", err)
            }
        }
    }
}

impl Error for GameInitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GameInitError::RendererCreation(err) => Some(err)
        }
    }
}

#[derive(Debug)]
pub enum GameRuntimeError {
    RendererNotAvailable
}

impl Display for GameRuntimeError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameRuntimeError::RendererNotAvailable => {
                write!(fmt, "Renderer isn't available.")
            }
        }
    }
}

impl Error for GameRuntimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
