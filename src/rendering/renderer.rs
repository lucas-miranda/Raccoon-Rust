use crate::{
    math::{
        Triangle
    },
    rendering::{
        HalState
    },
    window::{
        Window
    }
};

pub struct Renderer {
    _hal_state: HalState
}

impl Renderer {
    pub fn new(window: &Window) -> Result<Self, &'static str> {
        let hal_state = match HalState::new(&window.winit_window) {
            Ok(state) => state,
            Err(e) => panic!(e)
        };

        Ok(Self {
            _hal_state: hal_state
        })
    }

    pub fn draw_clear_frame(&mut self, color: [f32; 4]) -> Result<(), &'static str> {
        self._hal_state.draw_clear_frame(color)
    }

    pub fn draw_triangle_frame(&mut self, triangle: Triangle) -> Result<(), &'static str> {
        self._hal_state.draw_triangle_frame(triangle)
    }
}
