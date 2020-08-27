use crate::{
    math::Vector2
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MouseScrollDelta {
    Line {
        horizontal: f32,
        vertical: f32
    },
    Pixel(Vector2<f64>)
}
