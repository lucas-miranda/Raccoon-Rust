use std::any::Any;

use super::{
    Component,
    SystemDataContainer
};

pub trait System {
    type DataType: Component + 'static;

    fn run(&mut self);
    fn handle<'a>(&mut self, component_type: &SystemDataContainer<'a, Self::DataType>);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
