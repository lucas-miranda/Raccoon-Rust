use crate::{
    core::{
        Updatable,
        Renderable,
        entity::{
            Entity
        },
        scene::{
            Scene
        }
    }
};

pub trait SceneComponent: Updatable + Renderable {
    fn before_initialize(&self);
    fn initialize(&mut self, scene: &mut Scene);
    fn late_initialize(&self);
    fn entering(&self);
    fn leaving(&self);
    fn entity_added(&self, entity: &Entity);
    fn entity_removed(&self, entity: &Entity);
    fn scene_component_added(&self, scene_component: &dyn SceneComponent);
    fn scene_component_removed(&self, scene_component: &dyn SceneComponent);
}
