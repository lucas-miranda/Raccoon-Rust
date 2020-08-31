use std::{
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
    rendering::Renderer,
    window::WindowEvent
};

pub struct GameLoop {
    realm: Realm,
    renderer: Renderer,
    game_state: Weak<RefCell<GameState>>
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
    fn new(realm: Realm, renderer: Renderer, game_state: Weak<RefCell<GameState>>) -> Self {
        Self {
            realm,
            renderer,
            game_state
        }
    }

    fn step(&mut self, redraw_request: &mut bool) {
        *redraw_request = true;
        self.update();
    }

    fn update(&mut self) {
    }

    fn render(&mut self) {
        self.renderer.draw_clear_frame([0f32, 1f32, 0f32, 1f32]);
    }

    fn game_state(&self) -> Weak<RefCell<GameState>> {
        self.game_state.clone()
    }
}

impl GameLoop {

    /*
    fn get_game_state(&self, strong_ref: ) -> RefMut<'_, GameState> {
        <_ as Borrow<RefCell<GameState>>>::borrow(&strong_ref)
                                          .borrow_mut()
    }
    */
}
