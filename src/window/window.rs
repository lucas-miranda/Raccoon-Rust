use crate::{
    input::InputEventListener,
    math::{
        Size
    },
    window::{
        backends::{
            Backend,
            BackendInterface,
            InputEventsHandler,
            WindowEventsHandler
        },
        WindowEventListener
    }
};

pub struct Window {
    backend: Backend
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

impl Window {
    pub fn new<T: Into<String>>(title: T, size: Size<u32>) -> Result<Self, ()> {
        match Backend::new(title, size) {
            Ok(backend) => Ok(Window {
                               backend,
                           }),
            Err(e) => Err(())
        }
    }

    pub fn backend(&self) -> &Backend {
        &self.backend
    }

    pub fn poll_events(&mut self) {
        self.backend.poll_events();
    }

    /*
    pub fn get_title(&self) -> &String {
        &self.title
    }
    */

    /*
    pub fn poll_events(&mut self) {
        let mut events_loop = self.events_loop.take()
                                              .expect("Events loop not found.");

        events_loop.poll_events(|e| self.handle_event(e));
        self.events_loop = Some(events_loop);
    }

    pub fn subscribe_event<T: 'static + WindowEvent, C: 'static + Fn(&mut T)>(&mut self, callback: C) {
    }
    */

    /*
    pub fn subscribe_event<T: 'static + WindowEvent, C: 'static + Fn(&mut T)>(&mut self, callback: C) {
        if let Some(event_notifier) = self.notifiers.get_mut(&TypeId::of::<T>()) {
            event_notifier.subscribe(callback);
        } else {
            let mut event_notifier = LazyEventNotifier::new::<T>();
            event_notifier.subscribe(callback);
            self.notifiers.insert(TypeId::of::<T>(), event_notifier);
        }
    }
    */

    /*
    fn handle_event(&mut self, e: winit::Event) {
        match e {
            winit::Event::WindowEvent { window_id: _, event: window_event } => {
                match window_event {
                    winit::WindowEvent::Resized(new_size) => {
                        let s: (u32, u32) = new_size.into();

                        self.notifiers.get_mut(&TypeId::of::<WindowResizedEvent>())
                                      .unwrap()
                                      .notify(WindowResizedEvent::new(Size::from(s)))
                    },
                    winit::WindowEvent::Moved(pos) => {
                        let p: (i32, i32) = pos.into();

                        self.notifiers.get_mut(&TypeId::of::<WindowMovedEvent>())
                                      .unwrap()
                                      .notify(WindowMovedEvent::new(Vector2::from(p)))
                    },
                    winit::WindowEvent::CloseRequested => {
                        self.notifiers.get_mut(&TypeId::of::<WindowCloseRequestedEvent>())
                                      .unwrap()
                                      .notify(WindowCloseRequestedEvent::new())
                    },
                    winit::WindowEvent::Destroyed => {
                        self.notifiers.get_mut(&TypeId::of::<WindowDestroyedEvent>())
                                      .unwrap()
                                      .notify(WindowDestroyedEvent::new())
                    },
                    winit::WindowEvent::DroppedFile(path) => {
                        self.notifiers.get_mut(&TypeId::of::<WindowDroppedFileEvent>())
                                      .unwrap()
                                      .notify(WindowDroppedFileEvent::new(path))
                    },
                    winit::WindowEvent::HoveredFile(path) => {
                        self.notifiers.get_mut(&TypeId::of::<WindowHoveredFileEvent>())
                                      .unwrap()
                                      .notify(WindowHoveredFileEvent::new(path))
                    },
                    winit::WindowEvent::HoveredFileCancelled => {
                        self.notifiers.get_mut(&TypeId::of::<WindowDestroyedEvent>())
                                      .unwrap()
                                      .notify(WindowDestroyedEvent::new())
                    },
                    winit::WindowEvent::Focused(received_focus) => {
                        self.notifiers.get_mut(&TypeId::of::<WindowFocusedEvent>())
                                      .unwrap()
                                      .notify(WindowFocusedEvent::new(received_focus))
                    },
                    winit::WindowEvent::Refresh => {
                        self.notifiers.get_mut(&TypeId::of::<WindowRefreshEvent>())
                                      .unwrap()
                                      .notify(WindowRefreshEvent::new())
                    },
                    winit::WindowEvent::HiDpiFactorChanged(dpi) => {
                        self.notifiers.get_mut(&TypeId::of::<WindowHiDpiFactorChangedEvent>())
                                      .unwrap()
                                      .notify(WindowHiDpiFactorChangedEvent::new(dpi))
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }
    */
}
