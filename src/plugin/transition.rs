use strum_macros::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Display)]
pub enum PluginTransitionResult {
    NoChange,
    Changed,
}
