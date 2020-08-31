use crate::{
    /*
    math::{
        Triangle
    },
    */
    core::GameLoopInterface,
    rendering::{
        backends::{
            Backend,
            BackendInterface
        }
    },
    window::Window
};

pub struct Renderer {
    backend: Backend
}

impl Renderer {
    pub fn new<L: 'static + GameLoopInterface>(window: Option<&Window<L>>) -> Result<Self, &'static str> {
        let backend = if cfg!(feature = "no-backend") {
            Backend::new::<L>(None)?
        } else {
            match window {
                Some(w) => Backend::new(Some(w))?,
                None => return Err("Window reference not found")
            }
        };

        Ok(Self {
            backend: backend
        })
    }

    pub fn get_backend(&self) -> &Backend {
        &self.backend
    }

    pub fn draw_clear_frame(&mut self, color: [f32; 4]) {
        self.backend.draw_clear_frame(color)
    }

    /*
    pub fn draw_triangle_frame(&mut self, triangle: Triangle) -> Result<(), &'static str> {
        self._hal_state.draw_triangle_frame(triangle)
    }
    */
}
