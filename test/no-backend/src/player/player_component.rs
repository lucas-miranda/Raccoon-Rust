use std::{
    any::Any,
    time::Duration
};

use raccoon_rust::{
    core::{
        System,
        entity::{
            Component,
            Entity,
            TransformComponent,
            register_component,
            register_unique_component
        }
    },
    input::{
        Input
    }
};

use crate::{
    player::{
        PlayerInputComponent
    }
};

pub struct PlayerComponent {
}

impl Component for PlayerComponent {
    fn added(&self, entity: &mut Entity) {
        register_unique_component!(
            entity,
            TransformComponent, 
            PlayerInputComponent
        );
    }

    fn removed(&self, entity: &Entity) {
    }

    fn update(&mut self, delta_time: &Duration, entity: &mut Entity, system: &System) {
    }

    fn render(&self, entity: &Entity) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl PlayerComponent {
    pub fn new() -> PlayerComponent {
        PlayerComponent {
        }
    }
}
