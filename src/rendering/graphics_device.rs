use super::{
    RendererBackend,
    RendererBackendInterface
};

type DeviceAdapterBackend = <RendererBackend as RendererBackendInterface>::DeviceAdapterBackend;

pub struct GraphicsDevice {
    backend: DeviceAdapterBackend,
    next_texture_uid: u64
}

impl GraphicsDevice {
    pub fn new(backend: DeviceAdapterBackend) -> Self {
        Self {
            backend,
            next_texture_uid: 1u64
        }
    }

    pub fn backend(&self) -> &DeviceAdapterBackend {
        &self.backend
    }

    pub fn next_texture_uid(&mut self) -> u64 {
        let uid = self.next_texture_uid;
        self.next_texture_uid += 1;
        uid
    }
}
