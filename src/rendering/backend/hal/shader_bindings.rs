use core::mem::{
    ManuallyDrop
};

use gfx_hal::{
    device::Device,
    pso
};

use crate::{
    rendering::{
        backend::{
            RendererBackend,
            RendererBackendInterface,
        },
        GraphicsDevice,
        ResourceDisposable,
        panic_if_resource_isnt_disposed
    }
};

type InternalBackend = <RendererBackend as RendererBackendInterface>::InternalBackend;

pub struct ShaderBindings {
    descriptors_set_layout: Vec::<pso::DescriptorSetLayoutBinding>,
    descriptors_pool: Vec::<pso::DescriptorRangeDesc>,
    sampler: ManuallyDrop<<InternalBackend as gfx_hal::Backend>::Sampler>,
    disposed: bool
}

impl ResourceDisposable for ShaderBindings {
    fn is_disposed(&self) -> bool {
        self.disposed
    }

    fn dispose(&mut self, device: &GraphicsDevice) {
        if self.disposed {
            return;
        }

        self.disposed = true;
        let device_handle = device.backend().device();

        unsafe {
            device_handle.destroy_sampler(ManuallyDrop::take(&mut self.sampler));
        }
    }
}

impl Drop for ShaderBindings {
    fn drop(&mut self) {
        panic_if_resource_isnt_disposed!(self);
    }
}

impl ShaderBindings {
    pub fn new(device: &GraphicsDevice) -> Self {
        Self {
            descriptors_set_layout: Vec::new(),
            descriptors_pool: Vec::new(),
            sampler: ManuallyDrop::new(
                         unsafe {
                             device.backend().device().create_sampler(
                                 &gfx_hal::image::SamplerDesc::new(
                                     gfx_hal::image::Filter::Linear,
                                     gfx_hal::image::WrapMode::Clamp
                                 )
                             )
                         }
                         .expect("Can't create sampler.")
                     ),
            disposed: false
        }
    }

    pub fn sampler(&self) -> &<InternalBackend as gfx_hal::Backend>::Sampler {
        &*self.sampler
    }
}
