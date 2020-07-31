use std::{
    any::Any
};

use super::Component;

type ComponentCollection = Vec<Box<dyn Component>>;

pub trait SystemDataContainer {
    type ComponentType;

    fn new(component: Self::ComponentType) -> Self;
    fn try_from(components: &mut ComponentCollection) -> Result<Self, &'static str> where Self: Sized;
    fn take(&mut self) -> Vec<Option<Box<dyn Component>>>;
}

impl<T, C1, U, C2> SystemDataContainer for (T, U) where
  C1: Component + 'static,
  C2: Component + 'static,
  T: SystemDataContainer<ComponentType = C1>,
  U: SystemDataContainer<ComponentType = C2>,
{
    type ComponentType = (C1, C2);

    fn new(component: Self::ComponentType) -> Self {
        (T::new(component.0), U::new(component.1))
    }

    fn try_from(components: &mut ComponentCollection) -> Result<Self, &'static str> {
        if components.len() != 2 {
            return Err("");
        }

        let mut component_b = vec!(components.remove(1));
        let mut u_component = match U::try_from(&mut component_b) {
            Ok(c) => c,
            Err(e) => {
                components.insert(1, component_b.remove(0));
                //return Err(format!("When trying to convert component B:\n{}", e));
                return Err(e);
            }
        };

        let mut component_a = vec!(components.remove(0));
        let t_component = match T::try_from(&mut component_a) {
            Ok(c) => c,
            Err(e) => {
                components.insert(0, component_a.remove(0));
                components.insert(1, u_component.take().remove(0).unwrap());
                //return Err(format!("When trying to convert component A:\n{}", e));
                return Err(e);
            }
        };

        Ok((t_component, u_component))
    }

    fn take(&mut self) -> Vec<Option<Box<dyn Component>>> {
        vec!(
            self.0.take().remove(0), 
            self.1.take().remove(0)
        )
    }
}


/*
pub struct SystemDataContainer<'c, T: Component + 'static> {
    pub component_a: Option<&'c T>
}

impl<'c, T: Component + 'static> SystemDataContainer<'c, T> {
    pub fn new<'a>() -> SystemDataContainer<'a, T> {
        SystemDataContainer {
            component_a: None
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

        self.component_a
    }

    pub fn set(&mut self, index: i32, component: &'c T) {
        if index < 0 || index > 1 {
            panic!("Index out of range, acceptable range is [0, 0]");
        }

        self.component_a = Some(component);
    }
}
*/
