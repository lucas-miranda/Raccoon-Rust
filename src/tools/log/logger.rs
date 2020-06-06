use super::{
    LogListener
};

pub struct Logger {
    _listeners: Vec<Box<dyn LogListener>>
}

impl Logger {
    pub fn register<L: LogListener + 'static>(&mut self, listener: L) {
        self._listeners.push(Box::new(listener));
    }

    pub fn clear_listeners(&mut self) {
        self._listeners.clear();
    }

    pub fn write(&mut self, category: &str, msg: &str) {
        for listener in self._listeners.iter_mut() {
            listener.write(category, msg);
        }
    }

    pub fn writeln(&mut self, category: &str, msg: &str) {
        for listener in self._listeners.iter_mut() {
            listener.writeln(category, msg);
        }
    }

    pub(crate) fn new() -> Logger {
        Logger {
            _listeners: Vec::new()
        }
    }
}

