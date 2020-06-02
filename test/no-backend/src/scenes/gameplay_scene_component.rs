use std::time::Duration;

use raccoon_rust::{
    core::{
        Renderable,
        Updatable,
        entity::{
            Entity,
        },
        scene::{
            Scene,
            SceneComponent
        }
    }
};

use crate::{
    player::{
        PlayerComponent
    }
};

pub struct GameplaySceneComponent {
}

impl Updatable for GameplaySceneComponent {
    fn update(&mut self, delta_time: &Duration) {
        //println!("updating gameplay scene!");
    }
}

impl Renderable for GameplaySceneComponent {
    fn render(&self) {
        //println!("rendering gameplay scene!");
    }
}

impl SceneComponent for GameplaySceneComponent {
    fn before_initialize(&self) {
    }

    fn initialize(&mut self, scene: &mut Scene) {
        // player creation
        let mut player = Entity::new("Player");
        player.add_component(PlayerComponent::new());
        scene.add_entity(player);
    }

    fn late_initialize(&self) {
    }

    fn entering(&self) {
    }

    fn leaving(&self) {
    }

    fn entity_added(&self, entity: &Entity) {
    }

    fn entity_removed(&self, entity: &Entity) {
    }

    fn scene_component_added(&self, scene_component: &dyn SceneComponent) {
    }

    fn scene_component_removed(&self, scene_component: &dyn SceneComponent) {
    }
}

impl GameplaySceneComponent {
    pub fn new() -> GameplaySceneComponent {
        GameplaySceneComponent {
        }
    }
}
