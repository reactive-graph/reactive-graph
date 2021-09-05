pub trait Lifecycle {
    fn init(&self);
    fn shutdown(&self);
}
