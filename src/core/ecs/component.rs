use std::any::Any;

pub use crate::core::ecs::{
    components::Updatable
};


pub trait Component : Updatable {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
