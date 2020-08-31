use std::{
    cell::RefCell,
    rc::Weak
};

use crate::{
    events::Event,
    input::{
        Input,
    },
    tools::{
        log::Logger
    },
    window::{
        Window,
        WindowEvent
    }
};

pub struct GameState {
    input: Input,
    logger: Logger, 
    //window: Weak<RefCell<Window>>,
    is_running: bool
}

/*
impl WindowEventListener for GameState {
    fn handle(&mut self, event: &mut Event<WindowEvent>) {
        match event.kind() {
            WindowEvent::CloseRequested => {
                self.close_game();
                println!("Window close requested.");
                event.consume();
            },
            _ => ()
        }
    }
}
*/

impl GameState {
    pub fn new() -> GameState {
        GameState {
            input: Input::new(),
            logger: Logger::new(),
            //window: Some(window),
            is_running: false
        }
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    /*
    pub fn window(&self) -> &Option<Window> {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Option<Window> {
        &mut self.window
    }
    */

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn close_game(&mut self) {
        self.is_running = false;
    }

    pub(in crate::core) fn start(&mut self) {
        self.is_running = true;
    }

    /*
    pub(in crate::core) fn poll_events(&mut self) {
        match self.window {
            Some(ref mut window) => window.poll_events(),
            None => ()
        }
    }

    pub(in crate::core) fn handle_window_events<T: WindowEventListener>(&mut self, listener: &mut T) {
        let mut window = self.window.take();

        match window {
            Some(ref mut window) => {
                let mut listeners: Vec<Box<&mut dyn WindowEventListener>> = Vec::new();
                listeners.push(Box::new(self));
                listeners.push(Box::new(listener));

                <Window as WindowEventsHandler<_>>::handle_multiple(window, listeners);
            },
            None => ()
        }

        self.window = window;
    }

    pub(in crate::core) fn handle_input<T: InputEventListener>(&mut self, listener: &mut T) {
        match self.window {
            Some(ref mut window) => {

                let mut listeners: Vec<Box<&mut dyn InputEventListener>> = Vec::new();
                //listeners.push(Box::new(&mut self.input));
                listeners.push(Box::new(listener));

                //<Window as InputEventsHandler<_>>::handle_multiple(window, listeners);
                window.redirect_input_events(&mut self.input, listeners);
            },
            None => ()
        };
    }
    */

    /*
    pub(in crate::core) fn handle_events<T: WindowEventListener>(&mut self, listener: T) {
        let mut events_loop = self.window.events_loop.take();

        events_loop.poll_events(|e| {
            match e {
                winit::Event::WindowEvent { window_id: _, event } => {
                    match event {
                        winit::WindowEvent::Resized(new_size) => {
                            let s: (u32, u32) = new_size.into();
                            listener.handle(WindowEvent::Resize(Size::from(s)));
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        });

        self.window.events_loop = events_loop;
    }
    */
}
