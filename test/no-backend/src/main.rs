use raccoon_rust::{
    core::{
        Game,
        scene::{
            Scene
        }
    }
};

mod player;
mod scenes;

use crate::{
    scenes::{
        GameplaySceneComponent
    }
};

fn main() {
    println!("No backend test starting");

    match Game::new() {
        Ok(mut game) => {
            let mut gameplay_scene = Scene::new("gameplay scene");
            gameplay_scene.add_component(GameplaySceneComponent::new());

            game.scene_director.insert(gameplay_scene).unwrap();
            game.scene_director.play("gameplay scene");
            game.start();
        },
        Err(e) => panic!(e)
    };

    println!("No backend test end!");
}
