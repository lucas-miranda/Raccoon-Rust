use super::{
    InputEventsIndirectHandler
};

pub trait BackendInterface {
    fn poll_events(&mut self);
    fn redirect_input_events<T, H: InputEventsIndirectHandler<T>>(&mut self, handler: &mut H, listeners: Vec<T>);
}
