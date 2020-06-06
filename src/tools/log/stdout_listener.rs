use std::io::{
    self,
    Stdout,
    Write
};

use super::{
    LogListener
};

pub struct StdoutListener {
    _stdout_handle: Stdout
}

impl LogListener for StdoutListener {
    fn write(&mut self, category: &str, msg: &str) {
        if category.len() > 0 {
            self._stdout_handle.write_all(format!("{}: ", category).as_bytes());
        }

        self._stdout_handle.write_all(msg.as_bytes());
    }

    fn writeln(&mut self, category: &str, msg: &str) {
        if category.len() > 0 {
            self._stdout_handle.write_all(format!("{}: ", category).as_bytes());
        }

        self._stdout_handle.write_all(msg.as_bytes());
        self._stdout_handle.write_all(b"\n");
    }
}

impl StdoutListener {
    pub fn new() -> StdoutListener {
        StdoutListener {
            _stdout_handle: io::stdout()
        }
    }
}
