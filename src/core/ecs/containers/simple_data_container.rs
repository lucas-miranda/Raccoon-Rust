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
    },
    iter::{
        Iterator,
        Map
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

    //pub fn iter<'a>(&self) -> Map<Iter<'_, EntityId, Vec<Box<dyn Component>>>, FnMut((EntityId, Vec<Box<impl Component>>)) -> (EntityId, Vec<&'a T>)> {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (EntityId, Vec<&'a T>)> {
        self.components
            .iter()
            .map(|(k, v)| {
                let mut components = Vec::new();
                for component in v.iter() {
                    components.push(
                        component.as_any()
                                 .downcast_ref::<T>()
                                 .expect("Conversion from boxed component into concrete type is impossible.")
                    );
                }

                (*k, components)
            })
            .into_iter()
    }

    /*
    pub fn iter_mut<'a>(&mut self) -> IterMut<EntityId, Vec<&'a T>> {
        self.components.iter_mut()
    }
    */

    /*
    pub fn components<'a>(&self) -> Values<EntityId, Vec<&'a T>> {
        self.components.values()
    }
    */

    pub fn components_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        self.components
            .values_mut()
            .map(|v| {
                let mut components = Vec::new();
                for component in v.iter_mut() {
                    components.push(
                        component.as_any_mut()
                                 .downcast_mut::<T>()
                                 .expect("Conversion from boxed component into concrete type is impossible.")
                    );
                }

                components
            })
            .flatten()
            .into_iter()
    }
}
