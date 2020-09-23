use std::any::Any;

pub use crate::{
    core::ecs::{
        components::Updatable
    },
    rendering::{
        ResourceDisposable
    }
};

pub trait Component : Updatable + ResourceDisposable {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
