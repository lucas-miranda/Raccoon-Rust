
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8)
}
