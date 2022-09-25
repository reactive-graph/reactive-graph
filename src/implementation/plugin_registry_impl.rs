use std::collections::HashMap;
use std::ffi::OsStr;
use std::sync::Arc;
use std::sync::RwLock;

use crate::plugins::plugin::PluginInitializationError;
use crate::plugins::plugin::PluginPostInitializationError;
use crate::plugins::plugin::PluginPreShutdownError;
use crate::plugins::plugin::PluginShutdownError;
use crate::plugins::PluginLoadingError;
use async_trait::async_trait;
use libloading::Library;
use log::error;
use log::{debug, info};

use crate::api::ComponentBehaviourManager;
use crate::api::ComponentManager;
use crate::api::EntityBehaviourManager;
use crate::api::EntityTypeManager;
use crate::api::FlowTypeManager;
use crate::api::GraphQLQueryService;
use crate::api::Lifecycle;
use crate::api::PluginRegistry;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveFlowInstanceManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::RelationBehaviourManager;
use crate::api::RelationTypeManager;
use crate::api::WebResourceManager;
use crate::di::*;
use crate::plugin::registrar::PluginRegistrar;
use crate::plugin::ComponentManagerImpl;
use crate::plugin::EntityInstanceManagerImpl;
use crate::plugin::EntityTypeManagerImpl;
use crate::plugin::FlowInstanceManagerImpl;
use crate::plugin::FlowTypeManagerImpl;
use crate::plugin::GraphQLQueryServiceImpl;
use crate::plugin::PluginContextImpl;
use crate::plugin::PluginProxy;
use crate::plugin::PluginsConfig;
use crate::plugin::RelationInstanceManagerImpl;
use crate::plugin::RelationTypeManagerImpl;
use crate::plugins::Plugin;
use crate::plugins::PluginDeclaration;
use crate::plugins::INEXOR_RGF_PLUGIN_VERSION;
use crate::plugins::RUSTC_VERSION;

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
    flow_type_manager: Wrc<dyn FlowTypeManager>,
    graphql_query_service: Wrc<dyn GraphQLQueryService>,
    relation_behaviour_manager: Wrc<dyn RelationBehaviourManager>,
    relation_type_manager: Wrc<dyn RelationTypeManager>,
    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,
    reactive_relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,
    reactive_flow_instance_manager: Wrc<dyn ReactiveFlowInstanceManager>,
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
        reader.get(&name).cloned()
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
                error!("Failed to load plugin {} from {}", name, path);
                return;
            }
            let plugin_proxy = self.get(name.clone());
            match plugin_proxy {
                Some(plugin_proxy) => {
                    if let Ok(metadata) = plugin_proxy.metadata() {
                        info!(
                            "Loading plugin\n    name       : {}\n    version    : {}\n    description: {}\n    depends on : {}",
                            metadata.name,
                            metadata.version,
                            metadata.description,
                            metadata.depends_on.join(",")
                        );
                    }
                    if plugin_proxy.init().is_ok() {
                        if let Ok(Some(component_provider)) = plugin_proxy.get_component_provider() {
                            self.component_manager.add_provider(component_provider);
                        }
                        if let Ok(Some(entity_type_provider)) = plugin_proxy.get_entity_type_provider() {
                            self.entity_type_manager.add_provider(entity_type_provider);
                        }
                        if let Ok(Some(relation_type_provider)) = plugin_proxy.get_relation_type_provider() {
                            self.relation_type_manager.add_provider(relation_type_provider);
                        }
                        if let Ok(Some(flow_type_provider)) = plugin_proxy.get_flow_type_provider() {
                            self.flow_type_manager.add_provider(flow_type_provider);
                        }
                        if let Ok(Some(component_behaviour_provider)) = plugin_proxy.get_component_behaviour_provider() {
                            self.component_behaviour_manager.add_provider(component_behaviour_provider);
                        }
                        if let Ok(Some(entity_behaviour_provider)) = plugin_proxy.get_entity_behaviour_provider() {
                            self.entity_behaviour_manager.add_provider(entity_behaviour_provider);
                        }
                        if let Ok(Some(relation_behaviour_provider)) = plugin_proxy.get_relation_behaviour_provider() {
                            self.relation_behaviour_manager.add_provider(relation_behaviour_provider);
                        }
                        if let Ok(Some(flow_instance_provider)) = plugin_proxy.get_flow_instance_provider() {
                            self.reactive_flow_instance_manager.add_provider(flow_instance_provider);
                        }
                        if let Ok(Some(web_resource_provider)) = plugin_proxy.get_web_resource_provider() {
                            self.web_resource_manager.add_provider(web_resource_provider);
                        }
                        let component_manager = ComponentManagerImpl::new(self.component_manager.clone());
                        let entity_type_manager = EntityTypeManagerImpl::new(self.entity_type_manager.clone());
                        let relation_type_manager = RelationTypeManagerImpl::new(self.relation_type_manager.clone());
                        let flow_type_manager = FlowTypeManagerImpl::new(self.flow_type_manager.clone());
                        let entity_instance_manager =
                            EntityInstanceManagerImpl::new(self.entity_type_manager.clone(), self.reactive_entity_instance_manager.clone());
                        let relation_instance_manager =
                            RelationInstanceManagerImpl::new(self.relation_type_manager.clone(), self.reactive_relation_instance_manager.clone());
                        let flow_instance_manager = FlowInstanceManagerImpl::new(self.reactive_flow_instance_manager.clone());
                        let graphql_query_service = GraphQLQueryServiceImpl::new(self.graphql_query_service.clone());
                        let plugin_context = PluginContextImpl::new(
                            Arc::new(component_manager),
                            Arc::new(entity_type_manager),
                            Arc::new(relation_type_manager),
                            Arc::new(flow_type_manager),
                            Arc::new(entity_instance_manager),
                            Arc::new(relation_instance_manager),
                            Arc::new(flow_instance_manager),
                            Arc::new(graphql_query_service),
                        );
                        let context = Arc::new(plugin_context);
                        let _ = plugin_proxy.set_context(context);
                        let _ = plugin_proxy.post_init();
                    }
                }
                None => {
                    error!("Failed to initialize plugin {} from {}", name, path);
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
        // Shutdown all plugins
        let reader = self.plugins.0.read().unwrap();
        // TODO: correct (reverse) order
        for (name, plugin) in reader.iter() {
            if let Err(err) = plugin.pre_shutdown() {
                error!("Failed to shutdown plugin {}: {:?}", name, err);
            }
        }
        // TODO: correct (reverse) order
        for (name, plugin) in reader.iter() {
            if let Err(err) = plugin.shutdown() {
                error!("Failed to shutdown plugin {}: {:?}", name, err);
            }
        }
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
    unsafe fn load(&self, library_path: String) -> Result<(), PluginLoadingError> {
        debug!("Loading library {}", library_path.as_str());
        // Load the library into memory
        // <P: AsRef<OsStr>>
        let library_path_os = OsStr::new(library_path.as_str());
        let library = Library::new(library_path_os);
        match library {
            Ok(library) => {
                let library = Arc::new(library);
                // Get a pointer to the plugin_declaration symbol.
                let decl = library.get::<*mut PluginDeclaration>(b"plugin_declaration\0")?.read();
                // version checks to prevent accidental ABI incompatibilities
                if decl.rustc_version != RUSTC_VERSION {
                    error!(
                        "Cannot load plugin {} because of a compiler version mismatch: rustc {} (expected: {})",
                        library_path.as_str(),
                        decl.rustc_version,
                        RUSTC_VERSION
                    );
                    // error!("Plugin {} Version mismatch: rustc {} expected {}", library_path.clone(), decl.rustc_version, RUSTC_VERSION);
                    return Err(PluginLoadingError::CompilerVersionMismatch);
                }
                if decl.inexor_rgf_plugin_version != INEXOR_RGF_PLUGIN_VERSION {
                    error!(
                        "Cannot load plugin {} because of an API version mismatch: inexor_rgf_core_plugins {} (expected: {})",
                        library_path.as_str(),
                        decl.inexor_rgf_plugin_version,
                        INEXOR_RGF_PLUGIN_VERSION
                    );
                    return Err(PluginLoadingError::PluginApiVersionMismatch);
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
                Err(PluginLoadingError::LoadingDynamicLibraryFailed)
            }
        }
    }

    fn plugin_init(&self, name: String) -> Result<(), PluginInitializationError> {
        let plugin_proxy = self.get(name.clone());
        match plugin_proxy {
            Some(plugin_proxy) => {
                debug!("Initializing plugin {}", name);
                plugin_proxy.init()
            }
            None => {
                error!("Failed to initialize plugin {}: Not found", name);
                Err(PluginInitializationError::InitializationFailed)
            }
        }
    }

    fn plugin_post_init(&self, name: String) -> Result<(), PluginPostInitializationError> {
        let plugin_proxy = self.get(name.clone());
        match plugin_proxy {
            Some(plugin_proxy) => {
                debug!("Post-Initializing plugin {}", name);
                plugin_proxy.post_init()
            }
            None => {
                error!("Failed to post-initialize plugin {}: Not found", name);
                Err(PluginPostInitializationError::PostInitializationFailed)
            }
        }
    }

    fn plugin_pre_shutdown(&self, name: String) -> Result<(), PluginPreShutdownError> {
        let plugin_proxy = self.get(name.clone());
        match plugin_proxy {
            Some(plugin_proxy) => {
                debug!("Pre-Shutdown plugin {}", name);
                plugin_proxy.pre_shutdown()
            }
            None => {
                error!("Failed to pre-shutdown plugin {}: Not found", name);
                Err(PluginPreShutdownError::PreShutdownFailed)
            }
        }
    }

    fn plugin_shutdown(&self, name: String) -> Result<(), PluginShutdownError> {
        let plugin_proxy = self.get(name.clone());
        match plugin_proxy {
            Some(plugin_proxy) => {
                debug!("Shutting down plugin {}", name);
                plugin_proxy.shutdown()
            }
            None => {
                error!("Failed to shutdown plugin {}: Not found", name);
                Err(PluginShutdownError::ShutdownFailed)
            }
        }
    }
}

impl Lifecycle for PluginRegistryImpl {
    fn init(&self) {
        // TODO: Build dependency tree
        self.load_plugins();
    }

    fn shutdown(&self) {
        self.unload_plugins();
    }
}
