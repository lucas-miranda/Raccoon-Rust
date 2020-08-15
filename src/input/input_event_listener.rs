use crate::{
    events::Event,
    input:: InputEvent
};

pub trait InputEventListener {
    fn handle(&mut self, event: &mut Event<InputEvent>);
}
