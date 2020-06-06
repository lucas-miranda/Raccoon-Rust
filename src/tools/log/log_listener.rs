
pub trait LogListener {
    fn write(&mut self, category: &str, msg: &str);
    fn writeln(&mut self, category: &str, msg: &str);
}
