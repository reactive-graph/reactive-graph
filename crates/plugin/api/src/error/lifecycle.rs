use thiserror::Error;

#[derive(Debug, Error)]
pub enum PluginContextInitializationError {
    #[error("Failed to initialize plugin context")]
    InitializationError,
}

#[derive(Debug, Error)]
pub enum PluginContextDeinitializationError {
    #[error("Failed to deinitialize plugin context")]
    DeinitializationError,
}
