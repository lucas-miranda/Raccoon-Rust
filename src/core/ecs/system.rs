use std::{
    any::Any,
    borrow::BorrowMut
};

use crate::{
    core::{
        ecs::{
            Component,
            SystemDataContainer
        },
        GameController
    }
};

pub trait System {
    type DataType: SystemDataContainer;

    fn setup(&mut self, game_controller: &mut GameController);
    fn run(&mut self, game_controller: &mut GameController);
    fn handle<'a>(&mut self, component_type: &Self::DataType, game_controller: &mut GameController);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
