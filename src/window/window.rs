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
    pub fn get_title(&self) -> &String {
        &self.title
    }
    */
}
