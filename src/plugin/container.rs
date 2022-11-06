use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

use crate::api::PluginTransitionResult;
use crate::api::PluginTransitionResult::Changed;
use crate::api::PluginTransitionResult::NoChange;
use crate::plugins::plugin_state::PluginRefreshingState;
use crate::plugins::plugin_state::PluginResolveState;
use crate::plugins::plugin_state::PluginStartError;
use crate::plugins::plugin_state::PluginStartingState;
use crate::plugins::plugin_state::PluginStopError;
use crate::plugins::plugin_state::PluginStoppingState;
use crate::plugins::Plugin;
use crate::plugins::PluginContext;
use crate::plugins::PLUGIN_API_VERSION;
use crate::plugins::RUSTC_VERSION;
use dashmap::DashSet;
use libloading::Library;
use log::debug;
use log::error;
use log::info;
use log::trace;
use uuid::Uuid;

use crate::plugin::registrar::PluginRegistrar;
use crate::plugin::PluginProxy;
use crate::plugins::PluginDeclaration;
use crate::plugins::PluginDependency;
use crate::plugins::PluginState;

/// The plugin container holds the meta information and the library.
pub struct PluginContainer {
    /// The id.
    pub id: Uuid,

    /// The filename stem.
    pub stem: String,

    /// The path of the plugin in the file system.
    pub path: Box<Path>,

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
    pub fn new(stem: String, path: Box<Path>) -> Self {
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

    /// Loads the dynamic link library into memory.
    pub fn load_dll(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Installed {
            return NoChange;
        }
        unsafe {
            match Library::new(self.path.as_os_str()) {
                Ok(library) => {
                    let mut writer = self.library.write().unwrap();
                    *writer = Some(Arc::new(library));
                    self.state = PluginState::Resolving(PluginResolveState::Loaded);
                    debug!("Plugin {} successfully loaded dynamic link library located at {}", self.id, self.path.display());
                    Changed
                }
                Err(e) => {
                    error!("Plugin {} failed to load dynamic link library located at {}: {:?}", self.id, self.path.display(), e);
                    self.state = PluginState::Uninstalling;
                    Changed
                }
            }
        }
    }

    /// Unloads the dynamic link library.
    pub fn unload_dll(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Uninstalling {
            return NoChange;
        }
        let mut writer = self.library.write().unwrap();
        // This drops the library
        *writer = None;
        debug!("Plugin {} unloaded library located at {}", self.id, self.path.display());
        self.state = PluginState::Uninstalled;
        Changed
    }

    /// Uninstalls the dynamic link library by removing the file from the plugin folder.
    pub fn uninstall(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Uninstalling {
            return NoChange;
        }
        match self.unload_dll() {
            Changed => match std::fs::remove_file(self.path.clone()) {
                Ok(_) => {
                    debug!("Plugin {} deleted dynamic linked library located at {}", self.id, self.path.display());
                    Changed
                }
                Err(_) => NoChange,
            },
            NoChange => NoChange,
        }
    }

    // TODO!
    pub fn refresh_dll(&mut self) {
        if self.state != PluginState::Refreshing(PluginRefreshingState::UnloadLibrary) {
            return;
        }
        let mut writer = self.library.write().unwrap();
        // This drops the library
        trace!("Plugin {} unloaded library located at {}", self.id, self.path.display());
        *writer = None;
        unsafe {
            match Library::new(self.path.as_os_str()) {
                Ok(library) => {
                    let mut writer = self.library.write().unwrap();
                    *writer = Some(Arc::new(library));
                    debug!("Plugin {} successfully loaded library located at {}", self.id, self.path.display());
                    self.state = PluginState::Resolving(PluginResolveState::Loaded);
                }
                Err(e) => {
                    error!("Plugin {} failed to load library located at {}: {:?}", self.id, self.path.display(), e);
                }
            }
        }
    }

    /// Loads the plugin declaration from the dynamic link library.
    /// The plugin declaration contains version information and a registration method.
    pub fn load_plugin_declaration(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Resolving(PluginResolveState::Loaded) {
            return NoChange;
        }
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
                        self.state = PluginState::Resolving(PluginResolveState::PluginDeclarationLoaded);
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
        if self.state != PluginState::Resolving(PluginResolveState::PluginDeclarationLoaded) {
            return NoChange;
        }
        let reader = self.plugin_declaration.read().unwrap();
        match *reader {
            Some(plugin_declaration) => {
                if plugin_declaration.rustc_version != RUSTC_VERSION {
                    error!(
                        "Plugin {} is not compatible: Expected rustc_version {} - Actual {}",
                        self.id, RUSTC_VERSION, plugin_declaration.rustc_version
                    );
                    self.state = PluginState::Resolving(PluginResolveState::CompilerVersionMismatch);
                    return Changed;
                }
                if plugin_declaration.plugin_api_version != PLUGIN_API_VERSION {
                    error!(
                        "Plugin {} is not compatible: Expected plugin_api_version {} - Actual {}",
                        self.id, PLUGIN_API_VERSION, plugin_declaration.plugin_api_version
                    );
                    self.state = PluginState::Resolving(PluginResolveState::PluginApiVersionMismatch);
                    return Changed;
                }
                debug!("Plugin {} is compatible with the rustc_version and the plugin_api_version)", self.id);
                self.state = PluginState::Resolving(PluginResolveState::PluginCompatible);
                Changed
            }
            None => {
                self.state = PluginState::Resolving(PluginResolveState::Loaded);
                Changed
            }
        }
    }

    pub fn load_plugin_dependencies(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Resolving(PluginResolveState::PluginCompatible) {
            return NoChange;
        }
        let reader = self.plugin_declaration.read().unwrap();
        if let Some(plugin_declaration) = *reader {
            trace!("Plugin {} is loading the list of dependencies", self.id);
            unsafe {
                for dependency in (plugin_declaration.get_dependencies)() {
                    trace!("Plugin {} depends on {}:{}", self.id, &dependency.name, &dependency.version);
                    self.dependencies.insert(dependency);
                }
            }
            self.state = PluginState::Resolving(PluginResolveState::DependenciesNotActive);
            return Changed;
        }
        NoChange
    }

    /// Starts the plugin.
    pub fn start(&mut self) -> Result<(), PluginStartError> {
        match self.state {
            PluginState::Active => Err(PluginStartError::AlreadyActive),
            PluginState::Starting(_) | PluginState::Stopping(_) | PluginState::Refreshing(_) | PluginState::Uninstalling => Err(PluginStartError::InTransition),
            PluginState::Uninstalled | PluginState::Installed | PluginState::Resolving(_) => Err(PluginStartError::NotResolved(self.state.clone())),
            PluginState::Resolved => {
                info!("Starting plugin {}", self.id);
                self.state = PluginState::Starting(PluginStartingState::ConstructingProxy);
                Ok(())
            }
        }
    }

    /// Constructs the proxy for the plugin.
    pub fn construct_proxy(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Starting(PluginStartingState::ConstructingProxy) {
            return NoChange;
        }
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
            self.state = PluginState::Starting(PluginStartingState::InjectingContext);
            return Changed;
        }
        NoChange
    }

    /// Injects the context into the plugin.
    pub fn inject_context(&mut self, plugin_context: Arc<dyn PluginContext>) -> PluginTransitionResult {
        if self.state != PluginState::Starting(PluginStartingState::InjectingContext) {
            return NoChange;
        }
        let reader = self.proxy.read().unwrap();
        if let Some(proxy) = reader.as_ref().cloned() {
            trace!("Plugin {} is injecting context", self.id);
            match proxy.set_context(plugin_context) {
                Ok(_) => {
                    debug!("Plugin {} successfully injected context", self.id);
                    self.state = PluginState::Starting(PluginStartingState::Registering);
                    return Changed;
                }
                Err(e) => {
                    error!("Failed to inject context {}: {:?}", self.id, e);
                }
            }
        }
        NoChange
    }

    // /// Registers the providers of the plugin.
    // pub fn register(&mut self) -> PluginTransitionResult {
    //     if self.state != PluginState::Starting(PluginStartingState::Registering) {
    //         return NoChange;
    //     }
    //     let reader = self.proxy.read().unwrap();
    //     if let Some(proxy) = reader.as_ref().map(|proxy| proxy.clone()) {
    //         info!("Plugin {} is being registered", self.id);
    //
    //         if let Ok(Some(component_provider)) = proxy.get_component_provider() {
    //             self.component_manager.add_provider(component_provider);
    //         }
    //         if let Ok(Some(entity_type_provider)) = proxy.get_entity_type_provider() {
    //             self.entity_type_manager.add_provider(entity_type_provider);
    //         }
    //         if let Ok(Some(relation_type_provider)) = proxy.get_relation_type_provider() {
    //             self.relation_type_manager.add_provider(relation_type_provider);
    //         }
    //         if let Ok(Some(flow_type_provider)) = proxy.get_flow_type_provider() {
    //             self.flow_type_manager.add_provider(flow_type_provider);
    //         }
    //         if let Ok(Some(component_behaviour_provider)) = proxy.get_component_behaviour_provider() {
    //             self.component_behaviour_manager.add_provider(component_behaviour_provider);
    //         }
    //         if let Ok(Some(entity_behaviour_provider)) = proxy.get_entity_behaviour_provider() {
    //             self.entity_behaviour_manager.add_provider(entity_behaviour_provider);
    //         }
    //         if let Ok(Some(relation_behaviour_provider)) = proxy.get_relation_behaviour_provider() {
    //             self.relation_behaviour_manager.add_provider(relation_behaviour_provider);
    //         }
    //         if let Ok(Some(flow_instance_provider)) = proxy.get_flow_instance_provider() {
    //             self.reactive_flow_instance_manager.add_provider(flow_instance_provider);
    //         }
    //         if let Ok(Some(web_resource_provider)) = proxy.get_web_resource_provider() {
    //             self.web_resource_manager.add_provider(web_resource_provider);
    //         }
    //         info!("Plugin {} has been registered successfully", self.id);
    //         return Changed;
    //     }
    //     NoChange
    // }

    /// Calls the activate method of the plugin.
    pub fn activate(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Starting(PluginStartingState::Activating) {
            return NoChange;
        }
        let reader = self.proxy.read().unwrap();
        if let Some(proxy) = reader.as_ref().map(|proxy| proxy.clone()) {
            trace!("Plugin {} is being activated", self.id);
            match proxy.activate() {
                Ok(_) => {
                    debug!("Plugin {} has been activated successfully", self.id);
                    self.state = PluginState::Active;
                    return Changed;
                }
                Err(e) => {
                    error!("Plugin {} failed to activate: {:?}", self.id, e);
                }
            }
        }
        NoChange
    }

    pub fn stop(&mut self) -> Result<(), PluginStopError> {
        match self.state {
            PluginState::Stopping(_) | PluginState::Starting(_) | PluginState::Refreshing(_) => Err(PluginStopError::InTransition),
            PluginState::Uninstalled | PluginState::Installed | PluginState::Resolving(_) | PluginState::Resolved | PluginState::Uninstalling => {
                Err(PluginStopError::NotActive)
            }
            PluginState::Active => {
                trace!("Stopping plugin {}", self.id);
                self.state = PluginState::Stopping(PluginStoppingState::Deactivating);
                Ok(())
            }
        }
    }

    /// Calls the deactivate method of the plugin
    pub fn deactivate(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Stopping(PluginStoppingState::Deactivating) {
            return NoChange;
        }
        let reader = self.proxy.read().unwrap();
        if let Some(proxy) = reader.as_ref().map(|proxy| proxy.clone()) {
            trace!("Plugin {} is being deactivated", self.id);
            match proxy.deactivate() {
                Ok(_) => {
                    debug!("Plugin {} has been deactivated successfully", self.id);
                    self.state = PluginState::Stopping(PluginStoppingState::Unregistering);
                    return Changed;
                }
                Err(e) => {
                    error!("Plugin {} failed to deactivate: {:?}", self.id, e);
                }
            }
        }
        NoChange
    }

    // /// Calls the deactivate method of the plugin
    // pub fn unregister(&mut self) -> PluginTransitionResult {
    //     if self.state != PluginState::Stopping(PluginStoppingState::Unregistering) {
    //         return NoChange;
    //     }
    //     let reader = self.proxy.read().unwrap();
    //     if let Some(proxy) = reader.as_ref().map(|proxy| proxy.clone()) {
    //         trace!("Plugin {} is being deactivated", self.id);
    //         match proxy.unregister() {
    //             Ok(_) => {
    //                 debug!("Plugin {} has been deactivated successfully", self.id);
    //                 self.state = PluginState::Stopping(PluginStoppingState::RemoveContext);
    //                 return Changed;
    //             }
    //             Err(e) => {
    //                 error!("Plugin {} failed to deactivate: {:?}", self.id, e);
    //             }
    //         }
    //     }
    //     NoChange
    // }

    /// Removes the plugin context from the plugin.
    pub fn remove_context(&mut self) -> PluginTransitionResult {
        if self.state != PluginState::Stopping(PluginStoppingState::RemoveContext) {
            return NoChange;
        }
        let reader = self.proxy.read().unwrap();
        if let Some(proxy) = reader.as_ref().map(|proxy| proxy.clone()) {
            trace!("Plugin {} is removing the plugin context", self.id);
            match proxy.remove_context() {
                Ok(_) => {
                    debug!("Plugin {} successfully removed the plugin context", self.id);
                    self.state = PluginState::Stopping(PluginStoppingState::RemoveProxy);
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
        if self.state != PluginState::Stopping(PluginStoppingState::RemoveProxy) {
            return NoChange;
        }
        let mut writer = self.proxy.write().unwrap();
        // This drops the proxy.
        *writer = None;
        self.state = PluginState::Resolved;
        Changed
    }
}
