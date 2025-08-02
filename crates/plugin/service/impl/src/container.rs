use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use dashmap::DashSet;
use libloading::Library;
use log::debug;
use log::error;
use log::info;
use log::trace;
use springtime_di::instance_provider::ComponentInstanceProviderError;
use uuid::Uuid;

use reactive_graph_plugin_api::PLUGIN_API_VERSION;
use reactive_graph_plugin_api::PLUGIN_NAME_PREFIX;
use reactive_graph_plugin_api::Plugin;
use reactive_graph_plugin_api::PluginContext;
use reactive_graph_plugin_api::PluginDeclaration;
use reactive_graph_plugin_api::PluginDependency;
use reactive_graph_plugin_api::PluginDeployError;
use reactive_graph_plugin_api::PluginDisableError;
use reactive_graph_plugin_api::PluginLoadingError;
use reactive_graph_plugin_api::PluginRefreshingState;
use reactive_graph_plugin_api::PluginResolveState;
use reactive_graph_plugin_api::PluginStartError;
use reactive_graph_plugin_api::PluginStartingState;
use reactive_graph_plugin_api::PluginState;
use reactive_graph_plugin_api::PluginStopError;
use reactive_graph_plugin_api::PluginStoppingState;
use reactive_graph_plugin_api::PluginUninstallError;
use reactive_graph_plugin_api::PluginUninstallingState;
use reactive_graph_plugin_api::RUSTC_VERSION;
use reactive_graph_plugin_service_api::PluginTransitionResult;
use reactive_graph_plugin_service_api::PluginTransitionResult::Changed;
use reactive_graph_plugin_service_api::PluginTransitionResult::NoChange;
use reactive_graph_plugin_service_api::get_deploy_path;
use reactive_graph_plugin_service_api::get_install_path;

use crate::PluginProxy;
use crate::PluginRegistrar;

/// The plugin container holds the meta information and the library.
pub struct PluginContainer {
    /// The id.
    pub id: Uuid,

    /// The filename stem (actually the file_prefix).
    /// Does not contain the timestamp and the file extension.
    pub stem: String,

    /// The path of the plugin in the file system.
    /// The plugin is located in the installation directory.
    pub path: PathBuf,

    /// The state of the plugin.
    pub state: PluginState,

    /// The plugin declaration.
    pub plugin_declaration: RwLock<Option<PluginDeclaration>>,

    /// The plugin context.
    pub proxy: Arc<RwLock<Option<Arc<PluginProxy>>>>,

    /// The loaded library.
    pub library: RwLock<Option<Arc<Library>>>,

    /// The dependencies of the plugin.
    pub dependencies: DashSet<PluginDependency>,
}

impl PluginContainer {
    pub fn new(stem: String, path: PathBuf) -> Self {
        PluginContainer {
            id: Uuid::new_v4(),
            stem,
            path,
            state: PluginState::Installed,
            plugin_declaration: RwLock::new(None),
            proxy: Arc::new(RwLock::new(None)),
            library: RwLock::new(None),
            dependencies: DashSet::new(),
        }
    }

    /// Moves the file from the folder `deploy` to the folder `installed`
    pub fn deploy_dll(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Refreshing(PluginRefreshingState::Deploying) {
            return NoChange;
        }
        let Some(deploy_path) = get_deploy_path(&self.path) else {
            return NoChange;
        };
        if !deploy_path.exists() {
            error!("Failed to deploy dynamic link library: {} does not exist", deploy_path.display());
            self.state = PluginState::Uninstalled;
            return Changed;
        }
        let Some(install_path) = get_install_path(&self.path) else {
            return NoChange;
        };
        match fs::copy(&deploy_path, &install_path) {
            Ok(_) => {
                trace!("Copied plugin from {} to {}", deploy_path.display(), &install_path.display());
                match fs::remove_file(&deploy_path) {
                    Ok(_) => {
                        trace!("Removed plugin from {}", deploy_path.display());
                        self.path = install_path.clone();
                        self.state = PluginState::Refreshing(PluginRefreshingState::Installed);
                        debug!("Plugin {} successfully deployed from {} to {}", self.id, deploy_path.display(), install_path.display());
                        Changed
                    }
                    Err(e) => {
                        error!("Failed to deploy plugin {}: Failed to remove plugin from {}: {:?}", self.id, deploy_path.display(), e);
                        self.state = PluginState::Uninstalled;
                        Changed
                    }
                }
            }
            Err(e) => {
                error!(
                    "Failed to deploy plugin {}: Failed to copy plugin from {} to {}: {:?}",
                    self.id,
                    deploy_path.display(),
                    install_path.display(),
                    e
                );
                self.state = PluginState::Uninstalled;
                Changed
            }
        }
    }

    /// Loads the dynamic link library into memory.
    pub fn load_dll(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Installed && self.state != PluginState::Refreshing(PluginRefreshingState::Installed) {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Installed);
        unsafe {
            match Library::new(self.path.as_os_str()) {
                Ok(library) => {
                    let mut writer = self.library.write().unwrap();
                    *writer = Some(Arc::new(library));
                    if refreshing {
                        self.state = PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::Loaded));
                    } else {
                        self.state = PluginState::Resolving(PluginResolveState::Loaded);
                    }
                    debug!("Plugin {} successfully loaded dynamic link library located at {}", self.id, self.path.display());
                    Changed
                }
                Err(e) => {
                    error!("Plugin {} failed to load dynamic link library located at {}: {:?}", self.id, self.path.display(), e);
                    self.state = PluginState::Uninstalling(PluginUninstallingState::UnloadDll);
                    Changed
                }
            }
        }
    }

    /// Unloads the dynamic link library.
    pub fn unload_dll(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Uninstalling(PluginUninstallingState::UnloadDll)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UnloadDll))
        {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UnloadDll));
        {
            let mut writer = self.plugin_declaration.write().unwrap();
            // This drops the plugin declaration
            *writer = None;
        }
        {
            let mut writer = self.library.write().unwrap();
            // This drops the library
            *writer = None;
            debug!("Plugin {} unloaded dynamic linked library located at {}", self.id, self.path.display());
        }
        self.dependencies = DashSet::new();
        if refreshing {
            self.state = PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UninstallDll));
        } else {
            self.state = PluginState::Uninstalling(PluginUninstallingState::UninstallDll);
        }
        Changed
    }

    /// Uninstalls the dynamic link library by removing the file from the plugin folder.
    pub fn uninstall_dll(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Uninstalling(PluginUninstallingState::UninstallDll)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UninstallDll))
        {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UninstallDll));
        if !self.path.exists() {
            if refreshing {
                self.state = PluginState::Refreshing(PluginRefreshingState::Deploying);
            } else {
                self.state = PluginState::Uninstalled;
            }
            return NoChange;
        }
        match fs::remove_file(self.path.clone()) {
            Ok(_) => {
                debug!("Plugin {} deleted dynamic linked library located at {}", self.id, self.path.display());
                if refreshing {
                    self.state = PluginState::Refreshing(PluginRefreshingState::Deploying);
                } else {
                    self.state = PluginState::Uninstalled;
                }
                // Changed
                NoChange
            }
            Err(e) => {
                error!("Failed to delete dynamic linked library of plugin {} located at {}: {}", self.id, self.path.display(), e);
                NoChange
            }
        }
    }

    /// Loads the plugin declaration from the dynamic link library.
    /// The plugin declaration contains version information and a registration method.
    pub fn load_plugin_declaration(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Resolving(PluginResolveState::Loaded)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::Loaded))
        {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::Loaded));
        let reader = self.library.read().unwrap();
        if let Some(library) = reader.as_ref() {
            let library = library.clone();
            unsafe {
                trace!("Plugin {} is reading dynamic linked library symbol plugin_declaration", self.id);
                match library.get::<*mut PluginDeclaration>(b"plugin_declaration\0") {
                    Ok(plugin_declaration) => {
                        {
                            let mut writer = self.plugin_declaration.write().unwrap();
                            *writer = Some(plugin_declaration.read());
                        }
                        debug!("Plugin {} successfully loaded plugin declaration", self.id);
                        if refreshing {
                            self.state = PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::PluginDeclarationLoaded));
                        } else {
                            self.state = PluginState::Resolving(PluginResolveState::PluginDeclarationLoaded);
                        }
                        return Changed;
                    }
                    Err(e) => {
                        error!("Plugin {} failed to get symbol plugin_declaration: {}", self.id, e);
                    }
                }
            }
        }
        NoChange
    }

    /// Checks for compatibility.
    pub fn check_compatibility(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Resolving(PluginResolveState::PluginDeclarationLoaded)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::PluginDeclarationLoaded))
        {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::PluginDeclarationLoaded));
        let reader = self.plugin_declaration.read().unwrap();
        match *reader {
            Some(plugin_declaration) => {
                if plugin_declaration.rustc_version != RUSTC_VERSION {
                    error!(
                        "Plugin {} is not compatible: Expected rustc_version {} - Actual {}",
                        self.id, RUSTC_VERSION, plugin_declaration.rustc_version
                    );
                    if refreshing {
                        self.state = PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::CompilerVersionMismatch));
                    } else {
                        self.state = PluginState::Resolving(PluginResolveState::CompilerVersionMismatch);
                    }
                    return Changed;
                }
                if plugin_declaration.plugin_api_version != PLUGIN_API_VERSION {
                    error!(
                        "Plugin {} is not compatible: Expected plugin_api_version {} - Actual {}",
                        self.id, PLUGIN_API_VERSION, plugin_declaration.plugin_api_version
                    );
                    if refreshing {
                        self.state = PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::PluginApiVersionMismatch));
                    } else {
                        self.state = PluginState::Resolving(PluginResolveState::PluginApiVersionMismatch);
                    }
                    return Changed;
                }
                debug!("Plugin {} is compatible with the rustc_version and the plugin_api_version)", self.id);
                if refreshing {
                    self.state = PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::PluginCompatible));
                } else {
                    self.state = PluginState::Resolving(PluginResolveState::PluginCompatible);
                }
                Changed
            }
            None => {
                if refreshing {
                    self.state = PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::Loaded));
                } else {
                    self.state = PluginState::Resolving(PluginResolveState::Loaded);
                }
                Changed
            }
        }
    }

    #[allow(clippy::redundant_clone)]
    pub fn load_plugin_dependencies(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Resolving(PluginResolveState::PluginCompatible)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::PluginCompatible))
        {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::PluginCompatible));
        let reader = self.plugin_declaration.read().unwrap();
        if let Some(plugin_declaration) = *reader {
            trace!("Plugin {} is loading the list of dependencies", self.id);
            let dependencies = unsafe { (plugin_declaration.get_dependencies)() };
            for dependency in dependencies.clone() {
                trace!("Plugin {} depends on {}:{}", self.id, &dependency.name, &dependency.version);
                self.dependencies.insert(dependency);
            }
            if refreshing {
                self.state = PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::DependenciesNotActive));
            } else {
                self.state = PluginState::Resolving(PluginResolveState::DependenciesNotActive);
            }
            return Changed;
        }
        NoChange
    }

    /// Constructs the proxy for the plugin.
    #[allow(clippy::collapsible_match)]
    pub fn construct_proxy(&mut self, plugin_context: Arc<dyn PluginContext + Send + Sync>) -> PluginTransitionResult {
        if self.state != PluginState::Starting(PluginStartingState::ConstructingProxy)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::ConstructingProxy))
        {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::ConstructingProxy));

        let reader = self.plugin_declaration.read().unwrap();
        if let Some(plugin_declaration) = *reader {
            trace!("Plugin {} is constructing proxy", self.id);
            let mut registrar = PluginRegistrar::new(plugin_context);
            unsafe {
                if let Err(e) = (plugin_declaration.register)(&mut registrar) {
                    error!("Dependency injection error in plugin {}:\n\n  {e}\n", self.id);
                    if let PluginLoadingError::ComponentInstanceProviderError(ComponentInstanceProviderError::NoPrimaryInstance { type_name, .. }) = e {
                        if let Some(type_name) = type_name {
                            let type_name_stripped = type_name.replace("dyn ", "").replace(" + core::marker::Send + core::marker::Sync", "");
                            let notice = if type_name_stripped == "reactive_graph_plugin_api::plugin::Plugin" {
                                "\n  Notice:    Every plugin must provide a component that implements reactive_graph_plugin_api::Plugin!"
                            } else {
                                ""
                            };
                            error!("Missing component\n\n  Plugin:    {}\n  Component: {type_name_stripped}{notice}\n", plugin_declaration.name);
                        }
                    }
                    self.state = PluginState::Resolved;
                    return NoChange;
                }
                self.state = PluginState::Starting(PluginStartingState::Registering);
            }
            let mut writer = self.proxy.write().unwrap();
            *writer = registrar.plugin;
            debug!("Plugin {} successfully constructed proxy", self.id);
            if refreshing {
                self.state = PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Registering));
            } else {
                self.state = PluginState::Starting(PluginStartingState::Registering);
            }
            return Changed;
        }
        NoChange
    }

    /// Calls the activate method of the plugin.
    pub async fn activate(&mut self) -> PluginTransitionResult {
        // info!("{:?}", self.state);
        if self.state != PluginState::Starting(PluginStartingState::Activating)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Activating))
        {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Activating));

        let proxy = {
            let reader = self.proxy.read().unwrap();
            let Some(proxy) = reader.as_ref().cloned() else {
                return NoChange;
            };
            proxy.clone()
        };
        trace!("Plugin {} is being activated", self.id);
        match proxy.activate().await {
            Ok(_) => {
                debug!("Plugin {} has been activated successfully", self.id);
                self.state = PluginState::Active;
                info!(
                    "[ACTIVE] {} {}",
                    self.name().unwrap_or_default().replace(PLUGIN_NAME_PREFIX, ""),
                    self.version().unwrap_or_default()
                );
            }
            Err(e) => {
                error!(
                    "[FAILED] {} {}: {}",
                    self.name().unwrap_or_default().replace(PLUGIN_NAME_PREFIX, ""),
                    self.version().unwrap_or_default(),
                    e
                );
                if refreshing {
                    self.state = PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::ActivationFailed));
                } else {
                    self.state = PluginState::Starting(PluginStartingState::ActivationFailed);
                }
            }
        }
        Changed
    }

    /// Calls the deactivate method of the plugin
    pub async fn deactivate(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Stopping(PluginStoppingState::Deactivating)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Deactivating))
        {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Deactivating));

        let proxy = {
            let reader = self.proxy.read().unwrap();
            let Some(proxy) = reader.as_ref().cloned() else {
                return NoChange;
            };
            proxy.clone()
        };
        trace!("Plugin {} is being deactivated", self.id);
        match proxy.deactivate().await {
            Ok(_) => {
                debug!("Plugin {} has been deactivated successfully", self.id);
                if refreshing {
                    self.state = PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Unregistering));
                } else {
                    self.state = PluginState::Stopping(PluginStoppingState::Unregistering);
                }
                return Changed;
            }
            Err(e) => {
                error!("Plugin {} failed to deactivate: {:?}", self.id, e);
            }
        }
        NoChange
    }

    /// Removes the proxy from the plugin.
    pub fn remove_proxy(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Stopping(PluginStoppingState::RemoveProxy)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::RemoveProxy))
        {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::RemoveProxy));
        let mut writer = self.proxy.write().unwrap();
        // This drops the proxy.
        *writer = None;
        if refreshing {
            self.state = PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UnloadDll));
        } else {
            self.state = PluginState::Resolved;
        }
        Changed
    }

    // -- Entry Points --

    /// Starts the plugin.
    pub fn start(&mut self) -> Result<(), PluginStartError> {
        match self.state {
            PluginState::Active => Err(PluginStartError::AlreadyActive),
            PluginState::Starting(_) | PluginState::Stopping(_) | PluginState::Refreshing(_) | PluginState::Uninstalling(_) => {
                Err(PluginStartError::InTransition)
            }
            PluginState::Uninstalled | PluginState::Disabled | PluginState::Installed | PluginState::Resolving(_) => {
                Err(PluginStartError::NotResolved(self.state))
            }
            PluginState::Resolved => {
                trace!("Starting plugin {}", self.id);
                self.state = PluginState::Starting(PluginStartingState::ConstructingProxy);
                Ok(())
            }
        }
    }

    pub fn stop(&mut self) -> Result<(), PluginStopError> {
        match self.state {
            PluginState::Stopping(_) | PluginState::Starting(_) | PluginState::Refreshing(_) | PluginState::Uninstalling(_) => {
                Err(PluginStopError::InTransition)
            }
            PluginState::Uninstalled | PluginState::Disabled | PluginState::Installed | PluginState::Resolving(_) | PluginState::Resolved => {
                Err(PluginStopError::NotActive)
            }
            PluginState::Active => {
                trace!("Stopping plugin {}", self.id);
                self.state = PluginState::Stopping(PluginStoppingState::Deactivating);
                Ok(())
            }
        }
    }

    pub fn uninstall(&mut self) -> Result<(), PluginUninstallError> {
        match self.state {
            PluginState::Stopping(_) | PluginState::Starting(_) | PluginState::Refreshing(_) | PluginState::Uninstalling(_) => {
                Err(PluginUninstallError::InTransition)
            }
            PluginState::Uninstalled => Err(PluginUninstallError::AlreadyUninstalled),
            PluginState::Disabled => Err(PluginUninstallError::Disabled),
            PluginState::Active => Err(PluginUninstallError::NotStopped),
            PluginState::Installed | PluginState::Resolving(_) | PluginState::Resolved => {
                trace!("Uninstalling plugin {}", self.id);
                self.state = PluginState::Uninstalling(PluginUninstallingState::UnloadDll);
                Ok(())
            }
        }
    }

    pub fn redeploy(&mut self) -> Result<(), PluginDeployError> {
        match self.state {
            PluginState::Stopping(_) | PluginState::Starting(_) | PluginState::Uninstalling(_) => Err(PluginDeployError::InTransition),
            PluginState::Refreshing(PluginRefreshingState::Resolving(_)) => {
                // PluginResolveState::DependenciesNotActive
                trace!("Redeploying resolved plugin {}", self.id);
                self.state = PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UnloadDll));
                Ok(())
            }
            PluginState::Refreshing(_) => Err(PluginDeployError::InTransition),
            PluginState::Uninstalled => Err(PluginDeployError::Uninstalled),
            PluginState::Active => {
                trace!("Redeploying active plugin {}", self.id);
                self.state = PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Deactivating));
                Ok(())
            }
            PluginState::Resolved => {
                trace!("Redeploying resolved plugin {}", self.id);
                self.state = PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UnloadDll));
                Ok(())
            }
            PluginState::Installed | PluginState::Resolving(_) | PluginState::Disabled => {
                trace!("Redeploying installed plugin {}", self.id);
                self.state = PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UnloadDll));
                Ok(())
            }
        }
    }

    pub fn disable(&mut self) -> Result<(), PluginDisableError> {
        trace!("Disable plugin {}", self.id);
        self.state = PluginState::Disabled;
        Ok(())
    }

    // -- Getters --

    pub fn name(&self) -> Option<String> {
        let reader = self.plugin_declaration.read().unwrap();
        (*reader).map(|plugin_declaration| plugin_declaration.name.to_string())
    }

    pub fn name_canonicalized(&self) -> Option<String> {
        let reader = self.plugin_declaration.read().unwrap();
        (*reader).map(|plugin_declaration| plugin_declaration.name.replace(PLUGIN_NAME_PREFIX, ""))
    }

    pub fn name_version(&self) -> Option<String> {
        let reader = self.plugin_declaration.read().unwrap();
        (*reader).map(|plugin_declaration| format!("{}:{}", plugin_declaration.name.replace(PLUGIN_NAME_PREFIX, ""), plugin_declaration.version))
    }

    pub fn description(&self) -> Option<String> {
        let reader = self.plugin_declaration.read().unwrap();
        (*reader).map(|plugin_declaration| plugin_declaration.description.to_string())
    }

    pub fn version(&self) -> Option<String> {
        let reader = self.plugin_declaration.read().unwrap();
        (*reader).map(|plugin_declaration| plugin_declaration.version.to_string())
    }

    pub fn rustc_version(&self) -> Option<String> {
        let reader = self.plugin_declaration.read().unwrap();
        (*reader).map(|plugin_declaration| plugin_declaration.rustc_version.to_string())
    }

    pub fn plugin_api_version(&self) -> Option<String> {
        let reader = self.plugin_declaration.read().unwrap();
        (*reader).map(|plugin_declaration| plugin_declaration.plugin_api_version.to_string())
    }
}
