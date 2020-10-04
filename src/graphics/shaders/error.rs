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

#[derive(Debug)]
pub enum ShaderBuilderInitError {
    CompilerCreation
}

impl Display for ShaderBuilderInitError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ShaderBuilderInitError::CompilerCreation => {
                write!(fmt, "Shader compiler can't be created.")
            }
        }
    }
}

impl Error for ShaderBuilderInitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum ShaderBuildError {
    VertexFileRead(std::io::Error),
    FragmentFileRead(std::io::Error),
    VertexCompilation(shaderc::Error),
    FragmentCompilation(shaderc::Error)
}

impl Display for ShaderBuildError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ShaderBuildError::VertexFileRead(err) => {
                write!(fmt, "Can't read from vertex filepath. Cause: {}", err)
            },
            ShaderBuildError::FragmentFileRead(err) => {
                write!(fmt, "Can't read from fragment filepath. Cause: {}", err)
            },
            ShaderBuildError::VertexCompilation(err) => {
                write!(fmt, "Couldn't compile vertex shader. Cause: {}", err)
            },
            ShaderBuildError::FragmentCompilation(err) => {
                write!(fmt, "Couldn't compile fragment shader. Cause: {}", err)
            }
        }
    }
}

impl Error for ShaderBuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ShaderBuildError::VertexFileRead(err) => Some(err),
            ShaderBuildError::FragmentFileRead(err) => Some(err),
            ShaderBuildError::VertexCompilation(err) => Some(err),
            ShaderBuildError::FragmentCompilation(err) => Some(err),
            _ => None
        }
    }
}
