use std::{
    any::Any,
    cell::Ref
};

use crate::{
    core::{
        ecs::containers::{
            SystemDataContainer
        },
        GameController
    }
};

pub trait System {
    type DataType: SystemDataContainer;

    fn setup(&mut self, game_controller: &mut Ref<GameController>);
    fn run(&mut self, component_type: &mut Self::DataType, game_controller: &mut Ref<GameController>);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
