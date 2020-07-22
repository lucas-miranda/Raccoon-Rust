pub use super::{
    Component,
    Entity,
    Realm
};

pub struct EntityBuilder<'r> {
    entity: Entity,
    realm: &'r mut Realm
}

impl<'r> EntityBuilder<'r> {
    pub fn new<'a>(id: u64, realm: &'a mut Realm) -> EntityBuilder<'a> {
        EntityBuilder {
            entity: Entity::new(id),
            realm
        }
    }

    pub fn with_component<T: Component + 'static>(mut self, component: T) -> EntityBuilder<'r> {
        self.entity.add_component(component);
        self
    }

    pub fn build(self) {
        self.realm.add_entity(self.entity);
    }
}
