use crate::{
    events::Event
};

pub trait EventHandler<E> {
    fn handle(&mut self, event: &mut Event<E>);
}
