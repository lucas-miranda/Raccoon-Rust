use std::{
    cell::RefCell,
    rc::Weak
};

use crate::{
    core::{
        ecs::Realm,
        GameState,
    },
    events::EventHandler,
    input::InputEvent,
    rendering::Renderer,
    window::WindowEvent
};

pub trait GameLoopInterface : EventHandler<InputEvent> + EventHandler<WindowEvent> {
    fn new(realm: Realm, renderer: Renderer, game_state: Weak<RefCell<GameState>>) -> Self;
    fn step(&mut self, redraw_request: &mut bool);
    fn update(&mut self);
    fn render(&mut self);
    fn game_state(&self) -> Weak<RefCell<GameState>>;
}
