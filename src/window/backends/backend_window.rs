use raw_window_handle::HasRawWindowHandle;

use crate::{
    math::Size
};

pub trait BackendWindow : HasRawWindowHandle {
    fn inner_size(&self) -> Size<u32>;
}
