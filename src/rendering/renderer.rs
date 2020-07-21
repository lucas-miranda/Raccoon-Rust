use crate::{
    /*
    math::{
        Triangle
    },
    */
    graphics::Graphic,
    rendering::{
        backends::{
            BackendInterface,
            Backend
        }
    }
    /*
    window::{
        Window
    }
    */
};

pub struct Renderer {
    backend: Backend
}

impl Renderer {
    pub fn new() -> Result<Self, &'static str> {
        let backend = Backend::new()?;

        Ok(Self {
            backend: backend
        })
    }

    pub fn render<T: Graphic>(&self, graphic: &T) {
        self.backend.draw(graphic);
    }

    /*
    pub fn draw_clear_frame(&mut self, color: [f32; 4]) -> Result<(), &'static str> {
        self._hal_state.draw_clear_frame(color)
    }

    pub fn draw_triangle_frame(&mut self, triangle: Triangle) -> Result<(), &'static str> {
        self._hal_state.draw_triangle_frame(triangle)
    }
    */
}
