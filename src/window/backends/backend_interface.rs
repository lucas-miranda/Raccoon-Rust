
pub trait BackendInterface {
    fn poll_events(&mut self);
}
