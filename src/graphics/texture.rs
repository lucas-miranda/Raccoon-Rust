use std::{
    path::Path
};

use core::mem::{
    ManuallyDrop
};

use crate::{
    rendering::{
        GraphicsDevice,
        RendererBackend,
        RendererBackendInterface,
        ResourceDisposable,
        panic_if_resource_isnt_disposed
    }
};

use crate::{
    math::Size
};

type TextureBindings = <RendererBackend as RendererBackendInterface>::TextureBindings;

pub struct Texture {
    pub bindings: TextureBindings,
    uid: u64,
    disposed: bool
}

impl ResourceDisposable for Texture {
    fn is_disposed(&self) -> bool {
        self.disposed
    }

    fn dispose(&mut self, device: &GraphicsDevice) {
        if self.disposed {
            return;
        }

        self.disposed = true;
        self.bindings.dispose(device);
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        panic_if_resource_isnt_disposed!(self);
    }
}

impl Texture {
    pub fn from_file<P: AsRef<Path>>(filepath: P, device: &mut GraphicsDevice) -> Result<Self, &'static str> {
        let bindings = match TextureBindings::with(filepath, device) {
            Ok(b) => b,
            Err(e) => return Err(e)
        };

        Ok(Self {
            uid: device.next_texture_uid(),
            bindings,
            disposed: false
        })
    }

    pub fn uid(&self) -> u64 {
        self.uid
    }

    pub fn size(&self) -> &Size<u32> {
        self.bindings.size()
    }
}
