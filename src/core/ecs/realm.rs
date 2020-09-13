use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{
        hash_map::{
            Values
        },
        HashMap
    },
    rc::Weak
};

use crate::{
    core::{
        ecs::{
            containers::{
                SystemDataContainer
            },
            AnySystem,
            Component,
            Entity,
            EntityId,
            EntityBuilder,
            System,
        },
        GameState
    },
    events::{
        Event,
        EventListener
    },
    input::{
        InputEvent,
    },
    window::{
        WindowEvent,
    }
};

pub struct Realm {
    pub(in crate::core) game_state: Weak<RefCell<GameState>>,
    systems: HashMap<String, AnySystem>,
    entities: Option<HashMap<EntityId, Entity>>,
    next_entity_id: EntityId
}

impl EventListener<InputEvent> for Realm {
    fn notify(&mut self, event: &mut Event<InputEvent>) {
        match event.kind() {
            InputEvent::MouseButton(e) => {
                println!(
                    "state: {:?}, button: {:?}, modifiers: {:?}",
                    e.state,
                    e.button,
                    e.modifiers
                );
            },
            InputEvent::Keyboard(e) => {
                println!(
                    "scan code: {:?}, state: {:?}, key: {:?}, modifiers: {:?}",
                    e.scan_code,
                    e.state,
                    e.key,
                    e.modifiers
                );
            },
            InputEvent::MouseWheel { delta, phase, modifiers } => {
                println!(
                    "mouse wheel (delta: {:?}, phase: {:?}, modifiers: {:?})",
                    delta,
                    phase,
                    modifiers
                );
            },
            _ => ()
        }
    }
}

impl EventListener<WindowEvent> for Realm {
    fn notify(&mut self, event: &mut Event<WindowEvent>) {
    }
}

impl Realm {
    pub fn new() -> Realm {
        Realm {
            game_state: Weak::new(),
            systems: HashMap::new(),
            entities: Some(HashMap::new()),
            next_entity_id: 0u64
        }
    }

    pub fn setup_systems(&mut self) {
        match self.game_state.upgrade() {
            Some(ref mut game_state) => {
                for system in self.systems.values_mut() {
                    system.setup(game_state.borrow().borrow_mut());
                }
            },
            None => ()
        }
    }

    pub fn run_systems(&mut self) {
        let mut entities_map = self.entities.take();

        match entities_map {
            Some(ref mut entities) => {
                match self.game_state.upgrade() {
                    Some(ref mut game_state) => {
                        for system in self.systems.values_mut() {
                            system.run(entities, game_state.borrow().borrow_mut());
                        }
                    },
                    None => ()
                }
            },
            None => panic!("Entities not found.")
        }

        self.entities = entities_map;
    }

    pub fn run_system<T: Into<String>>(&mut self, label: T) {
        let mut entities_map = self.entities.take();

        match entities_map {
            Some(ref mut entities) => {
                match self.game_state.upgrade() {
                    Some(ref mut game_state) => {
                        let l = label.into();
                        match self.systems.get_mut::<String>(&l) {
                            Some(s) => s.run(entities, game_state.borrow().borrow_mut()),
                            None => panic!("System with label '{}' not found.", l)
                        }
                    },
                    None => ()
                }
            },
            None => panic!("Entities not found.")
        }

        self.entities = entities_map;
    }

    pub fn upkeep(&self) {
    }

    pub fn iter_systems(&self) -> Values<String, AnySystem> {
        self.systems.values()
    }

    pub fn get_system<'a, S: 'static + System, T: Into<String>>(&'a self, label: T) -> Option<&'a S> {
        match self.systems.get::<String>(&label.into()) {
            Some(any_system) => any_system.get_system::<S>(),
            None => None
        }
    }

    pub fn get_mut_system<'a, S: 'static + System, T: Into<String>>(&'a mut self, label: T) -> Option<&'a mut S> {
        match self.systems.get_mut::<String>(&label.into()) {
            Some(any_system) => any_system.get_mut_system::<S>(),
            None => None
        }
    }

    pub fn register_system<K: Into<String>, U: 'static + SystemDataContainer, T: 'static + System<DataType = U>>(&mut self, label: K, system: T) {
        self.systems.insert(label.into(), AnySystem::new(system));
    }

    pub fn register_component<C: Component>(&mut self) {
    }

    pub fn add_entity(&mut self, mut entity: Entity) {
        let mut entities_map = self.entities.take();

        if let Some(entities) = &mut entities_map {
            if entities.contains_key(&entity.get_id()) {
                // get a new id to entity
                entity.id = self.next_entity_id();
            }

            println!("Adding entity with id: {}", entity.id);
            entities.insert(entity.id, entity);
        } else {
            panic!("Entities collection is missing.")
        }

        self.entities = entities_map;
    }

    pub fn create_entity<'a>(&'a mut self) -> EntityBuilder<'a> {
        let builder = EntityBuilder::new(self.next_entity_id(), self);
        builder
    }

    fn next_entity_id(&mut self) -> EntityId {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        entity_id
    }
}
