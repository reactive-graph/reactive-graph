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

    /// The plugin is being uninstalled.
    Uninstalling(PluginUninstallingState),

    /// The plugin is being refreshed.
    Refreshing(PluginRefreshingState),

    /// The plugin has been removed from the runtime.
    Uninstalled,

    /// The plugin has been disabled.
    Disabled,
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
pub enum PluginUninstallingState {
    /// The DLL is being unloaded from memory.
    UnloadDll,

    /// The DLL is being deleted from file system.
    UninstallDll,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PluginRefreshingState {
    /// The plugin is being stopped.
    Stopping(PluginStoppingState),

    /// The plugin is being uninstalled.
    Uninstalling(PluginUninstallingState),

    /// The plugin is being deployed.
    Deploying,

    /// The plugin is installed.
    Installed,

    /// The plugin is being resolved.
    Resolving(PluginResolveState),

    /// The plugin is being started.
    Starting(PluginStartingState),
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

#[derive(Debug)]
pub enum PluginUninstallError {
    AlreadyUninstalled,
    NotStopped,
    InTransition,
    Uninstalled,
}

#[derive(Debug)]
pub enum PluginDeployError {
    InTransition,
    Uninstalled,
    NotFound,
}
