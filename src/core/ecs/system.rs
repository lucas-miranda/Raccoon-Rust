use std::any::Any;

use super::{
    Component
};

pub trait System {
    type DataType: Component + 'static;

    fn run<'a>(&mut self, component_type: &SystemDataContainer<'a, Self::DataType>);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}


pub struct SystemDataContainer<'c, T: Component + 'static> {
    pub componentA: Option<&'c T>
}

impl<'c, T: Component + 'static> SystemDataContainer<'c, T> {
    pub fn new<'a>() -> SystemDataContainer<'a, T> {
        SystemDataContainer {
            componentA: None
        }
    }

    pub fn check(&self, index: i32, component: &dyn Component) -> bool {
        if index < 0 || index > 1 {
            panic!("Index out of range, acceptable range is [0, 0]");
        }

        component.as_any().is::<T>()
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        if index < 0 || index > 1 {
            panic!("Index out of range, acceptable range is [0, 0]");
        }

        self.componentA
    }

    pub fn set(&mut self, index: i32, component: &'c T) {
        if index < 0 || index > 1 {
            panic!("Index out of range, acceptable range is [0, 0]");
        }

        self.componentA = Some(component);
    }
}


pub struct AnySystem {
    sys: Option<Box<dyn Any>>,
    filter: Box<dyn Fn(&Vec<Box<dyn Component>>) -> Vec<usize>>,
    runner: Box<dyn FnMut(&mut Box<dyn Any>, &mut Vec<Box<dyn Component>>)>
}

impl AnySystem {
    pub fn new<U: Component + 'static, T: System<DataType = U> + Any + 'static>(mut system: T) -> AnySystem {
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
            runner: Box::new(|s, components| {
                let mut container = SystemDataContainer::<U>::new();
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
                    Some(reconstructed_system) => reconstructed_system.run(&container),
                    None => panic!("Can't reconstruct system type.")
                };

                // TODO return components from container
            })
        }
    }

    pub fn try_run<'a>(&mut self, components: &'a mut Vec<Box<dyn Component>>) {
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
            Some(s) => (self.runner)(s, &mut requirements),
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
