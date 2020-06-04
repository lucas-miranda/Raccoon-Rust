use std::{
    any::Any
};

use crate::{
    core::{
        System,
        entity::{
            Entity
        }
    }
};

pub trait Component {
    fn added(&self, entity: &mut Entity);
    fn removed(&self, entity: &Entity);
    fn update(&mut self, entity: &mut Entity, system: &System);
    fn render(&self, entity: &Entity);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
