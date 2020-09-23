use crate::{
    graphics::Drawable,
    rendering::ResourceDisposable
};

pub trait Graphic : Drawable + ResourceDisposable {
}
