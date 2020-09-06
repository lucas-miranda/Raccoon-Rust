use std::{
    collections::{
        hash_map::{
            Drain,
            Iter,
            IterMut,
            Values,
            ValuesMut
        },
        HashMap,
    }
};

use crate::core::ecs::{
    containers::SystemDataContainer,
    Component,
    EntityId,
};

pub struct AnyDataContainer {
    components: HashMap<EntityId, Vec<Box<dyn Component>>>,
}

impl SystemDataContainer for AnyDataContainer {
    type ComponentType = ();

    fn new() -> Self {
        AnyDataContainer {
            components: HashMap::new()
        }
    }

    fn try_add(&mut self, entity_id: EntityId, components: &mut Vec<Box<dyn Component>>) {
        let mut captured_components = Vec::new();
        captured_components.reserve_exact(components.len());
        captured_components.extend(components.drain(..));
        self.components.insert(entity_id, captured_components);
    }

    fn drain(&mut self) -> Drain<EntityId, Vec<Box<dyn Component>>> {
        self.components.drain()
    }

}

impl AnyDataContainer {
    pub fn get<'a>(&'a self, entity_id: EntityId) -> Result<&'a Vec<Box<dyn Component>>, &'static str> {
        let boxed_components = match self.components.get(&entity_id) {
            Some(c) => c,
            None => return Err("Boxed component is empty.")
        };

        Ok(boxed_components.as_ref())
    }

    pub fn iter(&self) -> Iter<EntityId, Vec<Box<dyn Component>>> {
        self.components.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<EntityId, Vec<Box<dyn Component>>> {
        self.components.iter_mut()
    }

    pub fn components(&self) -> Values<EntityId, Vec<Box<dyn Component>>> {
        self.components.values()
    }

    pub fn components_mut(&mut self) -> ValuesMut<EntityId, Vec<Box<dyn Component>>> {
        self.components.values_mut()
    }
}
