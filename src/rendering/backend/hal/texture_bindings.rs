use std::{
    iter,
    path::Path,
    ptr
};

use core::mem::{
    ManuallyDrop
};

use image_handler;

use gfx_hal::{
    device::Device
};

use crate::{
    math::Size,
    rendering::{
        backend::{
            RendererBackend,
            RendererBackendInterface
        },
        GraphicsDevice,
        ResourceDisposable,
        panic_if_resource_isnt_disposed
    }
};

use super::{
    error::{
        HalTextureBindingsError
    }
};

type InternalBackend = <RendererBackend as RendererBackendInterface>::InternalBackend;

pub struct TextureBindings {
    data: image_handler::RgbaImage,
    upload_buffer: ManuallyDrop<<InternalBackend as gfx_hal::Backend>::Buffer>,
    upload_memory: Option<ManuallyDrop<<InternalBackend as gfx_hal::Backend>::Memory>>,
    size: Size<u32>,
    row_pitch: u32,
    image_stride: usize,
    disposed: bool
}

impl ResourceDisposable for TextureBindings {
    fn is_disposed(&self) -> bool {
        self.disposed
    }

    fn dispose(&mut self, device: &GraphicsDevice) {
        if self.disposed {
            return;
        }

        self.disposed = true;
        let device_handle = device.backend().device();
        device_handle.wait_idle().unwrap();

        unsafe {
            device_handle.destroy_buffer(ManuallyDrop::take(&mut self.upload_buffer));
            match self.upload_memory {
                Some(ref mut upload_memory) => device_handle.free_memory(ManuallyDrop::take(upload_memory)),
                None => ()
            }
        }
    }
}

impl Drop for TextureBindings {
    fn drop(&mut self) {
        panic_if_resource_isnt_disposed!(self);
    }
}

impl TextureBindings {
    pub fn with<P: AsRef<Path>>(filepath: P, device: &GraphicsDevice) -> Result<Self, HalTextureBindingsError> {
        let dynamic_image = image_handler::open(filepath)
                                          .map_err(|e| HalTextureBindingsError::ImageLoading(e))?;

        let rgba_data = dynamic_image.to_rgba();
        let (img_width, img_height) = rgba_data.dimensions();
        let kind = gfx_hal::image::Kind::D2(img_width as gfx_hal::image::Size, img_height as gfx_hal::image::Size, 1, 1);

        let limits = device.backend().limits();
        let device_handle = device.backend().device();

        let non_coherent_alignment = limits.non_coherent_atom_size as u64;
        let row_alignment_mask = limits.optimal_buffer_copy_pitch_alignment as u32 - 1;
        let image_stride = 4usize;
        let row_pitch = (img_width * image_stride as u32 + row_alignment_mask) & !row_alignment_mask;
        let upload_size = (img_height * row_pitch) as u64;
        let padded_upload_size = ((upload_size + non_coherent_alignment - 1) / non_coherent_alignment) * non_coherent_alignment;

        let mut upload_buffer = ManuallyDrop::new(
            unsafe {
                device_handle.create_buffer(padded_upload_size, gfx_hal::buffer::Usage::TRANSFER_SRC)
            }
            .map_err(|e| HalTextureBindingsError::BufferCreation(e))?
        );

        Ok(Self {
            data: rgba_data,
            upload_buffer,
            upload_memory: None,
            size: Size::with(img_width, img_height),
            row_pitch,
            image_stride,
            disposed: false
        })
    }

    pub fn copy_into_stagging_buffer(&mut self, memory_type_id: gfx_hal::MemoryTypeId, device: &GraphicsDevice) -> &<InternalBackend as gfx_hal::Backend>::Buffer {
        if let Some(_) = self.upload_memory {
            return &*self.upload_buffer;
        }

        let device_handle = device.backend().device();

        let upload_buffer_requirements = unsafe {
            device_handle.get_buffer_requirements(&self.upload_buffer)
        };

        let upload_memory = unsafe {
            let memory = device_handle.allocate_memory(memory_type_id, upload_buffer_requirements.size)
                                      .unwrap();

            device_handle.bind_buffer_memory(&memory, 0, &mut self.upload_buffer)
                         .unwrap();

            let mapping = device_handle.map_memory(&memory, gfx_hal::memory::Segment::ALL)
                                       .unwrap();

            let width = self.size.width() as usize;
            let height = self.size.height() as usize;
            for y in 0..height {
                let row = &(*self.data)[y * width * self.image_stride .. (y + 1) * width * self.image_stride];
                ptr::copy_nonoverlapping(
                    row.as_ptr(),
                    mapping.offset(y as isize * self.row_pitch as isize),
                    width * self.image_stride
                );
            }

            device_handle.flush_mapped_memory_ranges(iter::once((&memory, gfx_hal::memory::Segment::ALL)))
                         .unwrap();

            device_handle.unmap_memory(&memory);
            ManuallyDrop::new(memory)
        };

        self.upload_memory = Some(upload_memory);
        &*self.upload_buffer
    }

    pub fn size(&self) -> &Size<u32> {
        &self.size
    }

    pub fn row_pitch(&self) -> u32 {
        self.row_pitch
    }

    pub fn image_stride(&self) -> usize {
        self.image_stride
    }
}
