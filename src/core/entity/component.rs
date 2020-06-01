use std::any::Any;

use crate::{
    core::{
        entity::{
            Entity
        }
    },
    input::{
        Input
    }
};

pub trait Component {
    fn added(&self, entity: &mut Entity);
    fn removed(&self, entity: &Entity);
    fn update_input(&mut self, input: &mut Input, entity: &mut Entity);
    fn update(&mut self, entity: &mut Entity);
    fn render(&self, entity: &Entity);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
