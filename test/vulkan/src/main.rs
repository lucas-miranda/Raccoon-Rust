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
    graphics::{
        Image,
        Texture
    }
};

fn main() {
    println!("Vulkan test starting");

    match Game::new() {
        Ok(mut game) => {
            let mut realm = Realm::new();

            // components
            //realm.register_component<Graphic>();

            // systems

            // entities
            let graphic_renderer_component = {
                let renderer =  match game.mut_renderer() {
                    Some(renderer) => renderer,
                    None => panic!("Renderer isn't available.")
                };

                let graphics_device = renderer.mut_graphics_device();
                let texture = Texture::from_file("src/image-test.png", graphics_device).unwrap();

                let mut renderer_component = GraphicRendererComponent::new();
                renderer_component.register(Box::new(Image::new(texture)));

                renderer_component
            };

            realm.create_entity()
                 .with_component(TransformComponent::new())
                 .with_component(graphic_renderer_component)
                 .build();

            //

            game.run(realm);
        },
        Err(e) => panic!(e)
    };

    println!("Vulkan test end!");
}
