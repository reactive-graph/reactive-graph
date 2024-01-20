use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeserializationError {
    #[error("Failed to deserialize JSON failed: {0}")]
    Json(#[from] serde_json::Error),
    #[cfg(json5)]
    #[error("Failed to deserialize JSON5 failed: {0}")]
    Json5(#[from] json5::Error),
    #[cfg(toml)]
    #[error("Failed to deserialize TOML failed: {0}")]
    Toml(#[from] toml::de::Error),
}

#[derive(Debug, Error)]
pub enum SerializationError {
    #[error("Failed to serialize JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[cfg(json5)]
    #[error("Failed to serialize JSON5: {0}")]
    Json5(#[from] json5::Error),
    #[cfg(toml)]
    #[error("Failed to serialize TOML: {0}")]
    Toml(#[from] toml::ser::Error),
}
