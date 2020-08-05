use std::{
    error::{
        self
    },
    io::{
        self
    },
    fmt
};

#[derive(Debug)]
pub enum ErrorKind {
    IO(io::Error)
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind
}

impl error::Error for Error {
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::IO(e) => e.fmt(f)
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error {
            kind
        }
    }
}
