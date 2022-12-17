use strum_macros::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Display)]
pub enum PluginResolverMode {
    Starting,
    Neutral,
    Stopping,
}
