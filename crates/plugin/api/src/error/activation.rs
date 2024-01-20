use thiserror::Error;

// TODO: Add more specific error types
#[derive(Debug, Error)]
pub enum PluginActivationError {
    #[error("The activation of the plugin failed: {0}")]
    ActivationFailed(String),
    #[error("The activation of the plugin failed because the plugin context is missing!")]
    PluginRequiresMissingPluginContext,
}

// TODO: Add more specific error types
#[derive(Debug, Error)]
pub enum PluginDeactivationError {
    #[error("The deactivation of the plugin failed!")]
    DeactivationFailed,
    #[error("The deactivation of the plugin failed because the plugin context is missing!")]
    PluginRequiresMissingPluginContext,
}
