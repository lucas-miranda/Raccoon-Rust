use enumflags2::BitFlags;
use crate::{
    rendering::{
        BackendInterface,
        RenderingRequirements
    }
};

pub struct NoBackend {
}

impl NoBackend {
    pub fn new() -> Result<Self, &'static str> {
        Ok(Self { })
    }
}

impl BackendInterface for NoBackend {
    fn name() -> &'static str {
        "NoBackend"
    }

    fn has_requirements(requirements: RenderingRequirements) -> bool {
        if BitFlags::from(requirements).contains(RenderingRequirements::Texture) {
            // doesn't supports: 
            //   * Texture
            return false;
        }

        true
    }
}
