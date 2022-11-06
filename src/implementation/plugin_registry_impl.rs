use std::env::consts::DLL_EXTENSION;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use async_trait::async_trait;
use dashmap::DashMap;
use dashmap::DashSet;
use inexor_rgf_core_plugins::plugin_state::PluginStartError;
use inexor_rgf_core_plugins::plugin_state::PluginStopError;
use inexor_rgf_core_plugins::Plugin;
use log::info;
use notify::RecursiveMode;
use notify::Watcher;
use uuid::Uuid;

use crate::api::ComponentBehaviourManager;
use crate::api::ComponentManager;
use crate::api::EntityBehaviourManager;
use crate::api::EntityTypeManager;
use crate::api::FlowTypeManager;
use crate::api::GraphQLQueryService;
use crate::api::Lifecycle;
use crate::api::PluginRegistry;
use crate::api::PluginRegistryMode;
use crate::api::PluginTransitionResult;
use crate::api::PluginTransitionResult::Changed;
use crate::api::PluginTransitionResult::NoChange;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveFlowInstanceManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::RelationBehaviourManager;
use crate::api::RelationTypeManager;
use crate::api::SystemEventManager;
use crate::api::WebResourceManager;
use crate::di::*;
use crate::plugin::ComponentManagerImpl;
use crate::plugin::EntityInstanceManagerImpl;
use crate::plugin::EntityTypeManagerImpl;
use crate::plugin::FlowInstanceManagerImpl;
use crate::plugin::FlowTypeManagerImpl;
use crate::plugin::GraphQLQueryServiceImpl;
use crate::plugin::PluginContainer;
use crate::plugin::PluginContextImpl;
use crate::plugin::RelationInstanceManagerImpl;
use crate::plugin::RelationTypeManagerImpl;
use crate::plugin::SystemEventManagerImpl;
use crate::plugins::plugin_state::PluginResolveState;
use crate::plugins::plugin_state::PluginStartingState;
use crate::plugins::plugin_state::PluginStoppingState;
use crate::plugins::PluginContext;
use crate::plugins::PluginDependency;
use crate::plugins::PluginState;

#[wrapper]
pub struct PluginContainerStorage(DashMap<Uuid, PluginContainer>);

#[wrapper]
pub struct PluginRegistryModeState(RwLock<PluginRegistryMode>);

#[wrapper]
pub struct PluginContextStorage(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn plugin_container_storage() -> PluginContainerStorage {
    PluginContainerStorage(DashMap::new())
}

#[provides]
fn create_plugin_registry_mode() -> PluginRegistryModeState {
    PluginRegistryModeState(RwLock::new(PluginRegistryMode::Neutral))
}

#[provides]
fn create_plugin_context_storage() -> PluginContextStorage {
    PluginContextStorage(RwLock::new(None))
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
    system_event_manager: Wrc<dyn SystemEventManager>,
    web_resource_manager: Wrc<dyn WebResourceManager>,

    /// The plugin containers.
    pub plugin_containers: PluginContainerStorage,

    /// The mode.
    pub mode: PluginRegistryModeState,

    /// The plugin context.
    pub plugin_context: PluginContextStorage,
}

impl PluginRegistryImpl {
    fn scan_plugin_repository(&self) {
        info!("Scanning plugin repository");
        if let Ok(dir) = fs::read_dir("./plugins/installed") {
            for entry in dir.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if !file_type.is_file() {
                        continue;
                    }
                }
                let path = entry.path();
                if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                    if extension == DLL_EXTENSION {
                        if let Some(stem) = path.clone().file_stem().and_then(|e| e.to_str()) {
                            self.add_plugin(stem, &path);
                        }
                    }
                }
            }
        }
    }

    fn watch_deploy_repository(&self) {
        let watcher = notify::recommended_watcher(|res| match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        });
        match watcher {
            Ok(mut watcher) => {
                // Add a path to be watched. All files and directories at that path and
                // below will be monitored for changes.
                match watcher.watch(Path::new("./deploy"), RecursiveMode::NonRecursive) {
                    Ok(w) => {}
                    Err(e) => {}
                }
            }
            Err(_) => {}
        }
    }

    fn add_plugin(&self, stem: &str, path: &PathBuf) {
        if !self.has(stem) {
            let plugin_container = PluginContainer::new(stem.to_string(), path.clone().into_boxed_path());
            let id = plugin_container.id;
            self.plugin_containers.0.insert(id, plugin_container);
            info!("Detected plugin {} located at {} and assigned id {}", stem, path.display(), id);
        }
    }

    fn rescan_plugin_repository(&self) {
        info!("Scanning plugin repository");
        if let Ok(dir) = fs::read_dir("./plugins") {
            for entry in dir.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if !file_type.is_file() {
                        continue;
                    }
                }
                let path = entry.path();
                if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                    if extension == DLL_EXTENSION {
                        if let Some(id) = path.clone().file_stem().and_then(|e| e.to_str()) {
                            self.refresh_plugin(id, &path);
                        }
                    }
                }
            }
        }
    }

    fn refresh_plugin(&self, id: &str, path: &PathBuf) {
        // if let Some(mut plugin_container) = self.plugin_containers.0.get_mut(id).map(|e| e.value()) {
        //     plugin_container.state = match plugin_container.state {
        //         PluginState::Installed => {}
        //         PluginState::Resolving(_) => {}
        //         PluginState::Resolved => {}
        //         PluginState::Starting => {}
        //         PluginState::Active => PluginState::Refreshing(PluginRefreshingState::Stopping),
        //         PluginState::Stopping => {}
        //         PluginState::Refreshing(_) => {}
        //         PluginState::Uninstalling => {}
        //         PluginState::Uninstalled => {}
        //     }
        //     plugin_container.state = PluginState::Refreshing()
        //     let plugin_container = ;
        //     self.plugin_containers.0.insert(id.to_string(), plugin_container);
        //     info!("Detected plugin {} at {}", id, path.display());
        // }
    }

    fn load_dll(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                info!("Plugin {} is loading the dynamic linked library", id);
                plugin_container.load_dll()
            }
            None => NoChange,
        }
    }

    fn load_plugin_declaration(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                info!("Plugin {} is loading the plugin declaration", id);
                plugin_container.value_mut().load_plugin_declaration()
            }
            None => NoChange,
        }
    }

    fn check_plugin_compatibility(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                info!("Plugin {} is checked for compatibility", id);
                plugin_container.value_mut().check_compatibility()
            }
            None => NoChange,
        }
    }

    fn load_plugin_dependencies(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                info!("Plugin {} is loading the list of dependencies", id);
                plugin_container.value_mut().load_plugin_dependencies()
            }
            None => NoChange,
        }
    }

    fn get_dependency_state(&self, dependency: &PluginDependency) -> PluginState {
        self.get_plugin_by_dependency(dependency)
            .and_then(|id| self.get_plugin_state(&id))
            .unwrap_or(PluginState::Uninstalled)
    }

    fn has_dependencies(&self, id: &Uuid) -> bool {
        self.plugin_containers.0.get(id).iter().any(|e| !e.dependencies.is_empty())
    }

    fn get_dependencies(&self, id: &Uuid) -> DashSet<PluginDependency> {
        self.plugin_containers.0.get(id).map(|e| e.dependencies.clone()).unwrap_or_else(DashSet::new)
    }

    fn has_unsatisfied_dependencies(&self, id: &Uuid) -> bool {
        if !self.has_dependencies(id) {
            return false;
        }
        return !self.get_dependencies(id).iter().all(|d| self.get_dependency_state(&d) == PluginState::Active);
    }

    fn set_state(&self, id: &Uuid, new_state: PluginState) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                if plugin_container.state != new_state {
                    plugin_container.state = new_state;
                    return Changed;
                }
                NoChange
            }
            None => NoChange,
        }
    }

    fn resolve_dependencies_state(&self, id: &Uuid) -> PluginTransitionResult {
        if !self.has_unsatisfied_dependencies(id) {
            info!("Plugin {} has no unsatisfied dependencies", id);
            self.set_state(id, PluginState::Resolved)
        } else {
            info!("Plugin {} has unsatisfied dependencies", id);
            self.set_state(id, PluginState::Resolving(PluginResolveState::DependenciesNotActive))
        }
    }

    fn construct_proxy(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => plugin_container.construct_proxy(),
            None => NoChange,
        }
    }

    fn inject_context(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                let reader = self.plugin_context.0.read().unwrap();
                if let Some(plugin_context) = reader.as_ref() {
                    return plugin_container.inject_context(plugin_context.clone());
                }
                NoChange
            }
            None => NoChange,
        }
    }

    fn register(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                if plugin_container.state != PluginState::Starting(PluginStartingState::Registering) {
                    return NoChange;
                }
                let mut changed = false;
                {
                    let reader = plugin_container.proxy.read().unwrap();
                    if let Some(proxy) = reader.as_ref().map(|proxy| proxy.clone()) {
                        info!("Plugin {} is registering providers", id);
                        if let Ok(Some(component_provider)) = proxy.get_component_provider() {
                            self.component_manager.add_provider(component_provider);
                        }
                        if let Ok(Some(entity_type_provider)) = proxy.get_entity_type_provider() {
                            self.entity_type_manager.add_provider(entity_type_provider);
                        }
                        if let Ok(Some(relation_type_provider)) = proxy.get_relation_type_provider() {
                            self.relation_type_manager.add_provider(relation_type_provider);
                        }
                        if let Ok(Some(flow_type_provider)) = proxy.get_flow_type_provider() {
                            self.flow_type_manager.add_provider(flow_type_provider);
                        }
                        if let Ok(Some(component_behaviour_provider)) = proxy.get_component_behaviour_provider() {
                            self.component_behaviour_manager.add_provider(id.clone(), component_behaviour_provider);
                        }
                        if let Ok(Some(entity_behaviour_provider)) = proxy.get_entity_behaviour_provider() {
                            self.entity_behaviour_manager.add_provider(id.clone(), entity_behaviour_provider);
                        }
                        if let Ok(Some(relation_behaviour_provider)) = proxy.get_relation_behaviour_provider() {
                            self.relation_behaviour_manager.add_provider(id.clone(), relation_behaviour_provider);
                        }
                        if let Ok(Some(flow_instance_provider)) = proxy.get_flow_instance_provider() {
                            self.reactive_flow_instance_manager.add_provider(id.clone(), flow_instance_provider);
                        }
                        if let Ok(Some(web_resource_provider)) = proxy.get_web_resource_provider() {
                            self.web_resource_manager.add_provider(id.clone(), web_resource_provider);
                        }
                        changed = true;
                    }
                }
                if changed {
                    plugin_container.state = PluginState::Starting(PluginStartingState::Activating);
                    return Changed;
                }
                NoChange
            }
            None => NoChange,
        }
    }

    fn activate(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => plugin_container.activate(),
            None => NoChange,
        }
    }

    fn are_all_stopped(&self) -> bool {
        self.plugin_containers.0.iter().all(|p| match p.state.clone() {
            PluginState::Installed => true,
            PluginState::Resolving(_) => true,
            PluginState::Resolved => true,
            PluginState::Starting(_) => false,
            PluginState::Active => false,
            PluginState::Stopping(_) => false,
            PluginState::Refreshing(_) => false,
            PluginState::Uninstalling => true,
            PluginState::Uninstalled => true,
        })
    }

    fn deactivate(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => plugin_container.deactivate(),
            None => NoChange,
        }
    }

    fn unregister(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                if plugin_container.state != PluginState::Stopping(PluginStoppingState::Unregistering) {
                    return NoChange;
                }
                // self.component_manager.remove_provider(id);
                // self.component_manager.remove_provider(id);
                // self.entity_type_manager.remove_provider(id);
                // self.relation_type_manager.remove_provider(id);
                // self.flow_type_manager.remove_provider(id);
                self.component_behaviour_manager.remove_provider(id);
                self.entity_behaviour_manager.remove_provider(id);
                self.relation_behaviour_manager.remove_provider(id);
                self.reactive_flow_instance_manager.remove_provider(id);
                self.web_resource_manager.remove_provider(id);
                plugin_container.state = PluginState::Stopping(PluginStoppingState::RemoveContext);
                return Changed;
            }
            None => NoChange,
        }
    }

    fn remove_context(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => plugin_container.remove_context(),
            None => NoChange,
        }
    }

    fn remove_proxy(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => plugin_container.remove_proxy(),
            None => NoChange,
        }
    }

    fn unload_dll(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => plugin_container.value_mut().unload_dll(),
            None => NoChange,
        }
    }

    fn uninstall(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => match plugin_container.value_mut().uninstall() {
                Changed => {
                    self.plugin_containers.0.remove(id);
                    Changed
                }
                NoChange => NoChange,
            },
            None => NoChange,
        }
    }

    fn resolve(&self) -> PluginTransitionResult {
        info!("Resolving");
        let mode = self.get_mode();
        // Uninstalling --> Uninstalled
        for id in self.get_plugins_with_state(PluginState::Uninstalling) {
            if self.uninstall(&id) == Changed {
                return Changed;
            };
        }
        // PluginResolveState::CompilerVersionMismatch --> Uninstalling
        for id in self.get_plugins_with_state(PluginState::Resolving(PluginResolveState::CompilerVersionMismatch)) {
            if self.set_state(&id, PluginState::Uninstalling) == Changed {
                return Changed;
            }
        }
        // PluginResolveState::PluginApiVersionMismatch --> Uninstalling
        for id in self.get_plugins_with_state(PluginState::Resolving(PluginResolveState::PluginApiVersionMismatch)) {
            if self.set_state(&id, PluginState::Uninstalling) == Changed {
                return Changed;
            }
        }
        // Installed --> PluginResolveState::Loaded
        //           --> Uninstalling
        for id in self.get_plugins_with_state(PluginState::Installed) {
            if self.load_dll(&id) == Changed {
                return Changed;
            }
        }
        // PluginResolveState::Loaded --> PluginResolveState::PluginDeclarationLoaded
        for id in self.get_plugins_with_state(PluginState::Resolving(PluginResolveState::Loaded)) {
            if self.load_plugin_declaration(&id) == Changed {
                return Changed;
            }
        }
        // PluginResolveState::PluginDeclarationLoaded --> PluginResolveState::PluginCompatible
        //                                             --> PluginResolveState::CompilerVersionMismatch
        //                                             --> PluginResolveState::PluginApiVersionMismatch
        for id in self.get_plugins_with_state(PluginState::Resolving(PluginResolveState::PluginDeclarationLoaded)) {
            if self.check_plugin_compatibility(&id) == Changed {
                return Changed;
            }
        }
        // PluginResolveState::PluginCompatible --> PluginResolveState::DependenciesNotActive
        for id in self.get_plugins_with_state(PluginState::Resolving(PluginResolveState::PluginCompatible)) {
            if self.load_plugin_dependencies(&id) == Changed {
                return Changed;
            }
        }
        // PluginResolveState::DependenciesNotActive --> Resolved
        for id in self.get_plugins_with_state(PluginState::Resolving(PluginResolveState::DependenciesNotActive)) {
            if self.resolve_dependencies_state(&id) == Changed {
                return Changed;
            }
        }
        // Resolved --> Starting(ConstructingProxy)
        //          --> PluginResolveState::DependenciesNotActive
        match mode {
            PluginRegistryMode::Starting => {
                for id in self.get_plugins_with_state(PluginState::Resolved) {
                    if self.start(&id).map_err(|_| ()).is_ok() {
                        return Changed;
                    }
                }
            }
            PluginRegistryMode::Neutral => {
                for id in self.get_plugins_with_state(PluginState::Resolved) {
                    if self.resolve_dependencies_state(&id) == Changed {
                        return Changed;
                    }
                }
            }
            PluginRegistryMode::Stopping => {}
        }
        // Starting(ConstructingProxy) --> Starting(InjectingContext)
        for id in self.get_plugins_with_state(PluginState::Starting(PluginStartingState::ConstructingProxy)) {
            if self.construct_proxy(&id) == Changed {
                return Changed;
            }
        }
        // Starting(InjectingContext) --> Starting(Registering)
        for id in self.get_plugins_with_state(PluginState::Starting(PluginStartingState::InjectingContext)) {
            if self.inject_context(&id) == Changed {
                return Changed;
            }
        }
        // Starting(Registering) --> Starting(Activating)
        for id in self.get_plugins_with_state(PluginState::Starting(PluginStartingState::Registering)) {
            if self.register(&id) == Changed {
                return Changed;
            }
        }
        // Starting(Activating) --> Active
        for id in self.get_plugins_with_state(PluginState::Starting(PluginStartingState::Activating)) {
            if self.activate(&id) == Changed {
                return Changed;
            }
        }
        // Stopping(Deactivating) --> Stopping(Unregistering)
        for id in self.get_plugins_with_state(PluginState::Stopping(PluginStoppingState::Deactivating)) {
            if self.deactivate(&id) == Changed {
                return Changed;
            }
        }
        // Stopping(Unregistering) --> Stopping(RemoveContext)
        for id in self.get_plugins_with_state(PluginState::Stopping(PluginStoppingState::Unregistering)) {
            if self.unregister(&id) == Changed {
                return Changed;
            }
        }
        // Stopping(RemoveContext) --> Stopping(RemoveProxy)
        for id in self.get_plugins_with_state(PluginState::Stopping(PluginStoppingState::RemoveContext)) {
            if self.remove_context(&id) == Changed {
                return Changed;
            }
        }
        // Stopping(RemoveProxy) --> Resolved
        for id in self.get_plugins_with_state(PluginState::Stopping(PluginStoppingState::RemoveProxy)) {
            if self.remove_proxy(&id) == Changed {
                return Changed;
            }
        }
        // Active --> Deactivating
        for id in self.get_plugins_with_state(PluginState::Active) {
            if mode == PluginRegistryMode::Stopping {
                return match self.stop(&id) {
                    Ok(_) => Changed,
                    Err(e) => NoChange,
                };
            }
        }
        info!("Resolver finished");
        NoChange
    }

    fn get_plugin_state(&self, id: &Uuid) -> Option<PluginState> {
        self.plugin_containers.0.get(id).map(|e| e.state)
    }

    fn get_plugins(&self) -> Vec<Uuid> {
        self.plugin_containers.0.iter().map(|p| p.key().clone()).collect()
    }

    fn get_plugins_with_state(&self, state: PluginState) -> Vec<Uuid> {
        self.plugin_containers
            .0
            .iter()
            .filter(|p| p.state == state)
            .map(|p| p.key().to_owned())
            .collect()
    }

    fn get_plugin_by_dependency(&self, plugin_dependency: &PluginDependency) -> Option<Uuid> {
        self.plugin_containers
            .0
            .iter()
            .find(|e| {
                let reader = e.plugin_declaration.read().unwrap();
                match *reader {
                    Some(plugin_declaration) => plugin_declaration.name == plugin_dependency.name && plugin_declaration.version.eq(plugin_declaration.version),
                    None => false,
                }
            })
            .map(|e| e.key().clone())
    }

    fn construct_plugin_context(&self) {
        // -> Arc<dyn PluginContext> {
        let component_manager = ComponentManagerImpl::new(self.component_manager.clone());
        let entity_type_manager = EntityTypeManagerImpl::new(self.entity_type_manager.clone());
        let relation_type_manager = RelationTypeManagerImpl::new(self.relation_type_manager.clone());
        let flow_type_manager = FlowTypeManagerImpl::new(self.flow_type_manager.clone());
        let entity_instance_manager = EntityInstanceManagerImpl::new(
            self.component_manager.clone(),
            self.entity_type_manager.clone(),
            self.reactive_entity_instance_manager.clone(),
        );
        let relation_instance_manager = RelationInstanceManagerImpl::new(
            self.component_manager.clone(),
            self.relation_type_manager.clone(),
            self.reactive_relation_instance_manager.clone(),
        );
        let flow_instance_manager = FlowInstanceManagerImpl::new(self.reactive_flow_instance_manager.clone());
        let graphql_query_service = GraphQLQueryServiceImpl::new(self.graphql_query_service.clone());
        let system_event_manager = SystemEventManagerImpl::new(self.system_event_manager.clone());
        let plugin_context = PluginContextImpl::new(
            Arc::new(component_manager),
            Arc::new(entity_type_manager),
            Arc::new(relation_type_manager),
            Arc::new(flow_type_manager),
            Arc::new(entity_instance_manager),
            Arc::new(relation_instance_manager),
            Arc::new(flow_instance_manager),
            Arc::new(graphql_query_service),
            Arc::new(system_event_manager),
        );
        let plugin_context = Arc::new(plugin_context);
        let mut writer = self.plugin_context.0.write().unwrap();
        let _ = writer.insert(plugin_context);
    }
}

#[async_trait]
#[provides]
impl PluginRegistry for PluginRegistryImpl {
    fn has(&self, stem: &str) -> bool {
        self.plugin_containers.0.iter().any(|p| p.stem.eq(stem))
    }

    fn get_id(&self, stem: &str) -> Option<Uuid> {
        self.plugin_containers.0.iter().find(|p| p.stem.eq(stem)).map(|p| p.id)
    }

    fn resolve_until_idle(&self) {
        while self.resolve() == Changed {
            // TODO: timeout + circuit breaker
        }
    }

    fn start(&self, id: &Uuid) -> Result<(), PluginStartError> {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => plugin_container.start(),
            None => Err(PluginStartError::Uninstalled),
        }
    }

    fn start_by_stem(&self, stem: &str) -> Result<(), PluginStartError> {
        if let Some(id) = self.get_id(stem) {
            return self.start(&id);
        }
        Err(PluginStartError::Uninstalled)
    }

    fn stop(&self, id: &Uuid) -> Result<(), PluginStopError> {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => plugin_container.stop(),
            None => Err(PluginStopError::Uninstalled),
        }
    }

    fn stop_by_stem(&self, stem: &str) -> Result<(), PluginStopError> {
        if let Some(id) = self.get_id(stem) {
            return self.stop(&id);
        }
        Err(PluginStopError::Uninstalled)
    }

    fn stop_all(&self) {
        for id in self.get_plugins() {
            // TODO
            self.stop(&id);
        }
    }

    fn set_mode(&self, mode: PluginRegistryMode) {
        let mut writer = self.mode.0.write().unwrap();
        *writer = mode;
    }

    fn get_mode(&self) -> PluginRegistryMode {
        let reader = self.mode.0.read().unwrap();
        *reader
    }
}

impl Lifecycle for PluginRegistryImpl {
    fn init(&self) {
        self.construct_plugin_context();
        self.scan_plugin_repository();
        self.set_mode(PluginRegistryMode::Starting);
        self.resolve_until_idle();
        self.set_mode(PluginRegistryMode::Neutral);
    }

    fn shutdown(&self) {
        self.set_mode(PluginRegistryMode::Stopping);
        self.stop_all();
        while !self.are_all_stopped() {
            self.resolve_until_idle();
            thread::sleep(Duration::from_millis(100));
            // TODO: force stop after timeout
        }
    }
}
