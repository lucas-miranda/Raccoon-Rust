use std::{
    borrow::Borrow,
    cell::{ RefCell, RefMut },
    rc::Rc
};

use crate::{
    core::{
        ecs::{
            systems::{
                RenderingSystem,
                UpdateSystem
            },
            Realm
        },
        GameLoop,
        GameLoopInterface,
        GameState,
        GameInitError,
        GameRuntimeError
    },
    rendering::Renderer,
    window::{
        backends::{
            BackendEventLoop,
            BackendInterface
        },
        Window
    }
};

pub struct Game<L: GameLoopInterface = GameLoop> {
    game_state: Rc<RefCell<GameState>>,
    renderer: Option<Renderer>,
    window: Window<L>
}

impl Game {
    pub fn new() -> Result<Game<GameLoop>, GameInitError> {
        let window = Window::default();
        let renderer = Renderer::new(Some(&window))
                                .map_err(|e| GameInitError::RendererCreation(e))?;

        Ok(Game { 
            game_state: Rc::new(RefCell::new(GameState::new())),
            renderer: Some(renderer),
            window
        })
    }
}

impl<L: 'static + GameLoopInterface> Game<L> {
    pub fn with_custom_loop<T: 'static + GameLoopInterface>() -> Result<Game<T>, GameInitError> {
        let window = Window::default();
        let renderer = Renderer::new(Some(&window))
                                .map_err(|e| GameInitError::RendererCreation(e))?;

        Ok(Game { 
            game_state: Rc::new(RefCell::new(GameState::new())),
            renderer: Some(renderer),
            window
        })
    }

    pub fn run(&mut self, mut realm: Realm) -> Result<(), GameRuntimeError> {
        let renderer = Rc::new(RefCell::new(
            self.renderer
                .take()
                .ok_or(GameRuntimeError::RendererNotAvailable)?
        ));

        realm.game_state = Rc::downgrade(&self.game_state);

        // register default systems
        realm.register_system("update", UpdateSystem::new());
        realm.register_system("rendering", RenderingSystem::new(Rc::downgrade(&renderer)));

        realm.setup_systems();
        <_ as Borrow<RefCell<GameState>>>::borrow(&self.game_state)
                                          .borrow_mut()
                                          .start();

        // run window event loop
        self.window
            .event_loop()
            .run(L::new(realm, Rc::downgrade(&renderer), Rc::downgrade(&self.game_state)));
        
        // dispose managed resources
        // those which depends on renderer backend


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

        Ok(())
    }

    pub fn renderer(&self) -> &Option<Renderer> {
        &self.renderer
    }

    pub fn mut_renderer(&mut self) -> &mut Option<Renderer> {
        &mut self.renderer
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
    */
}
