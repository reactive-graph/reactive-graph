use thiserror::Error;

use crate::springtime_di::component_registry::ComponentDefinitionRegistryError;
use crate::springtime_di::instance_provider::ComponentInstanceProviderError;

#[derive(Debug, Error)]
pub enum PluginLoadingError {
    #[error("Loading the dynamic library (dll/so) failed!")]
    // TODO: LoadingDynamicLibraryFailed(Path)
    LoadingDynamicLibraryFailed,
    #[error("The plugin must be compiled with the same version as the runtime!")]
    // TODO: CompilerVersionMismatch(Version, Version)
    CompilerVersionMismatch,
    #[error("The version of the plugin API used by the plugin must match with the version of the plugin API used by the runtime")]
    // TODO: PluginApiVersionMismatch(Version, Version)
    PluginApiVersionMismatch,
    #[error("Failed to initialize the plugin container!")]
    PluginContainerInitializationError,
    #[error("Failed to globalize the plugin context in the plugin")]
    GlobalizePluginContextError,
    #[error("The component definition registry failed with an error: {0}")]
    ComponentDefinitionRegistryError(ComponentDefinitionRegistryError),
    #[error("The component instance provider failed with an error: {0}")]
    ComponentInstanceProviderError(ComponentInstanceProviderError),
    #[error("The plugin declaration is wrong: {message}")]
    PluginDeclarationError { message: String },
}

#[derive(Debug, Error)]
pub enum PluginUnloadingError {
    #[error("Failed to unload the plugin!")]
    UnloadingFailed,
}
