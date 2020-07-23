use super::Component;

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
