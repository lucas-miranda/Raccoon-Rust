use std::{
    borrow::Borrow,
    cell::{ RefCell, RefMut },
    rc::Rc
};

use crate::{
    core::{
        ecs::{
            systems::{
                UpdateSystem
            },
            Realm
        },
        GameState,
        GameError
    },
    window::{
        backends::BackendInterface,
        Window
    }
};

pub struct Game {
    game_state: Rc<RefCell<GameState>>,
    window: Window
}

impl Game {
    pub fn new() -> Result<Game, GameError> {
        Ok(Game { 
            game_state: Rc::new(RefCell::new(GameState::new())),
            window: Window::default()
        })
    }

    pub fn run(&mut self, mut realm: Realm) {
        realm.game_state = Rc::downgrade(&self.game_state);

        // register default systems
        realm.register_system("update", UpdateSystem::new());

        realm.setup_systems();
        <_ as Borrow<RefCell<GameState>>>::borrow(&self.game_state)
                                          .borrow_mut()
                                          .start();

        // 

        match self.window.new_backend_weak_ref().upgrade() {
            Some(ref window) => {
                <_ as Borrow<RefCell<crate::window::backends::Backend>>>::borrow(window)
                      .borrow_mut()
                      .run(Rc::downgrade(&self.game_state), realm);
            },
            None => ()
        }

        /*
        loop {
            {
                let mut game_state = self.game_state();

                game_state.poll_events();
                game_state.handle_window_events(&mut realm);
                game_state.handle_input(&mut realm);

                if !game_state.is_running() {
                    break;
                }
            }

            realm.run_systems();
            //realm.upkeep();
        }
        */
    }

    /*
    fn game_state(&self) -> RefMut<'_, GameState> {
        <_ as Borrow<RefCell<GameState>>>::borrow(&self.game_state).borrow_mut()
    }
    */

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
