use raccoon_rust::{
    core::{
        System,
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

    fn update(&mut self, system: &System) {
    }

    fn render(&self) {
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
