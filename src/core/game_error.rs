use std::{
    error,
    fmt
};

#[derive(Debug, Clone)]
pub struct GameError;

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A game error occurred")
    }
}

impl error::Error for GameError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
