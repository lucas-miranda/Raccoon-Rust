use gfx_hal::{
    adapter::PhysicalDevice
};

use crate::{
    rendering::backend::{
        RendererBackend,
        RendererBackendInterface
    }
};

type InternalBackend = <RendererBackend as RendererBackendInterface>::InternalBackend;

pub struct DeviceAdapterBackend {
    device: <InternalBackend as gfx_hal::Backend>::Device,
    adapter: gfx_hal::adapter::Adapter<InternalBackend>,
}

impl DeviceAdapterBackend {
    pub fn new(device: <InternalBackend as gfx_hal::Backend>::Device, adapter: gfx_hal::adapter::Adapter<InternalBackend>) -> Self {
        Self {
            device,
            adapter
        }
    }

    pub fn device(&self) -> &<InternalBackend as gfx_hal::Backend>::Device {
        &self.device
    }

    pub fn adapter(&self) -> &gfx_hal::adapter::Adapter<InternalBackend> {
        &self.adapter
    }

    pub fn limits(&self) -> gfx_hal::Limits {
        self.adapter
            .physical_device
            .limits()
    }
}
