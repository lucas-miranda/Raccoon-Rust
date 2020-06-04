use std::time::Duration;

use crate::{
    core::{
        System,
        entity::{
            Entity
        },
        scene::{
            SceneComponent
        }
    }
};

pub struct Scene {
    pub name: String,
    _components: Option<Vec<Box<dyn SceneComponent>>>,
    _entities: Vec<Entity>
}

impl Scene {
    pub fn new(name: &'static str) -> Self {
        Self {
            name: name.to_owned(),
            _components: Some(Vec::new()),
            _entities: Vec::new()
        }
    }

    pub fn before_initialize(&mut self) {
        println!("Scene -> before initialize");
        let mut c = self._components.take(); 

        match &mut c {
            Some(components) => {
                for component in components.iter_mut() {
                    component.before_initialize();
                }
            },
            None => ()
        }

        self._components = c;
    }

    pub fn initialize(&mut self) {
        println!("Scene -> initialize");
        let mut c = self._components.take(); 

        match &mut c {
            Some(components) => {
                for component in components.iter_mut() {
                    component.initialize(self);
                }
            },
            None => ()
        }

        self._components = c;
    }

    pub fn late_initialize(&mut self) {
        println!("Scene -> late initialize");
        let mut c = self._components.take(); 

        match &mut c {
            Some(components) => {
                for component in components.iter_mut() {
                    component.late_initialize();
                }
            },
            None => ()
        }

        self._components = c;
    }

    pub fn entering(&self) {
        println!("Entering scene")
    }

    pub fn leaving(&self) {
    }

    pub fn update(&mut self, system: &System) {
        match &mut self._components {
            Some(components) => {
                for component in components.iter_mut() {
                    component.update(system);
                }
            },
            None => ()
        }

        for entity in self._entities.iter_mut() {
            entity.update(system);
        }
    }

    pub fn render(&self) {
        match &self._components {
            Some(components) => {
                for component in components.iter() {
                    component.render();
                }
            },
            None => ()
        }

        for entity in self._entities.iter() {
            entity.render();
        }
    }

    pub fn add_component<T: SceneComponent + 'static>(&mut self, component: T) {
        match &mut self._components {
            Some(components) => components.push(Box::new(component)),
            None => return ()
        }

        match &self._components {
            Some(components) => self.scene_component_added(components.last().unwrap()),
            None => ()
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self._entities.push(entity);
        self.entity_added(self._entities.last().unwrap());
    }

    pub fn remove_entity(&mut self, entity: &Entity) -> bool {
        use std::ptr;

        let mut i = 0;
        for e in self._entities.iter() {
            if ptr::eq(e, entity) {
                let removed_entity = self._entities.remove(i);
                self.entity_removed(&removed_entity);
                return true;
            }

            i += 1;
        }

        false
    }

    fn entity_added(&self, entity: &Entity) {
    }

    fn entity_removed(&self, entity: &Entity) {
    }

    fn scene_component_added(&self, scene_component: &Box<dyn SceneComponent>) {
    }

    fn scene_component_removed(&self, scene_component: &Box<dyn SceneComponent>) {
        unimplemented!();
    }
}
