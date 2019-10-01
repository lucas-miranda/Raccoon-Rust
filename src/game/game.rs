#[allow(unused_imports)]
use log::{ debug, error, info, trace, warn };

use crate::{
    game::{
        GameError
    },
    input::{
        UserInput
    },
    math::{
        Triangle
    },
    rendering::{
        //HalState,
        Renderer
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
    pub window: Window
}

impl Game {
    pub fn new() -> Result<Self, GameError> {
        simple_logger::init().unwrap();
        info!("~ Raccoon Rust ~");

        info!("Creating Window...");
        let window = Window::default();

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
            window
        })
    }

    pub fn start(&mut self) {
        info!("Creating Renderer...");
        let mut renderer = match Renderer::new(&self.window) {
            Ok(renderer) => renderer,
            Err(e) => panic!(e)
        };

        info!("Starting...");

        loop {
            let inputs = UserInput::poll_events_loop(&mut self.window.events_loop);
            if inputs.end_requested {
                break;
            }

            if inputs.new_frame_size.is_some() {
                debug!("Window changed size, restarting Renderer...");
                std::mem::drop(renderer);
                renderer = match Renderer::new(&self.window) {
                    Ok(renderer) => renderer,
                    Err(e) => panic!(e)
                };
            }

            self.update_from_input(inputs);

            if let Err(e) = self.render(&mut renderer) {
                error!("Rendering Error: {:?}", e);
                debug!("Auto-restarting HalState...");
                std::mem::drop(renderer);
                renderer = match Renderer::new(&self.window) {
                    Ok(renderer) => renderer,
                    Err(e) => panic!(e)
                };
            }
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) -> Result<(), &'static str> {
        /*
        let r = (self.mouse_x / self.frame_width) as f32;
        let g = (self.mouse_y / self.frame_height) as f32;
        let b = (r + g) * 0.3;
        let a = 1.0;
        self._hal_state.draw_clear_frame([r, g, b, a])
        */

        let x = ((self.mouse_x / self.frame_width) * 2.0) - 1.0;
        let y = ((self.mouse_y / self.frame_height) * 2.0) - 1.0;
        let triangle = Triangle {
            points: [
                [-0.5, 0.5], 
                [-0.5, -0.5], 
                [x as f32, y as f32]
            ]
        };

        renderer.draw_triangle_frame(triangle)
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
