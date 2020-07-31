use std::any::Any;

use crate::{
    core::{
        ecs::{
            Component,
            System,
            SystemDataContainer
        },
        GameController
    }
};

pub struct AnySystem {
    sys: Option<Box<dyn Any>>,
    setup: Box<dyn FnMut(&mut Box<dyn Any>, &mut GameController)>,
    filter: Box<dyn Fn(&Vec<Box<dyn Component>>) -> Vec<usize>>,
    runner: Box<dyn FnMut(&mut Box<dyn Any>, &mut GameController)>,
    handler: Box<dyn FnMut(&mut Box<dyn Any>, &mut Vec<Box<dyn Component>>, &mut GameController)>
}

type ComponentCollection = Vec<Box<dyn Component>>;
impl AnySystem {
    pub fn new<U: 'static + SystemDataContainer, T: System<DataType = U> + Any>(system: T) -> AnySystem {
        AnySystem {
            sys: Some(Box::new(system)),
            filter: Box::new(|components| {
                let mut indices = Vec::new();

                let mut i = 0usize;
                for c in components.iter() {
                    if c.as_any().is::<U>() {
                        indices.push(i);
                    }

                    i += 1;
                }

                indices
            }),
            setup: Box::new(|s, game_utilities| {
                match s.downcast_mut::<T>() {
                    Some(reconstructed_system) => reconstructed_system.setup(game_utilities),
                    None => panic!("Can't reconstruct system type.")
                };
            }),
            runner: Box::new(|s, game_utilities| {
                match s.downcast_mut::<T>() {
                    Some(reconstructed_system) => reconstructed_system.run(game_utilities),
                    None => panic!("Can't reconstruct system type.")
                };
            }),
            handler: Box::new(|s, components, game_utilities| {
                let mut container = U::try_from(components);
                /*
                let mut i = 0;

                for c in components.iter() {
                    container.set(
                        i, 
                        c.as_any()
                         .downcast_ref::<U>()
                         .expect("Expecting a concrete type.")
                    );

                    i += 1;
                }

                match s.downcast_mut::<T>() {
                    Some(reconstructed_system) => reconstructed_system.handle(&container, game_utilities),
                    None => panic!("Can't reconstruct system type.")
                };
                */

                // TODO return components from container
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

    pub fn run(&mut self, game_utilities: &mut GameController) {
        let mut sys = self.sys.take();

        match &mut sys {
            Some(s) => (self.runner)(s, game_utilities),
            None => panic!("Impossible to run, system is None.")
        }

        self.sys = sys;
    }

    pub fn handle<'a>(&mut self, components: &'a mut Vec<Box<dyn Component>>, game_utilities: &mut GameController) {
        let indices = (self.filter)(components);
        if indices.is_empty() {
            return;
        }

        let mut requirements = Vec::new();
        // TODO  remove at indexes reversed sort order
        for index in indices.iter() {
            requirements.push(components.remove(*index));
        }

        let mut sys = self.sys.take();

        match &mut sys {
            Some(s) => (self.handler)(s, &mut requirements, game_utilities),
            None => panic!("Impossible to run, system is None.")
        }

        self.sys = sys;

        // TODO  return components
    }

    pub fn get_system(&self) -> &dyn Any {
        match self.sys {
            Some(ref system) => system.as_ref(),
            None => panic!("Impossible to run, system is None.")
        }
    }

    pub fn get_mut_system(&mut self) -> &mut dyn Any {
        match &mut self.sys {
            Some(system) => system.as_mut(),
            None => panic!("Impossible to run, system is None.")
        }
    }
}
