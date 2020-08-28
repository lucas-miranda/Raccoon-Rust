use std::{
    cell::RefCell,
    rc::{ Rc, Weak }
};

use raw_window_handle::HasRawWindowHandle;

use crate::{
    core::{
        ecs::Realm,
        GameState,
    },
    window::backends::InputEventsIndirectHandler
};

pub trait BackendInterface : HasRawWindowHandle {
    fn run(&mut self, game_state: Weak<RefCell<GameState>>, realm: Realm);
    //fn poll_events(&mut self);
    //fn redirect_input_events<T, H: InputEventsIndirectHandler<T>>(&mut self, handler: &mut H, listeners: Vec<T>);
}
