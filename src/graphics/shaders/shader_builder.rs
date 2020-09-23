use std::{
    fs
};

use shaderc;

use crate::{
    graphics::shaders::{
        Shader
    },
    rendering::{
        GraphicsDevice,
        RendererBackendInterface
    }
};

pub struct ShaderBuilder {
    compiler: shaderc::Compiler
}

impl ShaderBuilder {
    pub fn new() -> Result<Self, &'static str> {
        let compiler = match shaderc::Compiler::new() {
            Some(c) => c,
            None => return Err("Shader compiler can't be created.")
        };

        Ok(Self {
            compiler
        })
    }
    
    pub fn shader_from_files(&mut self, vertex_filepath: &str, fragment_filepath: &str, device: &GraphicsDevice) -> Result<Shader, &'static str> {
        let vertex_contents = match fs::read_to_string(vertex_filepath) {
            Ok(contents) => contents,
            Err(e) => return Err("Can't read from vertex filepath.")
        };

        let fragment_contents = match fs::read_to_string(fragment_filepath) {
            Ok(contents) => contents,
            Err(e) => return Err("Can't read from fragment filepath.")
        };

        let vertex_artifact = self.compiler
            .compile_into_spirv(
                &vertex_contents,
                shaderc::ShaderKind::Vertex,
                vertex_filepath,
                "main",
                None
            )
            .map_err(|_e| "Couldn't compile vertex shader.")?;

        let fragment_artifact = self.compiler
            .compile_into_spirv(
                &fragment_contents,
                shaderc::ShaderKind::Fragment,
                fragment_filepath,
                "main",
                None
            )
            .map_err(|_e| "Couldn't compile fragment shader.")?;

        Ok(Shader::new(
            vertex_artifact.as_binary().to_owned(),
            fragment_artifact.as_binary().to_owned(),
            device
        ))
    }
}
