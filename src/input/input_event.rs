use crate::{
    input::{
        KeyboardEvent,
        KeyModifiers,
        MouseButtonEvent,
        MouseScrollDelta,
        TouchEvent,
        TouchPhase
    },
    math::Vector2,
};

pub enum InputEvent {
    Keyboard(KeyboardEvent),
    ReceivedChar(char),
    MouseButton(MouseButtonEvent),
    MouseWheel {
        delta: MouseScrollDelta,
        phase: TouchPhase,
        modifiers: KeyModifiers
    },
    CursorEntered,
    CursorLeft,
    CursorMoved {
        position: Vector2<f64>, 
        modifiers: KeyModifiers
    },
    TouchpadPressure {
        pressure: f32,
        stage: i64
    },
    Touch(TouchEvent),
    AxisMotion {
        axis: u32,
        value: f64
    }
}
