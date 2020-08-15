
pub trait WindowEventsHandler<T> {
    fn handle(&mut self, listener: &mut T);
    fn handle_multiple(&mut self, listeners: Vec<T>);
}
