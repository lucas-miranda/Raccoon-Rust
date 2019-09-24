#[allow(unused_imports)]
use log::{ debug, error, info, trace, warn };

use crate::{
    core::{
        HalState,
        GameError
    },
    input::{
        UserInput
    },
    window::{
        Window
    }
};

pub struct Game {
    pub frame_width: f64,
    pub frame_height: f64,
    pub mouse_x: f64,
    pub mouse_y: f64,
    _window: Window,
    _hal_state: HalState
}

impl Game {
    pub fn new() -> Result<Self, GameError> {
        simple_logger::init().unwrap();
        info!("~ Raccoon Rust ~");

        let window = Window::default();

        let hal_state = match HalState::new(&window.winit_window) {
            Ok(state) => state,
            Err(e) => panic!(e)
        };

        let (frame_width, frame_height) = window
            .winit_window
            .get_inner_size()
            .map(|logical| logical.into())
            .unwrap_or((0.0, 0.0));

        Ok(Self {
            frame_width,
            frame_height,
            mouse_x: 0.0,
            mouse_y: 0.0,
            _window: window,
            _hal_state: hal_state
        })
    }

    pub fn start(&mut self) {
        info!("Starting...");

        loop {
            let inputs = UserInput::poll_events_loop(&mut self._window.events_loop);
            if inputs.end_requested {
                break;
            }

            self.update_from_input(inputs);

            if let Err(e) = self.render() {
                error!("Rendering Error: {:?}", e);
                break;
            }
        }
    }

    pub fn render(&mut self) -> Result<(), &'static str> {
        let r = (self.mouse_x / self.frame_width) as f32;
        let g = (self.mouse_y / self.frame_height) as f32;
        let b = (r + g) * 0.3;
        let a = 1.0;
        self._hal_state.draw_clear_frame([r, g, b, a])
    }

    fn update_from_input(&mut self, input: UserInput) {
        if let Some(frame_size) = input.new_frame_size {
            self.frame_width = frame_size.0;
            self.frame_height = frame_size.1;
        }

        if let Some(position) = input.new_mouse_position {
            self.mouse_x = position.0;
            self.mouse_y = position.1;
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
    }
}
