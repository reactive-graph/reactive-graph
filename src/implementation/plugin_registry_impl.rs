use std::collections::HashMap;
use std::ffi::OsStr;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use libloading::Library;
use log::{debug, error};
use waiter_di::*;

use crate::api::{
    ComponentBehaviourManager, ComponentManager, EntityBehaviourManager, EntityTypeManager, Lifecycle, PluginRegistry, ReactiveEntityInstanceManager,
    ReactiveFlowManager, ReactiveRelationInstanceManager, RelationBehaviourManager, RelationTypeManager, WebResourceManager,
};
use crate::plugin::registrar::PluginRegistrar;
use crate::plugin::{
    ComponentManagerImpl, EntityInstanceManagerImpl, EntityTypeManagerImpl, FlowManagerImpl, PluginContextImpl, PluginProxy, PluginsConfig,
    RelationInstanceManagerImpl, RelationTypeManagerImpl,
};
use crate::plugins::{Plugin, PluginDeclaration, PluginError, INEXOR_RGF_PLUGIN_VERSION, RUSTC_VERSION};

#[wrapper]
pub struct PluginProxies(RwLock<HashMap<String, Arc<PluginProxy>>>);

#[wrapper]
pub struct PluginLibraries(RwLock<Vec<Arc<Library>>>);

#[provides]
fn provide_plugin_proxies() -> PluginProxies {
    PluginProxies(RwLock::new(HashMap::new()))
}

#[provides]
fn provide_plugin_libraries() -> PluginLibraries {
    PluginLibraries(RwLock::new(Vec::new()))
}

#[component]
pub struct PluginRegistryImpl {
    component_behaviour_manager: Wrc<dyn ComponentBehaviourManager>,
    component_manager: Wrc<dyn ComponentManager>,
    entity_behaviour_manager: Wrc<dyn EntityBehaviourManager>,
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    relation_behaviour_manager: Wrc<dyn RelationBehaviourManager>,
    relation_type_manager: Wrc<dyn RelationTypeManager>,
    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,
    reactive_relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,
    reactive_flow_manager: Wrc<dyn ReactiveFlowManager>,
    web_resource_manager: Wrc<dyn WebResourceManager>,

    pub plugins: PluginProxies,
    pub libraries: PluginLibraries,
}

#[async_trait]
#[provides]
impl PluginRegistry for PluginRegistryImpl {
    fn has(&self, name: String) -> bool {
        self.get(name).is_some()
    }

    fn get(&self, name: String) -> Option<Arc<PluginProxy>> {
        let reader = self.plugins.0.read().unwrap();
        let plugin_proxy = reader.get(&name);
        if plugin_proxy.is_some() {
            return Some(plugin_proxy.unwrap().clone());
        }
        None
    }

    fn load_plugins(&self) {
        // Load list of plugins from TOML
        let toml_config = std::fs::read_to_string("./config/plugins.toml");
        match toml_config {
            Ok(toml_string) => {
                let plugins_config: Result<PluginsConfig, _> = toml::from_str(&toml_string);
                match plugins_config {
                    Ok(plugins_config) => {
                        for plugin_config in plugins_config.plugin.iter() {
                            if plugin_config.active {
                                self.load_plugin(plugin_config.name.clone(), plugin_config.path.clone());
                            }
                        }
                    }
                    Err(_) => {
                        error!("Failed to load plugin configuration from {}: Invalid TOML:", "./config/plugins.toml");
                    }
                }
            }
            Err(_) => {
                error!("Failed to load plugin configuration from {}", "./config/plugins.toml");
            }
        }
    }

    fn load_plugin(&self, name: String, path: String) {
        unsafe {
            let result = self.load(path.clone());
            if result.is_err() {
                error!("Failed to load plugin {} from {}", name.clone(), path.clone());
                return;
            }
            let plugin_proxy = self.get(name.clone());
            match plugin_proxy {
                Some(plugin_proxy) => {
                    if plugin_proxy.init().is_ok() {
                        match plugin_proxy.get_component_provider() {
                            Ok(component_provider) => self.component_manager.add_provider(component_provider),
                            Err(_) => {}
                        }
                        match plugin_proxy.get_entity_type_provider() {
                            Ok(entity_type_provider) => self.entity_type_manager.add_provider(entity_type_provider),
                            Err(_) => {}
                        }
                        match plugin_proxy.get_relation_type_provider() {
                            Ok(relation_type_provider) => self.relation_type_manager.add_provider(relation_type_provider),
                            Err(_) => {}
                        }
                        match plugin_proxy.get_component_behaviour_provider() {
                            Ok(component_behaviour_provider) => self.component_behaviour_manager.add_provider(component_behaviour_provider),
                            Err(_) => {}
                        }
                        match plugin_proxy.get_entity_behaviour_provider() {
                            Ok(entity_behaviour_provider) => self.entity_behaviour_manager.add_provider(entity_behaviour_provider),
                            Err(_) => {}
                        }
                        match plugin_proxy.get_relation_behaviour_provider() {
                            Ok(relation_behaviour_provider) => self.relation_behaviour_manager.add_provider(relation_behaviour_provider),
                            Err(_) => {}
                        }
                        match plugin_proxy.get_flow_provider() {
                            Ok(flow_provider) => self.reactive_flow_manager.add_provider(flow_provider),
                            Err(_) => {}
                        }
                        match plugin_proxy.get_web_resource_provider() {
                            Ok(web_resource_provider) => self.web_resource_manager.add_provider(web_resource_provider),
                            Err(_) => {}
                        }
                        let component_manager = ComponentManagerImpl::new(self.component_manager.clone());
                        let entity_type_manager = EntityTypeManagerImpl::new(self.entity_type_manager.clone());
                        let relation_type_manager = RelationTypeManagerImpl::new(self.relation_type_manager.clone());
                        let entity_instance_manager =
                            EntityInstanceManagerImpl::new(self.entity_type_manager.clone(), self.reactive_entity_instance_manager.clone());
                        let relation_instance_manager =
                            RelationInstanceManagerImpl::new(self.relation_type_manager.clone(), self.reactive_relation_instance_manager.clone());
                        let flow_manager = FlowManagerImpl::new(self.reactive_flow_manager.clone());
                        let plugin_context = PluginContextImpl::new(
                            Arc::new(component_manager),
                            Arc::new(entity_type_manager),
                            Arc::new(relation_type_manager),
                            Arc::new(entity_instance_manager),
                            Arc::new(relation_instance_manager),
                            Arc::new(flow_manager),
                        );
                        let context = Arc::new(plugin_context);
                        let _ = plugin_proxy.set_context(context);
                        let _ = plugin_proxy.post_init();
                    }
                }
                None => {
                    error!("Failed to initialize plugin {} from {}", name.clone(), path.clone());
                    // TODO: Handle error: plugin with name not found
                }
            }
        }
    }

    fn unload_plugins(&self) {
        // TODO: Implement an unloading mechanism (that is safe)
        // TODO: Also implement an reloading mechanism (that is safe)
        // TODO: Also implement an deploy mechanism (dropping a dynamically linked library into a specific folder -> load plugin automatically)
        // TODO: Also implement a file watcher (when the library file has been overwritten -> unload old version, load new reload library)
        // // Shutdown all plugins
        // for plugin in self.plugins.0.plugins.iter().rev() {
        //     plugin.pre_shutdown();
        // }
        // // TODO: Reverse: remove providers
        // for plugin in self.plugins.0.plugins.iter().rev() {
        //     plugin.shutdown();
        // }
    }

    /// Load a plugin library and add all contained functions to the internal
    /// function table.
    ///
    /// # Safety
    ///
    /// A plugin library **must** be implemented using the
    /// [`plugins_core::plugin_declaration!()`] macro. Trying manually implement
    /// a plugin without going through that macro will result in undefined
    /// behaviour.
    unsafe fn load(&self, library_path: String) -> Result<(), PluginError> {
        debug!("Loading library");
        // Load the library into memory
        // <P: AsRef<OsStr>>
        let library_path = OsStr::new(library_path.as_str());
        let library = Library::new(library_path);
        match library {
            Ok(library) => {
                let library = Arc::new(library);
                // Get a pointer to the plugin_declaration symbol.
                let decl = library.get::<*mut PluginDeclaration>(b"plugin_declaration\0")?.read();
                // version checks to prevent accidental ABI incompatibilities
                if decl.rustc_version != RUSTC_VERSION {
                    error!("Version mismatch: rustc");
                    return Err(PluginError::Other {
                        message: String::from("Version mismatch: rustc"),
                    }
                    .into());
                }
                if decl.inexor_rgf_plugin_version != INEXOR_RGF_PLUGIN_VERSION {
                    error!("Version mismatch: inexor_rgf_core_plugins");
                    return Err(PluginError::Other {
                        message: String::from("Version mismatch: inexor_rgf_core_plugins"),
                    }
                    .into());
                }

                let mut registrar = PluginRegistrar::new(Arc::clone(&library));

                (decl.register)(&mut registrar);

                // add all loaded plugins to the plugins map
                self.plugins.0.write().unwrap().extend(registrar.plugins);
                // self.plugins.extend(registrar.plugins);
                // and make sure PluginRegistry keeps a reference to the library
                self.libraries.0.write().unwrap().push(library);

                Ok(())
            }
            Err(e) => {
                error!("Failed to load dynamic library: {}", e.to_string());
                return Err(PluginError::PluginCreationError);
            }
        }
    }

    fn plugin_init(&self, name: String) -> Result<(), PluginError> {
        let plugin_proxy = self.get(name.clone());
        match plugin_proxy {
            Some(plugin_proxy) => {
                debug!("Initializing plugin {}", name.clone());
                plugin_proxy.init()
            }
            None => {
                error!("Failed to initialize plugin {}: Not found", name.clone());
                return Err(PluginError::InitializationError);
            }
        }
    }

    fn plugin_post_init(&self, name: String) -> Result<(), PluginError> {
        let plugin_proxy = self.get(name.clone());
        match plugin_proxy {
            Some(plugin_proxy) => {
                debug!("Post-Initializing plugin {}", name.clone());
                plugin_proxy.post_init()
            }
            None => {
                error!("Failed to post-initialize plugin {}: Not found", name.clone());
                return Err(PluginError::PostInitializationError);
            }
        }
    }

    fn plugin_pre_shutdown(&self, name: String) -> Result<(), PluginError> {
        let plugin_proxy = self.get(name.clone());
        match plugin_proxy {
            Some(plugin_proxy) => {
                debug!("Pre-Shutdown plugin {}", name.clone());
                plugin_proxy.pre_shutdown()
            }
            None => {
                error!("Failed to pre-shutdown plugin {}: Not found", name.clone());
                return Err(PluginError::PreShutdownError);
            }
        }
    }

    fn plugin_shutdown(&self, name: String) -> Result<(), PluginError> {
        let plugin_proxy = self.get(name.clone());
        match plugin_proxy {
            Some(plugin_proxy) => {
                debug!("Shutting down plugin {}", name.clone());
                plugin_proxy.shutdown()
            }
            None => {
                error!("Failed to shutdown plugin {}: Not found", name.clone());
                return Err(PluginError::ShutdownError);
            }
        }
    }
}

impl Lifecycle for PluginRegistryImpl {
    fn init(&self) {
        self.load_plugins();
    }

    fn shutdown(&self) {
        self.unload_plugins();
    }
}
