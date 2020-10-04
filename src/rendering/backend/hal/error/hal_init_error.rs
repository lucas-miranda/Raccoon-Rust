use std::{
    error::{
        Error
    },
    fmt::{
        self,
        Display,
        Formatter
    }
};

use gfx_hal::{
    device,
    pso,
    window
};

#[derive(Debug)]
pub enum HalInitError {
    AllocationDescriptorSetFromPool(pso::AllocationError),
    CommandDescriptorPoolCreation(device::OutOfMemory),
    CommandPoolCreation(device::OutOfMemory),
    DescriptorPoolCreation(device::OutOfMemory),
    DescriptorSetLayoutCreation(device::OutOfMemory),
    Device(device::CreationError),
    DeviceRenderPassCreation(device::OutOfMemory),
    ExpectedQueueFamilyNotFound,
    FenceCreation(device::OutOfMemory),
    GpuQueueGroupNotFound,
    GraphicalAdapterNotFound,
    NoCommandQueuesAtQueueGroup,
    SemaphoreCreation(device::OutOfMemory),
    SwapchainConfigureCreation(window::CreationError),
    UnsupportedBackend,
    WindowHandle(window::InitError)
}

impl Display for HalInitError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HalInitError::UnsupportedBackend => {
                write!(fmt, "Can't create backend instance")
            },
            HalInitError::WindowHandle(err) => {
                write!(fmt, "Error with window handle: {}", err)
            },
            HalInitError::GraphicalAdapterNotFound => {
                write!(fmt, "Couldn't find a graphical adapter with a valid queue family.")
            },
            HalInitError::ExpectedQueueFamilyNotFound => {
                write!(fmt, "Couldn't find a queue family which supports graphics.")
            },
            HalInitError::Device(err) => {
                write!(fmt, "Couldn't create a logical device: {}", err)
            },
            HalInitError::GpuQueueGroupNotFound => {
                write!(fmt, "Can't find queue group from expected queue family at Gpu.")
            },
            HalInitError::NoCommandQueuesAtQueueGroup => {
                write!(fmt, "Queue group didn't have any command queue available.")
            },
            HalInitError::CommandDescriptorPoolCreation(err) => {
                write!(fmt, "Command descriptor pool can't be created: {}", err)
            },
            HalInitError::DescriptorSetLayoutCreation(err) => {
                write!(fmt, "Descriptor set layout can't be created: {}", err)
            },
            HalInitError::DescriptorPoolCreation(err) => {
                write!(fmt, "Descriptor pool can't be created: {}", err)
            },
            HalInitError::AllocationDescriptorSetFromPool(err) => {
                write!(fmt, "Descriptor set layout can't be allocated from pool: {}", err)
            },
            HalInitError::SwapchainConfigureCreation(err) => {
                write!(fmt, "Swapchain configuration raised an error: {}", err)
            },
            HalInitError::DeviceRenderPassCreation(err) => {
                write!(fmt, "Device render pass can't be created: {}", err)
            },
            HalInitError::CommandPoolCreation(err) => {
                write!(fmt, "Command pool can't be created: {}", err)
            },
            HalInitError::SemaphoreCreation(err) => {
                write!(fmt, "Semaphore can't be created: {}", err)
            },
            HalInitError::FenceCreation(err) => {
                write!(fmt, "Fence can't be created: {}", err)
            },
        }
    }
}

impl Error for HalInitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HalInitError::WindowHandle(err) => Some(err),
            HalInitError::Device(err) => Some(err),
            HalInitError::CommandDescriptorPoolCreation(err) => Some(err),
            HalInitError::DescriptorSetLayoutCreation(err) => Some(err),
            HalInitError::DescriptorPoolCreation(err) => Some(err),
            HalInitError::AllocationDescriptorSetFromPool(err) => Some(err),
            HalInitError::SwapchainConfigureCreation(err) => Some(err),
            HalInitError::DeviceRenderPassCreation(err) => Some(err),
            HalInitError::CommandPoolCreation(err) => Some(err),
            HalInitError::SemaphoreCreation(err) => Some(err),
            HalInitError::FenceCreation(err) => Some(err),
            _ => None
        }
    }
}
