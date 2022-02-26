pub trait Lifecycle {
    fn init(&self);
    fn post_init(&self);
    fn pre_shutdown(&self);
    fn shutdown(&self);
}
