pub trait Updatable {
    fn before_update(&mut self);
    fn update(&mut self);
    fn late_update(&mut self);
}
