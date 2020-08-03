use std::{
    any::Any,
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

    fn setup(&mut self, game_controller: &mut GameController);
    fn run(&mut self, component_type: &mut Self::DataType, game_controller: &mut GameController);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
