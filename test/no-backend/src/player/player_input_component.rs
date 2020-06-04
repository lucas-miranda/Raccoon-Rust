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
            require_component
        }
    },
    input::{
        Input,
        KeyCode
    }
};

pub struct PlayerInputComponent {
}

impl Component for PlayerInputComponent {
    fn added(&self, entity: &mut Entity) {
        require_component!(entity, TransformComponent);
    }

    fn removed(&self, entity: &Entity) {
    }

    fn update(&mut self, entity: &mut Entity, system: &System) {
        let transform_component = entity.get_mut_component::<TransformComponent>().unwrap();

        if system.input.key(KeyCode::A).unwrap().is_pressed() {
            transform_component.change_x(-5f32);
            println!("Player is moving to the left! (x: {})", transform_component.x());
        }
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

impl PlayerInputComponent {
    pub fn new() -> PlayerInputComponent {
        PlayerInputComponent {
        }
    }
}
