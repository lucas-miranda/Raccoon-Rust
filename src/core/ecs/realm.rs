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

pub struct Realm {
    systems: HashMap<String, AnySystem>,
    entities: Option<Vec<Entity>>
}

impl Realm {
    pub fn new() -> Realm {
        Realm {
            systems: HashMap::new(),
            entities: Some(Vec::new())
        }
    }

    pub fn run_systems(&mut self) {
        let mut entities = self.entities.take();

        match entities {
            Some(ref mut e) => {
                for system in self.systems.values_mut() {
                    // TODO get requirements
                    // TODO check

                    // run
                    for entity in e.iter_mut() {
                        system.try_run(entity.get_mut_components());
                    }
                }
            },
            None => panic!("Entities not found.")
        }

        self.entities = entities;
    }

    pub fn upkeep(&self) {
    }

    pub fn iter_systems(&self) -> Values<String, AnySystem> {
        self.systems.values()
    }

    pub fn get_system<'a, S: System + 'static, T: Into<String>>(&'a self, label: T) -> Option<&'a S> {
        match self.systems.get::<String>(&label.into()) {
            Some(any_system) => any_system.get_system().downcast_ref::<S>(),
            None => None
        }
    }

    pub fn get_mut_system<'a, S: System + 'static, T: Into<String>>(&'a mut self, label: T) -> Option<&'a mut S> {
        match self.systems.get_mut::<String>(&label.into()) {
            Some(any_system) => any_system.get_mut_system().downcast_mut::<S>(),
            None => None
        }
    }

    pub fn register_system<K: Into<String>, U: Component + 'static, T: System<DataType = U> + 'static>(&mut self, label: K, system: T) {
        self.systems.insert(label.into(), AnySystem::new(system));
    }

    pub fn register_component<C: Component>(&mut self) {
    }

    pub fn build_entity() {
    }
}
