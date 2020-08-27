
#[derive(Debug, PartialEq, Copy, Clone, Hash)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled
}
