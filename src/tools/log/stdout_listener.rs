use std::{
    collections::HashMap,
    io::{
        self,
        Stdout,
        Write
    }
};

use colored::Colorize;

use super::{
    LogListener,
    Error,
    ErrorKind
};

pub struct StdoutListener {
    _stdout_handle: Stdout,
    _contexts: HashMap<String, Box<dyn Fn(&str) -> String>>
}

impl LogListener for StdoutListener {
    fn write(&mut self, context: Option<&str>, msg: &str) -> Result<(), Error> {
        match context {
            Some(c) => {
                if let Some((name, processor)) = self._contexts.remove_entry(c) {
                    match self.write_to_stdout(&processor(msg)) {
                        Ok(()) => self._contexts.insert(name, processor),
                        Err(e) => {
                            self._contexts.insert(name, processor);
                            return Err(e);
                        }
                    };
                } else {
                    self.write_to_stdout(msg)?;
                }

                Ok(())
            },
            None => {
                self.write_to_stdout(msg)?;
                Ok(())
            }
        }
    }
}

impl StdoutListener {
    pub fn new() -> StdoutListener {
        let mut stdout_listener = StdoutListener {
            _stdout_handle: io::stdout(),
            _contexts: HashMap::new()
        };

        // std contexts
        stdout_listener.register_context("info",        |msg| msg.blue().to_string());
        stdout_listener.register_context("warning",     |msg| msg.yellow().to_string());
        stdout_listener.register_context("error",       |msg| msg.red().to_string());
        stdout_listener.register_context("success",     |msg| msg.green().to_string());
        stdout_listener.register_context("timestamp",   |msg| msg.bright_black().to_string());

        stdout_listener
    }

    pub fn register_context<T: 'static + Fn(&str) -> String>(&mut self, context: &str, processor: T) {
        self._contexts.insert(context.into(), Box::new(processor));
    }

    fn write_to_stdout(&mut self, msg: &str) -> Result<(), Error> {
        match self._stdout_handle.write_all(msg.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(ErrorKind::IO(e)))
        }
    }
}
