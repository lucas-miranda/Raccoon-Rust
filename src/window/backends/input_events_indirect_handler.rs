use crate::{
    events::Event,
    input::InputEvent
};

pub trait InputEventsIndirectHandler<T> {
    fn handle(&mut self, listener: &mut T, events: &mut Vec<Event<InputEvent>>);
    fn handle_multiple(&mut self, listeners: Vec<T>, events: &mut Vec<Event<InputEvent>>);
}
