use gfx_hal::{
    adapter::PhysicalDevice
};

use super::{
    Backend,
    BackendInterface
};

type GfxBackend = <Backend as BackendInterface>::InternalBackend;

pub struct GraphicsDevice {
    device: <GfxBackend as gfx_hal::Backend>::Device,
    adapter: gfx_hal::adapter::Adapter<GfxBackend>,
    next_texture_uid: u64
}

impl GraphicsDevice {
    pub fn new(device: <GfxBackend as gfx_hal::Backend>::Device, adapter: gfx_hal::adapter::Adapter<GfxBackend>) -> Self {
        Self {
            device,
            adapter,
            next_texture_uid: 1u64
        }
    }

    pub fn handle(&self) -> &<GfxBackend as gfx_hal::Backend>::Device {
        &self.device
    }

    pub fn adapter(&self) -> &gfx_hal::adapter::Adapter<GfxBackend> {
        &self.adapter
    }

    pub fn limits(&self) -> gfx_hal::Limits {
        self.adapter
            .physical_device
            .limits()
    }

    pub fn next_texture_uid(&mut self) -> u64 {
        let uid = self.next_texture_uid;
        self.next_texture_uid += 1;
        uid
    }
}
