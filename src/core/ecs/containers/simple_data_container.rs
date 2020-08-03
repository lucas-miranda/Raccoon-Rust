use std::{
    collections::{
        hash_map::Drain,
        HashMap,
    },
    marker::PhantomData
};

use crate::core::ecs::{
    containers::SystemDataContainer,
    Component,
    EntityId,
};

pub struct SimpleDataContainer<T: Component + 'static> {
    components: HashMap<EntityId, Vec<Box<dyn Component>>>,
    phantom: PhantomData<T>
}

impl<T: Component + 'static> SystemDataContainer for SimpleDataContainer<T> {
    type ComponentType = T;

    fn new() -> Self {
        SimpleDataContainer {
            components: HashMap::new(),
            phantom: PhantomData
        }
    }

    fn try_add(&mut self, entity_id: EntityId, components: &mut Vec<Box<dyn Component>>) {
        let mut captured_components = Vec::new();

        let mut index: usize = 0;
        while index != components.len() {
            if let Some(_) = components[index].as_any().downcast_ref::<T>() {
                let component = components.remove(index);
                captured_components.push(component);
            } else {
                index += 1;
            }
        }

        self.components.insert(entity_id, captured_components);
    }

    fn drain(&mut self) -> Drain<EntityId, Vec<Box<dyn Component>>> {
        self.components.drain()
    }
}

impl<T: Component> SimpleDataContainer<T> {
    pub fn get<'a>(&'a self, entity_id: EntityId) -> Result<Vec<&'a T>, &'static str> {
        let boxed_components = match self.components.get(&entity_id) {
            Some(c) => c,
            None => return Err("Boxed component is empty.")
        };

        let mut components = Vec::new();
        for component in boxed_components.iter() {
            components.push(
                component.as_any()
                         .downcast_ref::<T>()
                         .expect("Conversion from boxed component into concrete type is impossible.")
            );
        }

        Ok(components)
    }
}
