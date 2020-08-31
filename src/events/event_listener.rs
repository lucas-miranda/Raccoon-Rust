use crate::{
    events::Event
};

pub trait EventListener<E> {
    fn notify(&mut self, event: &mut Event<E>);
}

