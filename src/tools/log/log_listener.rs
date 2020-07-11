use super::{
    Error
};

pub trait LogListener {
    fn write(&mut self, context: Option<&str>, msg: &str) -> Result<(), Error>;
}
