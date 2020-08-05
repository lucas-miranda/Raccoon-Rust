use raccoon_rust::{
    core::{
        Game,
        ecs::{
            components::{
                GraphicRendererComponent,
                TransformComponent
            },
            Realm
        },
    },
    graphics::Image
};

fn main() {
    println!("No backend test starting");

    match Game::new() {
        Ok(mut game) => {
            let mut realm = Realm::new();

            // components
            //realm.register_component<Graphic>();

            // systems

            // entities
            realm.create_entity()
                 .with_component(TransformComponent::new())
                 .with_component(GraphicRendererComponent::new())
                 .build();

            realm.create_entity()
                 .with_component(TransformComponent::new())
                 .with_component(GraphicRendererComponent::new())
                 .build();

            let image = Image::new();

            game.start(realm);
        },
        Err(e) => panic!(e)
    };

    println!("No backend test end!");
}
