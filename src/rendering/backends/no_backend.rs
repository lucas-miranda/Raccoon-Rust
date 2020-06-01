use super::BackendInterface;

pub struct NoBackend {
}

impl NoBackend {
    pub fn new() -> Result<Self, &'static str> {
        Ok(Self { })
    }
}

impl BackendInterface for NoBackend {
    fn has_texture_available() -> bool {
        false
    }
}
