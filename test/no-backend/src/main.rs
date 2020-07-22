use raccoon_rust::{
    core::{
        Game,
        ecs::{
            components::{
                GraphicRendererComponent,
                TransformComponent
            },
            systems::{
                GameSystem,
            },
            Realm
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
            let mut realm = Realm::new();

            // components
            //realm.register_component<Graphic>();

            // systems
            realm.register_system("game", GameSystem::new());

            // entities
            realm.create_entity()
                 .with_component(TransformComponent::new())
                 .with_component(GraphicRendererComponent::new())
                 .build();

            game.start(realm);
        },
        Err(e) => panic!(e)
    };

    println!("No backend test end!");
}
