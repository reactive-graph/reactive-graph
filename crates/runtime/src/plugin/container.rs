use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use crate::implementation::get_deploy_path;
use crate::implementation::get_install_path;
use dashmap::DashSet;
use libloading::Library;
use log::debug;
use log::error;
use log::info;
use log::trace;
use uuid::Uuid;

use crate::plugin::registrar::PluginRegistrar;
use crate::plugin::PluginProxy;
use crate::plugin::PluginTransitionResult;
use crate::plugin::PluginTransitionResult::Changed;
use crate::plugin::PluginTransitionResult::NoChange;
use crate::plugins::plugin_state::PluginRefreshingState;
use crate::plugins::plugin_state::PluginResolveState;
use crate::plugins::plugin_state::PluginStartError;
use crate::plugins::plugin_state::PluginStartingState;
use crate::plugins::plugin_state::PluginStopError;
use crate::plugins::plugin_state::PluginStoppingState;
use crate::plugins::Plugin;
use crate::plugins::PluginContext;
use crate::plugins::PluginDeclaration;
use crate::plugins::PluginDependency;
use crate::plugins::PluginDeployError;
use crate::plugins::PluginState;
use crate::plugins::PluginUninstallError;
use crate::plugins::PluginUninstallingState;
use crate::plugins::PLUGIN_API_VERSION;
use crate::plugins::RUSTC_VERSION;

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
    pub proxy: RwLock<Option<Arc<PluginProxy>>>,

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
            proxy: RwLock::new(None),
            library: RwLock::new(None),
            dependencies: DashSet::new(),
        }
    }

    /// Moves the file from the deploy directory to the installed directory
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
        match fs::rename(&deploy_path, &install_path) {
            Ok(_) => {
                self.path = install_path.clone();
                self.state = PluginState::Refreshing(PluginRefreshingState::Installed);
                debug!("Plugin {} successfully deployed from {} to {}", self.id, deploy_path.display(), install_path.display());
                Changed
            }
            Err(e) => {
                error!("Failed to deploy plugin {} from {} to {}: {}", self.id, deploy_path.display(), install_path.display(), e);
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
        let mut writer = self.library.write().unwrap();
        // This drops the library
        *writer = None;
        debug!("Plugin {} unloaded dynamic linked library located at {}", self.id, self.path.display());
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
            unsafe {
                for dependency in (plugin_declaration.get_dependencies)() {
                    trace!("Plugin {} depends on {}:{}", self.id, &dependency.name, &dependency.version);
                    self.dependencies.insert(dependency);
                }
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
    pub fn construct_proxy(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Starting(PluginStartingState::ConstructingProxy)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::ConstructingProxy))
        {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::ConstructingProxy));
        let reader = self.plugin_declaration.read().unwrap();
        if let Some(plugin_declaration) = *reader {
            trace!("Plugin {} is constructing proxy", self.id);
            let mut registrar = PluginRegistrar::new();
            unsafe {
                (plugin_declaration.register)(&mut registrar);
            }
            let mut writer = self.proxy.write().unwrap();
            *writer = registrar.plugin;
            debug!("Plugin {} successfully constructed proxy", self.id);
            if refreshing {
                self.state = PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::InjectingContext));
            } else {
                self.state = PluginState::Starting(PluginStartingState::InjectingContext);
            }
            return Changed;
        }
        NoChange
    }

    /// Injects the context into the plugin.
    pub fn inject_context(&mut self, plugin_context: Arc<dyn PluginContext>) -> PluginTransitionResult {
        if self.state != PluginState::Starting(PluginStartingState::InjectingContext)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::InjectingContext))
        {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::InjectingContext));
        let reader = self.proxy.read().unwrap();
        if let Some(proxy) = reader.as_ref().cloned() {
            trace!("Plugin {} is injecting context", self.id);
            match proxy.set_context(plugin_context) {
                Ok(_) => {
                    debug!("Plugin {} successfully injected context", self.id);
                    if refreshing {
                        self.state = PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Registering));
                    } else {
                        self.state = PluginState::Starting(PluginStartingState::Registering);
                    }
                    return Changed;
                }
                Err(e) => {
                    error!("Failed to inject context {}: {:?}", self.id, e);
                }
            }
        }
        NoChange
    }

    /// Calls the activate method of the plugin.
    pub fn activate(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Starting(PluginStartingState::Activating)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Activating))
        {
            return NoChange;
        }
        let reader = self.proxy.read().unwrap();
        if let Some(proxy) = reader.as_ref().map(|proxy| proxy.clone()) {
            trace!("Plugin {} is being activated", self.id);
            match proxy.activate() {
                Ok(_) => {
                    debug!("Plugin {} has been activated successfully", self.id);
                    self.state = PluginState::Active;
                    info!("[ACTIVE] {}:{}", self.name().unwrap_or_default(), self.version().unwrap_or_default());
                    return Changed;
                }
                Err(e) => {
                    error!("Plugin {} failed to activate: {:?}", self.id, e);
                }
            }
        }
        NoChange
    }

    /// Calls the deactivate method of the plugin
    pub fn deactivate(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Stopping(PluginStoppingState::Deactivating)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Deactivating))
        {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Deactivating));
        let reader = self.proxy.read().unwrap();
        if let Some(proxy) = reader.as_ref().map(|proxy| proxy.clone()) {
            trace!("Plugin {} is being deactivated", self.id);
            match proxy.deactivate() {
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
        }
        NoChange
    }

    /// Removes the plugin context from the plugin.
    pub fn remove_context(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Stopping(PluginStoppingState::RemoveContext)
            && self.state != PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::RemoveContext))
        {
            return NoChange;
        }
        let refreshing = self.state == PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::RemoveContext));
        let reader = self.proxy.read().unwrap();
        if let Some(proxy) = reader.as_ref().map(|proxy| proxy.clone()) {
            trace!("Plugin {} is removing the plugin context", self.id);
            match proxy.remove_context() {
                Ok(_) => {
                    debug!("Plugin {} successfully removed the plugin context", self.id);
                    if refreshing {
                        self.state = PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::RemoveProxy));
                    } else {
                        self.state = PluginState::Stopping(PluginStoppingState::RemoveProxy);
                    }
                    return Changed;
                }
                Err(e) => {
                    error!("Plugin {} failed to remove plugin context: {:?}", self.id, e);
                }
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
            PluginState::Uninstalled | PluginState::Installed | PluginState::Resolving(_) => Err(PluginStartError::NotResolved(self.state.clone())),
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
            PluginState::Uninstalled | PluginState::Installed | PluginState::Resolving(_) | PluginState::Resolved => Err(PluginStopError::NotActive),
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
            PluginState::Stopping(_) | PluginState::Starting(_) | PluginState::Refreshing(_) | PluginState::Uninstalling(_) => {
                Err(PluginDeployError::InTransition)
            }
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
            PluginState::Installed | PluginState::Resolving(_) => {
                trace!("Redeploying installed plugin {}", self.id);
                self.state = PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UnloadDll));
                Ok(())
            }
        }
    }

    // -- Getters --

    pub fn name(&self) -> Option<String> {
        let reader = self.plugin_declaration.read().unwrap();
        match *reader {
            Some(plugin_declaration) => {
                return Some(plugin_declaration.name.to_string());
            }
            None => None,
        }
    }

    pub fn description(&self) -> Option<String> {
        let reader = self.plugin_declaration.read().unwrap();
        match *reader {
            Some(plugin_declaration) => {
                return Some(plugin_declaration.description.to_string());
            }
            None => None,
        }
    }

    pub fn version(&self) -> Option<String> {
        let reader = self.plugin_declaration.read().unwrap();
        match *reader {
            Some(plugin_declaration) => {
                return Some(plugin_declaration.version.to_string());
            }
            None => None,
        }
    }

    pub fn rustc_version(&self) -> Option<String> {
        let reader = self.plugin_declaration.read().unwrap();
        match *reader {
            Some(plugin_declaration) => {
                return Some(plugin_declaration.rustc_version.to_string());
            }
            None => None,
        }
    }

    pub fn plugin_api_version(&self) -> Option<String> {
        let reader = self.plugin_declaration.read().unwrap();
        match *reader {
            Some(plugin_declaration) => {
                return Some(plugin_declaration.plugin_api_version.to_string());
            }
            None => None,
        }
    }
}
