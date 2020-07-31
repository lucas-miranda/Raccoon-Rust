use super::{
    Component,
    SystemDataContainer
};

pub struct SimpleDataContainer<T: Component + 'static> {
    component: Option<Box<dyn Component>>,
    phantom: PhantomData<T>
}

impl<T: Component + 'static> SystemDataContainer for SimpleDataContainer<T> {
    type ComponentType = T;

    fn new(component: Self::ComponentType) -> Self {
        SimpleDataContainer {
            component: Some(Box::new(component)),
            phantom: PhantomData
        }
    }

    fn try_from(components: &mut Vec<Box<dyn Component>>) -> Result<Self, &'static str> {
        if components.len() != 1 {
            return Err("");
        }

        let component = components.remove(0);

        component.as_any()
                 .downcast_ref::<T>()
                 .expect("Can't perform conversion from component to concrete type.");

        Ok(SimpleDataContainer {
            component: Some(component),
            phantom: PhantomData
        })
    }

    fn take(&mut self) -> Vec<Option<Box<dyn Component>>> {
        vec!(self.component.take())
    }
}

impl<T: Component> SimpleDataContainer<T> {
    pub fn get<'a>(&'a self) -> Result<&'a T, &'static str> {
        let boxed_component = match &self.component {
            Some(c) => c,
            None => return Err("Boxed component is empty.")
        };

        boxed_component.as_any()
                       .downcast_ref::<T>()
                       .ok_or("Conversion from boxed component into concrete type is impossible.")
    }
}
