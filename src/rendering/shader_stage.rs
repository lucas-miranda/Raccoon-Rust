use std::{
    fmt::{
        self,
        Display,
        Formatter
    }
};

#[derive(Debug, PartialEq)]
pub enum ShaderStage {
    Vertex,
    Hull,
    Domain,
    Geometry,
    Fragment,
    Compute,
    Task,
    Mesh
}

impl Display for ShaderStage {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

