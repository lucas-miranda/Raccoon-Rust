use chrono::{
    Local
};

use super::{
    LogListener,
    Error
};

pub struct Logger {
    pub context_enabled: bool,
    _listeners: Vec<Box<dyn LogListener>>,
    _block_level: u32,
    _subjects: Vec<String>
}

impl Logger {
    pub fn register<L: LogListener + 'static>(&mut self, listener: L) {
        self._listeners.push(Box::new(listener));
    }

    pub fn clear_listeners(&mut self) {
        self._listeners.clear();
    }

    pub fn write(&mut self, context: &str, msg: &str) {
        let c = {
            if context.len() > 0 {
                Some(context)
            } else {
                None
            }
        };

        for listener in self._listeners.iter_mut() {
            listener.write(c, msg);
        }
    }

    pub fn writeln(&mut self, context: &str, msg: &str) -> Result<(), Error> {
        let timestamp = self.get_system_time_now();
        let separation = "  ";

        if context.len() > 0 {
            for listener in self._listeners.iter_mut() {
                listener.write(Some("timestamp"), &timestamp);
                listener.write(None, separation);
                listener.write(Some(context), context);
                listener.write(None, separation);
                listener.write(None, msg);
                listener.write(None, "\n");
            }
        } else {
            for listener in self._listeners.iter_mut() {
                listener.write(Some("timestamp"), &timestamp);
                listener.write(None, separation);
                listener.write(None, msg);
                listener.write(None, "\n");
            }
        }

        Ok(())
    }

    pub fn push_subject<T: Into<String>>(&mut self, subject: T) {
        self._subjects.push(subject.into());
    }

    pub fn pop_subject(&mut self) {
        self._subjects.pop();
    }

    pub fn clear_subjects(&mut self) {
        self._subjects.clear();
    }

    pub fn begin_block(&mut self) {
        self._block_level += 1;
    }

    pub fn end_block(&mut self) {
        if self._block_level == 0 {
            return;
        }

        self._block_level -= 1;
    }

    pub fn end_mult_blocks(&mut self, blocks: u32) {
        if self._block_level <= blocks {
            self._block_level = 0;
            return;
        }

        self._block_level -= blocks;
    }

    pub fn end_all_blocks(&mut self) {
        self._block_level = 0;
    }

    pub fn get_block_level(&self) -> u32 {
        self._block_level
    }

    pub(crate) fn new() -> Logger {
        Logger {
            context_enabled: true,
            _listeners: Vec::new(),
            _block_level: 0,
            _subjects: Vec::new()
        }
    }

    fn get_system_time_now(&self) -> String {
        Local::now()
              .format("%d/%m/%G  %T")
              .to_string()
    }
}

#[macro_export]
macro_rules! logln {
    ($logger:expr) => {
        $logger.write("", "\n");
    };

    ($logger:expr, $message:expr) => {
        $logger.writeln("", $message);
    };

    ($logger:expr, $message:expr, $($arg:tt),*) => {
        $logger.writeln("", format!($message, format_args!($message, $($arg)*)));
    };
}

#[macro_export]
macro_rules! log {
    ($logger:expr, $message:expr) => {
        $logger.write("", $message);
    };

    ($logger:expr, $message:expr, $($arg:tt),*) => {
        $logger.write("", format!($message, format_args!($message, $($arg)*)));
    };
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $message:expr) => {
        $logger.writeln("info", $message);
    };

    ($logger:expr, $message:expr, $($arg:tt),*) => {
        $logger.writeln("info", format!($message, format_args!($message, $($arg)*)));
    };
}

#[macro_export]
macro_rules! log_warning {
    ($logger:expr, $message:expr) => {
        $logger.writeln("warning", $message);
    };

    ($logger:expr, $message:expr, $($arg:tt),*) => {
        $logger.writeln("warning", format!($message, format_args!($message, $($arg)*)));
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $message:expr) => {
        $logger.writeln("error", $message);
    };

    ($logger:expr, $message:expr, $($arg:tt),*) => {
        $logger.writeln("error", format!($message, format_args!($message, $($arg)*)));
    };
}
