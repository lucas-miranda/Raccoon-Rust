use crate::{
    graphics::Graphic,
    rendering::{
        BackendInterface
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
    fn has_texture_available(&self) -> bool {
        false
    }

    fn draw<T: Graphic>(&self, _graphic: &T) {
        // can't draw anything :/
    }
}
