use std::{
    collections::HashMap
};

use crate::{
    core::{
        System,
        scene::{
            Scene,
            SceneError
        }
    }
};


pub struct SceneDirector {
    _current_scene_name: Option<String>,
    _scenes: HashMap<String, Scene>
}

impl SceneDirector {
    pub fn new() -> Result<Self, &'static str> {
        Ok(Self {
            _current_scene_name: None,
            _scenes: HashMap::new()
        })
    }

    pub fn initialize(&mut self) {
        for scene in self._scenes.values_mut() {
            scene.before_initialize();
        }

        for scene in self._scenes.values_mut() {
            scene.initialize();
        }

        for scene in self._scenes.values_mut() {
            scene.late_initialize();
        }

        let start_scene_name = match &self._current_scene_name {
            Some(scene_name) => scene_name,
            None => panic!("Starting scene not defined.\nBefore initializing, please call play(scene_name).")
        };

        match self.get(start_scene_name) {
            Some(start_scene) => start_scene.entering(),
            None => panic!("Scene with name '{}' not found, when initializing start scene.", start_scene_name)
        }
    }

    pub fn update(&mut self, system: &mut System) {
        match self.current_scene_mut() {
            Some(scene) => scene.update(system),
            None => ()
        }

        let timer = system.get_timer();
        if timer.as_secs() >= 3 {
            println!("Timer test has ended!");
            system.close_game();
        }
    }

    pub fn render(&self) {
        match self.current_scene() {
            Some(scene) => scene.render(),
            None => ()
        }
    }

    pub fn play(&mut self, scene_name: &str) {
        match self.get(scene_name) {
            Some(scene) => {
                self._current_scene_name = Some(scene.name.clone());
            },
            None => panic!("Scene with name '{}' not found.", scene_name)
        }
    }

    pub fn current_scene(&self) -> Option<&Scene> {
        match &self._current_scene_name {
            Some(name) => self.get(&name),
            None => None
        }
    }

    pub fn current_scene_mut(&mut self) -> Option<&mut Scene> {
        let scene_name = match &self._current_scene_name {
            Some(name) => name.clone(),
            None => return None
        };

        self.get_mut(&scene_name)
    }

    pub fn insert(&mut self, scene: Scene) -> Result<&Scene, SceneError> {
        if self._scenes.contains_key(&scene.name) {
            return Err(SceneError::DuplicateScene(scene.name.to_owned()));
        }

        let name = scene.name.clone();
        self._scenes.insert(name.clone(), scene);

        Ok(self._scenes.get(&name).unwrap())
    }
    
    pub fn get(&self, scene_name: &str) -> Option<&Scene> {
        self._scenes.get(scene_name)
    }

    pub fn get_mut(&mut self, scene_name: &str) -> Option<&mut Scene> {
        self._scenes.get_mut(scene_name)
    }

    pub fn contains_with_name(&self, scene_name: &str) -> bool {
        self._scenes.contains_key(scene_name)
    }
}
