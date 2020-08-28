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
        GameState,
    },
    input::InputEventListener,
    math::{
        Size
    },
    window::{
        backends::{
            Backend,
            BackendInterface,
            InputEventsHandler,
            InputEventsIndirectHandler,
            WindowEventsHandler
        },
        WindowEventListener
    }
};

pub struct Window {
    backend: Rc<RefCell<Backend>>
    //title: String
}

impl Default for Window {
    fn default() -> Self {
        Self::new(
            "Default Window",
            Size::with(480, 320)
        )
        .expect("Could not create a window!")
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let backend = <_ as Borrow<RefCell<Backend>>>::borrow(&self.backend)
                                                      .borrow();

        backend.raw_window_handle()
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

impl Window {
    pub fn new<T: Into<String>>(title: T, size: Size<u32>) -> Result<Self, ()> {
        match Backend::new(title, size) {
            Ok(backend) => Ok(
                Window {
                    backend: Rc::new(RefCell::new(backend)),
                }
            ),
            Err(e) => Err(())
        }
    }

    pub fn new_backend_weak_ref(&self) -> Weak<RefCell<Backend>> {
        Rc::downgrade(&self.backend)
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
