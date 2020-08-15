use winit::{
    dpi::{
        LogicalSize
    },
    CreationError,
    EventsLoop,
    //Event
};

use crate::{
    events::Event,
    input::{
        InputEvent,
        InputEventListener,
    },
    math::{
        Size,
        Vector2
    },
    window::{
        backends::{
            BackendInterface,
            InputEventsHandler,
            WindowEventsHandler
        },
        WindowEvent,
        WindowEventListener
    }
};

pub struct WinitBackend {
    events_loop: EventsLoop,
    winit_window: winit::Window,
    window_events: Vec<Event<WindowEvent>>,
    input_events: Vec<Event<InputEvent>>
}

impl BackendInterface for WinitBackend {
    fn poll_events(&mut self) {
        let window_events = &mut self.window_events;
        self.events_loop.poll_events(|e| {
            match e {
                winit::Event::WindowEvent { window_id: _, event } => {
                    match event {
                        winit::WindowEvent::Resized(new_size) => {
                            let size: (u32, u32) = new_size.into();
                            window_events.push(Event::new(WindowEvent::Resized(Size::from(size))));
                        },
                        winit::WindowEvent::Moved(new_pos) => {
                            let pos: (i32, i32) = new_pos.into();
                            window_events.push(Event::new(WindowEvent::Moved(Vector2::from(pos))));
                        },
                        winit::WindowEvent::CloseRequested => {
                            window_events.push(Event::new(WindowEvent::CloseRequested));
                        },
                        winit::WindowEvent::Destroyed => {
                            window_events.push(Event::new(WindowEvent::Destroyed));
                        },
                        winit::WindowEvent::DroppedFile(file_path) => {
                            window_events.push(Event::new(WindowEvent::DroppedFile(file_path)));
                        },
                        winit::WindowEvent::HoveredFile(file_path) => {
                            window_events.push(Event::new(WindowEvent::HoveredFile(file_path)));
                        },
                        winit::WindowEvent::HoveredFileCancelled => {
                            window_events.push(Event::new(WindowEvent::HoveredFileCancelled));
                        },
                        winit::WindowEvent::Focused(received_focus) => {
                            window_events.push(Event::new(WindowEvent::Focused(received_focus)));
                        },
                        winit::WindowEvent::Refresh => {
                            window_events.push(Event::new(WindowEvent::Refresh));
                        },
                        winit::WindowEvent::HiDpiFactorChanged(dpi) => {
                            window_events.push(Event::new(WindowEvent::HiDpiFactorChanged(dpi)));
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        });
    }
}

impl<T: InputEventListener> InputEventsHandler<T> for WinitBackend {
    fn handle(&mut self, listener: &mut T) {
        for event in self.input_events.iter_mut() {
            listener.handle(event);
        }

        self.input_events.clear();
    }

    fn handle_multiple(&mut self, mut listeners: Vec<T>) {
        for event in self.input_events.iter_mut() {
            for listener in listeners.iter_mut() {
                listener.handle(event);
            }
        }

        self.input_events.clear();
    }
}

impl InputEventsHandler<Box<&mut dyn InputEventListener>> for WinitBackend {
    fn handle(&mut self, listener: &mut Box<&mut dyn InputEventListener>) {
        for event in self.input_events.iter_mut() {
            listener.handle(event);
        }

        self.input_events.clear();
    }

    fn handle_multiple(&mut self, mut listeners: Vec<Box<&mut dyn InputEventListener>>) {
        for event in self.input_events.iter_mut() {
            for listener in listeners.iter_mut() {
                listener.handle(event);
            }
        }

        self.input_events.clear();
    }
}

impl<T: WindowEventListener> WindowEventsHandler<T> for WinitBackend {
    fn handle(&mut self, listener: &mut T) {
        for event in self.window_events.iter_mut() {
            listener.handle(event);
        }

        self.window_events.clear();
    }

    fn handle_multiple(&mut self, mut listeners: Vec<T>) {
        for event in self.window_events.iter_mut() {
            for listener in listeners.iter_mut() {
                listener.handle(event);
            }
        }

        self.window_events.clear();
    }
}

impl WindowEventsHandler<Box<&mut dyn WindowEventListener>> for WinitBackend {
    fn handle(&mut self, listener: &mut Box<&mut dyn WindowEventListener>) {
        for event in self.window_events.iter_mut() {
            listener.handle(event);
        }

        self.window_events.clear();
    }

    fn handle_multiple(&mut self, mut listeners: Vec<Box<&mut dyn WindowEventListener>>) {
        for event in self.window_events.iter_mut() {
            for listener in listeners.iter_mut() {
                listener.handle(event);
            }
        }

        self.window_events.clear();
    }
}

impl WinitBackend {
    pub fn new<T: Into<String>>(window_title: T, size: Size<u32>) -> Result<Self, CreationError> {
        let events_loop = EventsLoop::new();

        winit::WindowBuilder::new()
              .with_title(&window_title.into())
              .with_dimensions(LogicalSize { width: size.width().into(), height: size.height().into() })
              .build(&events_loop)
              .map(|winit_window| {
                  Self {
                      events_loop,
                      winit_window,
                      window_events: Vec::new(),
                      input_events: Vec::new()
                  }
              })
    }
}
