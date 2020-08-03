use super::Component;

pub type EntityId = u64;

pub struct Entity {
    pub(super) id: EntityId,
    components: Vec<Box<dyn Component>>
}

impl Entity {
    pub fn new(id: EntityId) -> Entity {
        Entity {
            id,
            components: Vec::new()
        }
    }

    pub fn get_id(&self) -> EntityId {
        self.id
    }

    pub fn get_components(&self) -> &Vec<Box<dyn Component>> {
        &self.components
    }

    pub fn get_mut_components(&mut self) -> &mut Vec<Box<dyn Component>> {
        &mut self.components
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T) {
        self.components.push(Box::new(component))
    }
}
