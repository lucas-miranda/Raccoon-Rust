use std::{
    borrow::Borrow,
    cell::RefCell,
    rc::Weak
};

use crate::{
    core::{
        ecs::Realm,
        GameLoopInterface,
        GameState
    },
    events::{
        Event,
        EventHandler,
        EventListener
    },
    input::InputEvent,
    rendering::{
        GraphicsDevice,
        Renderer,
        ResourceDisposable,
        panic_if_resource_isnt_disposed
    },
    window::WindowEvent
};

pub struct GameLoop {
    realm: Realm,
    renderer: Weak<RefCell<Renderer>>,
    game_state: Weak<RefCell<GameState>>,
    finalized: bool
}

impl EventHandler<InputEvent> for GameLoop {
    fn handle(&mut self, event: &mut Event<InputEvent>) {
        self.realm.notify(event);
    }
}

impl EventHandler<WindowEvent> for GameLoop {
    fn handle(&mut self, event: &mut Event<WindowEvent>) {
        self.realm.notify(event);
    }
}

impl GameLoopInterface for GameLoop {
    fn new(realm: Realm, renderer: Weak<RefCell<Renderer>>, game_state: Weak<RefCell<GameState>>) -> Self {
        Self {
            realm,
            renderer,
            game_state,
            finalized: false
        }
    }

    fn step(&mut self, redraw_request: &mut bool) {
        *redraw_request = true;
        self.update();
    }

    fn update(&mut self) {
    }

    fn render(&mut self) {
        self.realm.run_system("rendering");

        /*
        match self.renderer.upgrade() {
            Some(renderer_strong_ref) => {
                let mut renderer = <_ as Borrow<RefCell<Renderer>>>::borrow(&renderer_strong_ref)
                                                                    .borrow_mut();

                renderer.draw_clear_frame([0f32, 1f32, 0f32, 1f32]);
            },
            None => eprintln!("Can't retrieve renderer strong ref (from game loop)")
        }
        */
    }

    fn game_state(&self) -> Weak<RefCell<GameState>> {
        self.game_state.clone()
    }

    fn finalize(&mut self) {
        if self.finalized {
            return;
        }

        self.finalized = true;
        let r = self.renderer.upgrade();

        match r {
            Some(renderer_strong_ref) => {
                let mut renderer = <_ as Borrow<RefCell<Renderer>>>::borrow(&renderer_strong_ref)
                                                                    .borrow_mut();

                let graphics_device = renderer.graphics_device();

                self.realm.dispose(graphics_device);
            },
            None => eprintln!("Can't retrieve renderer strong ref (from game loop)")
        };
    }
}

impl Drop for GameLoop {
    fn drop(&mut self) {
        self.finalize();
    }
}

impl GameLoop {
}
