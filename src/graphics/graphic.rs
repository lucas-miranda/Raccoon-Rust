use crate::{
    graphics::Drawable,
    rendering::backends::ResourceDisposable
};

pub trait Graphic : Drawable + ResourceDisposable {
}
