/// Dual layer runtime lifecycle for initialization and shutdown of services
pub trait Lifecycle {
    /// Called at initialization
    fn init(&self) {}

    /// Called after initialization
    fn post_init(&self) {}

    /// Called before shutdown
    fn pre_shutdown(&self) {}

    /// Called for shutdown
    fn shutdown(&self) {}
}
