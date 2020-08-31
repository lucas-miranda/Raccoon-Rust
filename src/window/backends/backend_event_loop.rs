use std::{
    cell::RefCell,
    rc::{ Rc, Weak }
};


use crate::{
    core::{
        ecs::Realm,
        GameLoopInterface,
        GameState
    },
    rendering::Renderer
};

pub trait BackendEventLoop<L: GameLoopInterface> {
    fn run(self, game_loop: L);
    //fn poll_events(&mut self);
    //fn redirect_input_events<T, H: InputEventsIndirectHandler<T>>(&mut self, handler: &mut H, listeners: Vec<T>);
}
