/// A plugin has one of these lifecycle states.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PluginState {
    /// The runtime knows the plugin is there.
    Installed,

    /// The state of the plugin is not yet complete
    Resolving(PluginResolveState),

    /// The plugin is there and all it’s prerequisites (dependencies) are available. The plugin can be started (or has been stopped).
    Resolved,

    /// The plugin is being started. If it has a init method, it is executed. When done, the plugin will become "Active".
    Starting(PluginStartingState),

    /// The plugin is running.
    Active,

    /// The plugin is being stopped. If it has a shutdown method, it is executed. When done, the plugin will become "Resolved".
    Stopping(PluginStoppingState),

    /// The bundle is being uninstalled.
    Uninstalling(PluginUninstallingState),

    /// The bundle is being refreshed.
    Refreshing(PluginRefreshingState),

    /// The bundle has been removed from the runtime.
    Uninstalled,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PluginResolveState {
    /// The runtime has loaded the dynamic link library.
    Loaded,

    /// The runtime has loaded the plugin declaration.
    PluginDeclarationLoaded,

    /// The plugin was compiled with an incompatible version of rustc.
    CompilerVersionMismatch,

    /// The plugin was compiled with an incompatible version of the plugin api.
    PluginApiVersionMismatch,

    /// The plugin is compatible.
    PluginCompatible,

    /// At least of of the dependencies are not active.
    DependenciesNotActive,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PluginStartingState {
    /// The plugin proxy is being constructed.
    ConstructingProxy,

    /// The plugin context is being injected.
    InjectingContext,

    /// The providers of the plugin are being registered.
    Registering,

    /// The plugin is being activated.
    Activating,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PluginStoppingState {
    /// The plugin is being deactivated.
    Deactivating,

    /// The providers of the plugin are being unregistered.
    Unregistering,

    /// The plugin context is being removed.
    RemoveContext,

    /// The plugin proxy is being destructed.
    RemoveProxy,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PluginRefreshingState {
    /// The plugin being stopped.
    Stopping,

    /// The plugin proxy is being unloaded.
    UnloadingProxy,

    /// The library is being unloaded.
    UnloadPluginDeclaration,

    /// The library is being unloaded.
    UnloadLibrary,
}

#[derive(Debug)]
pub enum PluginUnloadingError {
    UnloadingFailed,
}

#[derive(Debug)]
pub enum PluginStartError {
    AlreadyActive,
    InTransition,
    NotResolved(PluginState),
    Uninstalled,
}

#[derive(Debug)]
pub enum PluginStopError {
    NotActive,
    InTransition,
    Uninstalled,
}
