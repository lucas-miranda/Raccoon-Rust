use std::collections::{
    hash_map::{
        Values
    },
    HashMap
};

use super::{
    AnySystem,
    Component,
    Entity,
    System,
    SystemDataContainer
};

pub struct Realm<'s> {
    systems: HashMap<String, AnySystem<'s>>,
    entities: Vec<Entity>
}

impl<'s> Realm<'s> {
    pub fn new<'a>() -> Realm<'a> {
        Realm {
            systems: HashMap::new(),
            entities: Vec::new()
        }
    }

    pub fn run_systems(&'s mut self) {
        for system in self.systems.values_mut() {
            // TODO get requirements
            // TODO check

            // run
            for entity in self.entities.iter() {
                system.try_run(entity.get_components());
            }
        }
    }

    pub fn upkeep(&self) {
    }

    pub fn iter_systems(&self) -> Values<String, AnySystem<'s>> {
        self.systems.values()
    }

    pub fn get_system<S: System + 'static, T: Into<String>>(&self, label: T) -> Option<&S> {
        match self.systems.get::<String>(&label.into()) {
            Some(any_system) => any_system.get_system().downcast_ref::<S>(),
            None => None
        }
    }

    pub fn get_mut_system<S: System + 'static, T: Into<String>>(&mut self, label: T) -> Option<&mut S> {
        match self.systems.get_mut::<String>(&label.into()) {
            Some(any_system) => any_system.get_mut_system().downcast_mut::<S>(),
            None => None
        }
    }

    pub fn register_system<K: Into<String>, U: Component + 'static, T: System<DataType = SystemDataContainer<'s, U>> + 'static>(&mut self, label: K, system: T) {
        self.systems.insert(label.into(), AnySystem::new(system));
    }

    pub fn register_component<C: Component>(&mut self) {
    }

    pub fn build_entity() {
    }
}
