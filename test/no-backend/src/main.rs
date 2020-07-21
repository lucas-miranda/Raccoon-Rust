use raccoon_rust::{
    core::{
        Game,
        scene::{
            Scene
        }
    }
};

/*
mod player;
mod scenes;

use crate::{
    scenes::{
        GameplaySceneComponent
    }
};
*/

fn main() {
    println!("No backend test starting");

    match Game::new() {
        Ok(mut game) => {
            let realm = Realm::new();

            // components
            //realm.register_component<Graphic>();

            // entities
            realm.create_entity()
                 .with_component(Transform::new())
                 .with_component(Graphic::new())
                 .build();

            game.start(realm);
        },
        Err(e) => panic!(e)
    };

    println!("No backend test end!");
}
