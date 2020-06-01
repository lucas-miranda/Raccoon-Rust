use std::{
    error,
    fmt
};

#[derive(Debug, Clone)]
pub enum SceneError {
    DuplicateScene(String)
}

impl fmt::Display for SceneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SceneError::DuplicateScene(ref e) => write!(f, "A scene with the same name '{}' already exists.", e)
        }
    }
}

impl error::Error for SceneError {
}
