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
    HalInitError,
    HalRenderError
};

#[derive(Debug)]
pub enum HalError {
    Init(HalInitError),
    Render(HalRenderError)
}

impl Display for HalError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HalError::Init(err) => {
                write!(fmt, "Initialization Error => {}", err)
            },
            HalError::Render(err) =>{
                write!(fmt, "Render Error => {}", err)
            }
        }
    }
}

impl Error for HalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HalError::Init(err) => Some(err),
            HalError::Render(err) => Some(err),
            _ => None
        }
    }
}

impl From<HalInitError> for HalError {
    fn from(init_error: HalInitError) -> HalError {
        HalError::Init(init_error)
    }
}

impl From<HalRenderError> for HalError {
    fn from(render_error: HalRenderError) -> HalError {
        HalError::Render(render_error)
    }
}
