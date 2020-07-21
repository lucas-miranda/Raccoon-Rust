use std::any::Any;

use super::{
    Component
};

pub trait System {
    type DataType;

    fn run(&mut self, component_type: Self::DataType);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}


pub struct SystemDataContainer<'a, T: Component + 'static> {
    componentA: Option<&'a T>
}

impl<'a, T: Component + 'static> SystemDataContainer<'a, T> {
    pub fn new() -> SystemDataContainer<'a, T> {
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

    pub fn get(&self, index: i32) -> Option<&'a T> {
        if index < 0 || index > 1 {
            panic!("Index out of range, acceptable range is [0, 0]");
        }

        self.componentA
    }

    pub fn set(&mut self, index: i32, component: &'a T) {
        if index < 0 || index > 1 {
            panic!("Index out of range, acceptable range is [0, 0]");
        }

        self.componentA = Some(component);
    }
}


pub struct AnySystem<'s> {
    sys: Option<Box<dyn Any>>,
    runner: Box<dyn FnMut(&mut Box<dyn Any>, &'s Vec<Box<dyn Component>>) + 's>
}

impl<'s> AnySystem<'s> {
    pub fn new<U: Component + 'static, T: System<DataType = SystemDataContainer<'s, U>> + Any + 'static>(mut system: T) -> AnySystem<'s> {
        AnySystem {
            sys: Some(Box::new(system)),
            runner: Box::new(|s, components| {
                let mut container = SystemDataContainer::<U>::new();
                let mut i = 0;

                for c in components.iter() {
                    if container.check(i, c.as_ref()) {
                        container.set(
                            i, 
                            c.as_any()
                             .downcast_ref::<U>()
                             .expect("Expecting a concrete type.")
                        );

                        i += 1;
                    }
                }

                match s.downcast_mut::<T>() {
                    Some(reconstructed_system) => reconstructed_system.run(container),
                    None => panic!("Can't reconstruct system type.")
                }
            })
        }
    }

    pub fn try_run(&mut self, components: &'s Vec<Box<dyn Component>>) {
        let mut sys = self.sys.take();

        match &mut sys {
            Some(s) => (self.runner)(s, components),
            None => panic!("Impossible to run, system is None.")
        }

        self.sys = sys;
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
