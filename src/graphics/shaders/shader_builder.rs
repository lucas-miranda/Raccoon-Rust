use std::{
    fs
};

use shaderc;

use crate::{
    graphics::shaders::{
        Shader,
        ShaderBuildError,
        ShaderBuilderInitError
    },
    rendering::{
        backend::{
            RendererBackendInterface
        },
        GraphicsDevice
    }
};

pub struct ShaderBuilder {
    compiler: shaderc::Compiler
}

impl ShaderBuilder {
    pub fn new() -> Result<Self, ShaderBuilderInitError> {
        let compiler = match shaderc::Compiler::new() {
            Some(c) => Ok(c),
            None => Err(ShaderBuilderInitError::CompilerCreation)
        }?;

        Ok(Self {
            compiler
        })
    }
    
    pub fn shader_from_files(&mut self, vertex_filepath: &str, fragment_filepath: &str, device: &GraphicsDevice) -> Result<Shader, ShaderBuildError> {
        let vertex_contents = fs::read_to_string(vertex_filepath)
                                 .map_err(|e| ShaderBuildError::VertexFileRead(e))?;

        let fragment_contents = fs::read_to_string(fragment_filepath)
                                   .map_err(|e| ShaderBuildError::FragmentFileRead(e))?;

        let vertex_artifact = self.compiler
            .compile_into_spirv(
                &vertex_contents,
                shaderc::ShaderKind::Vertex,
                vertex_filepath,
                "main",
                None
            )
            .map_err(|e| ShaderBuildError::VertexCompilation(e))?;

        let fragment_artifact = self.compiler
            .compile_into_spirv(
                &fragment_contents,
                shaderc::ShaderKind::Fragment,
                fragment_filepath,
                "main",
                None
            )
            .map_err(|e| ShaderBuildError::FragmentCompilation(e))?;

        Ok(Shader::new(
            vertex_artifact.as_binary().to_owned(),
            fragment_artifact.as_binary().to_owned(),
            device
        ))
    }
}
