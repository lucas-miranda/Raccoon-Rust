use crate::{
    events::Event,
    window::WindowEvent
};

pub trait WindowEventListener {
    fn handle(&mut self, event: &mut Event<WindowEvent>);
}
