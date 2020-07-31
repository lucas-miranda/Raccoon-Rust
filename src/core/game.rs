use crate::{
    core::{
        ecs::{
            systems::{
                UpdateSystem
            },
            Realm
        },
        GameController,
        GameError
    },
    /*
    rendering::{
        Renderer
    },
    tools::{
        log::{
            StdoutListener,
            log_info
        }
    }
    */
};

pub struct Game {
    realm: Option<Realm>
}

impl Game {
    pub fn new() -> Result<Game, GameError> {
        Ok(Game { 
            realm: None
        })
    }

    pub fn start(&mut self, mut realm: Realm) {
        // register default systems
        realm.register_system("update", UpdateSystem::new());

        self.realm = Some(realm);
        self.run();
    }

    fn run(&mut self) {
        let mut game_controller = GameController::new();
        let mut realm = self.realm.take()
                                  .expect("Realm not found.");

        realm.setup_systems(&mut game_controller);
        game_controller.start();

        while game_controller.is_running() {
            realm.run_systems(&mut game_controller);
            //realm.upkeep();
        }
    }

    /*
    pub fn render(&mut self, renderer: &mut Renderer) -> Result<(), &'static str> {
        /*
        let r = (self.mouse_x / self.frame_width) as f32;
        let g = (self.mouse_y / self.frame_height) as f32;
        let b = (r + g) * 0.3;
        let a = 1.0;
        self._hal_state.draw_clear_frame([r, g, b, a])
        */

        let x = ((self.mouse_x / self.frame_width) * 2.0) - 1.0;
        let y = ((self.mouse_y / self.frame_height) * 2.0) - 1.0;
        let triangle = Triangle {
            points: [
                [-0.5, 0.5], 
                [-0.5, -0.5], 
                [x as f32, y as f32]
            ]
        };

        renderer.draw_triangle_frame(triangle)
    }

    fn update_from_input(&mut self, input: UserInput) {
        if let Some(frame_size) = input.new_frame_size {
            self.frame_width = frame_size.0;
            self.frame_height = frame_size.1;
        }

        if let Some(position) = input.new_mouse_position {
            self.mouse_x = position.0;
            self.mouse_y = position.1;
        }
    }
    */
}

impl Drop for Game {
    fn drop(&mut self) {
    }
}
