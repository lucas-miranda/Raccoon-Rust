use crate::{
    core::{
        Renderable,
        Updatable,
        entity::{
            Component
        }
    }
};

pub struct Entity {
    pub name: String,
    _components: Option<Vec<Box<dyn Component>>>
} 
impl Updatable for Entity {
    fn update(&mut self) {
        let mut components = self._components.take();

        match &mut components {
            Some(c) => {
                for component in c.iter_mut() {
                    component.update(self);
                }
            }
            None => ()
        }

        self._components = components;
    }
}

impl Renderable for Entity {
    fn render(&self) {
        match &self._components {
            Some(c) => {
                for component in c.iter() {
                    component.render(self);
                }
            }
            None => ()
        }
    }
}

impl Entity {
    pub fn new<S: Into<String>>(name: S) -> Entity {
        Entity {
            name: name.into().clone(),
            _components: Some(Vec::new())
        }
    }

    pub fn add_component<C: Component + 'static>(&mut self, component: C) {
        component.added(self);

        match &mut self._components {
            Some(components) => components.push(Box::new(component)),
            None => panic!("Can't add component to entity. Entity's component list is at invalid state.")
        }
    }

    pub fn has_component<C: Component + 'static>(&self) -> bool {
        match &self._components {
            Some(c) => {
                for component in c.iter() {
                    if component.as_any().is::<C>() {
                        return true;
                    }
                }
            }
            None => ()
        }

        false
    }

    pub fn get_component<C: Component + 'static>(&self) -> Option<&C> {
        match &self._components {
            Some(c) => {
                for component in c.iter() {
                    if component.as_any().is::<C>() {
                        return Some(component.as_any().downcast_ref::<C>().unwrap());
                    }
                }
            }
            None => ()
        }

        None
    }

    pub fn get_mut_component<C: Component + 'static>(&mut self) -> Option<&mut C> {
        match &mut self._components {
            Some(c) => {
                for component in c.iter_mut() {
                    if component.as_any().is::<C>() {
                        return Some(component.as_any_mut().downcast_mut::<C>().unwrap());
                    }
                }
            }
            None => ()
        }

        None
    }
}

#[macro_export]
macro_rules! require_component {
    ($entity:expr, $component_type:ty) => {
        if !Entity::has_component::<$component_type>($entity) {
            panic!("Entity '{}' is required to has component '{}'.", $entity.name, stringify!($component_type));
        }
    };

    ($entity:expr, $component_type:ty, $($components_types:ty),+) => {
        require_component!($entity, $component_type);
        require_component!($entity, $($components_types),*);
    };
}

#[macro_export]
macro_rules! register_component {
    ($entity:expr, $component_type:ty) => {
        $entity.add_component(<$component_type>::new());
    };

    ($entity:expr, $component_type:ty, $($components_types:ty),+) => {
        register_component!($entity, $component_type);
        register_component!($entity, $($components_types),*);
    };
}

#[macro_export]
macro_rules! register_unique_component {
    ($entity:expr, $component_type:ty) => {
        if !Entity::has_component::<$component_type>($entity) {
            register_component!($entity, $component_type);
        }
    };

    ($entity:expr, $component_type:ty, $($components_types:ty),+) => {
        register_unique_component!($entity, $component_type);
        register_unique_component!($entity, $($components_types),*);
    };
}
