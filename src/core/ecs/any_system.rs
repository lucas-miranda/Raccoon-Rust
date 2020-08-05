use std::{
    any::Any,
    collections::{
        HashMap,
    }
};

use crate::{
    core::{
        ecs::{
            containers::{
                SystemDataContainer
            },
            Entity,
            EntityId,
            System,
        },
        GameController
    }
};

pub struct AnySystem {
    sys: Option<Box<dyn Any>>,
    setup: Box<dyn FnMut(&mut Box<dyn Any>, &mut GameController)>,
    runner: Box<dyn FnMut(&mut Box<dyn Any>, &mut HashMap<EntityId, Entity>, &mut GameController)>
}

impl AnySystem {
    pub fn new<U: 'static + SystemDataContainer, T: System<DataType = U> + Any>(system: T) -> AnySystem {
        AnySystem {
            sys: Some(Box::new(system)),
            setup: Box::new(|s, game_utilities| {
                match s.downcast_mut::<T>() {
                    Some(reconstructed_system) => reconstructed_system.setup(game_utilities),
                    None => panic!("Can't reconstruct system type.")
                };
            }),
            runner: Box::new(|s, entities, game_utilities| {
                let mut container = U::new();

                // prepare container
                for (entity_id, entity) in entities.iter_mut() {
                    container.try_add(*entity_id, entity.get_mut_components());
                }

                // call system
                match s.downcast_mut::<T>() {
                    Some(reconstructed_system) => reconstructed_system.run(&mut container, game_utilities),
                    None => panic!("Can't reconstruct system type.")
                };

                // return each borrowed components to it's own entity
                for (entity_id, mut components) in container.drain() {
                    match entities.get_mut(&entity_id) {
                        Some(entity) => entity.get_mut_components()
                                              .extend(components.drain(..)),

                        None => eprintln!("Entity (with id: {}) not found! Can't return borrowed components to it's entity.", entity_id)
                    };
                }
            })
        }
    }

    pub fn setup(&mut self, game_utilities: &mut GameController) {
        let mut sys = self.sys.take();

        match &mut sys {
            Some(s) => (self.setup)(s, game_utilities),
            None => panic!("Impossible to setup, system is None.")
        }

        self.sys = sys;
    }

    pub fn run<'a>(&mut self, entities: &'a mut HashMap<EntityId, Entity>, game_utilities: &mut GameController) {
        let mut sys = self.sys.take();

        match &mut sys {
            Some(s) => (self.runner)(s, entities, game_utilities),
            None => panic!("Impossible to run, system is None.")
        }

        self.sys = sys;
    }

    pub fn get_underlying_system(&self) -> &dyn Any {
        match self.sys {
            Some(ref system) => system.as_ref(),
            None => panic!("Impossible to run, system is None.")
        }
    }

    pub fn get_mut_underlying_system(&mut self) -> &mut dyn Any {
        match &mut self.sys {
            Some(system) => system.as_mut(),
            None => panic!("Impossible to run, system is None.")
        }
    }

    pub fn get_system<T: 'static + System>(&self) -> Option<&T> {
        match self.sys {
            Some(ref system) => system.as_ref().downcast_ref::<T>(),
            None => panic!("Impossible to run, system is None.")
        }
    }

    pub fn get_mut_system<T: 'static + System>(&mut self) -> Option<&mut T> {
        match self.sys {
            Some(ref mut system) => system.as_mut().downcast_mut::<T>(),
            None => panic!("Impossible to run, system is None.")
        }
    }
}
