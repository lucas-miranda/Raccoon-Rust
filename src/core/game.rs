use crate::{
    core::{
        ecs::{
            systems::{
                GameSystem,
                //RenderingSystem
            },
            Realm
        },
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

pub struct Game<'r> {
    realm: Option<Realm<'r>>
}

impl<'r> Game<'r> {
    pub fn new() -> Result<Game<'r>, GameError> {
        Ok(Game { 
            realm: None
        })
    }

    pub fn start<'a: 'r>(&'a mut self, realm: Realm<'r>) {
        self.realm = Some(realm);
        self.run();
    }

    fn run<'a: 'r>(&'a mut self) {
        let realm = &mut self.realm;

        loop {
            match realm.as_mut().unwrap().get_mut_system::<GameSystem, _>("game") {
                Some(game_system) => {
                    //game_system.try_run();
                    if !game_system.is_running() {
                        break;
                    }

                    game_system.step_timer();
                    //println!("dt: {:?}, et: {:?}", delta_time, self.system.get_timer());
                },
                None => break
            };

            realm.as_mut().unwrap().run_systems();

            /*
            for system in realm.iter_systems() {
                system.try_run();
            }
            */

            /*
            let rendering_system = realm.get_system("rendering");
            rendering_system.run();
            */

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

impl<'r> Drop for Game<'r> {
    fn drop(&mut self) {
    }
}
