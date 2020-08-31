use std::{
    borrow::Borrow,
    cell::{ RefCell, RefMut },
    rc::{ Rc, Weak }
};

use raw_window_handle::{
    HasRawWindowHandle,
    RawWindowHandle
};

use crate::{
    core::{
        ecs::Realm,
        GameLoopInterface,
        GameState,
    },
    math::{
        Size
    },
    window::{
        backends::{
            backend,
            BackendEventLoop,
            BackendInterface,
            BackendWindow
        }
    }
};

pub struct Window<L: GameLoopInterface> {
    backend: backend::Backend<L>
    //title: String
}

impl<L: 'static + GameLoopInterface> Default for Window<L> {
    fn default() -> Self {
        Self::new(
            "Default Window",
            Size::with(480, 320)
        )
        .expect("Could not create a window!")
    }
}

unsafe impl<L: 'static + GameLoopInterface> HasRawWindowHandle for Window<L> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.backend.window().raw_window_handle()
    }
}

/*
impl<T: InputEventListener> InputEventsHandler<T> for Window {
    fn handle(&mut self, listener: &mut T) {
        <Backend as InputEventsHandler<T>>::handle(&mut self.backend, listener);
    }

    fn handle_multiple(&mut self, listeners: Vec<T>) {
        <Backend as InputEventsHandler<T>>::handle_multiple(&mut self.backend, listeners);
    }
}

impl InputEventsHandler<Box<&mut dyn InputEventListener>> for Window {
    fn handle(&mut self, listener: &mut Box<&mut dyn InputEventListener>) {
        <Backend as InputEventsHandler<Box<&mut dyn InputEventListener>>>::handle(&mut self.backend, listener);
    }

    fn handle_multiple(&mut self, listeners: Vec<Box<&mut dyn InputEventListener>>) {
        <Backend as InputEventsHandler<Box<&mut dyn InputEventListener>>>::handle_multiple(&mut self.backend, listeners);
    }
}

impl<T: WindowEventListener> WindowEventsHandler<T> for Window {
    fn handle(&mut self, listener: &mut T) {
        <Backend as WindowEventsHandler<T>>::handle(&mut self.backend, listener);
    }

    fn handle_multiple(&mut self, listeners: Vec<T>) {
        <Backend as WindowEventsHandler<T>>::handle_multiple(&mut self.backend, listeners);
    }
}

impl WindowEventsHandler<Box<&mut dyn WindowEventListener>> for Window {
    fn handle(&mut self, listener: &mut Box<&mut dyn WindowEventListener>) {
        <Backend as WindowEventsHandler<Box<&mut dyn WindowEventListener>>>::handle(&mut self.backend, listener);
    }

    fn handle_multiple(&mut self, listeners: Vec<Box<&mut dyn WindowEventListener>>) {
        <Backend as WindowEventsHandler<Box<&mut dyn WindowEventListener>>>::handle_multiple(&mut self.backend, listeners);
    }
}
*/

impl<L: 'static + GameLoopInterface> Window<L> {
    pub fn new<T: Into<String>>(title: T, size: Size<u32>) -> Result<Self, ()> {
        match backend::Backend::new(title, size) {
            Ok(backend) => Ok(
                Window {
                    backend,
                }
            ),
            Err(e) => Err(())
        }
    }

    pub fn inner_size(&self) -> Size<u32> {
        self.backend.window().inner_size()
    }

    pub fn event_loop(&mut self) -> <backend::Backend<L> as BackendInterface<L>>::EventLoop {
        self.backend.event_loop()
    }

    /*
    pub fn run(&mut self, game_state: Rc<RefCell<GameState>>, realm: Realm) {
        self.backend.run();
    }
    */

    /*
    pub fn redirect_input_events<T, H: InputEventsIndirectHandler<T>>(&mut self, handler: &mut H, listeners: Vec<T>) {
        self.backend.redirect_input_events(handler, listeners);
    }
    */

    /*
    pub fn get_title(&self) -> &String {
        &self.title
    }
    */
}
