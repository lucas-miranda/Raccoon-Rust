use super::{
    GraphicsDevice
};

pub trait ResourceDisposable : Drop {
    fn is_disposed(&self) -> bool;
    fn dispose(&mut self, device: &GraphicsDevice);
}

#[macro_export]
macro_rules! panic_if_resource_isnt_disposed {
    ($self:expr) => {
        if !$self.disposed {
            panic!("Managed resource hasn't been properly disposed.\nPlease, call dispose() at it!");
        }
    };
}
