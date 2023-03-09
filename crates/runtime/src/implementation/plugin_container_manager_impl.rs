use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use dashmap::DashSet;
use log::debug;
use log::trace;
use semver::Version;
use semver::VersionReq;
use uuid::Uuid;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::FlowTypeManager;
use crate::api::Lifecycle;
use crate::api::PluginContainerManager;
use crate::api::ReactiveFlowInstanceManager;
use crate::api::RelationTypeManager;
use crate::api::WebResourceManager;
use crate::di::*;
use crate::plugin::PluginContainer;
use crate::plugin::PluginTransitionResult;
use crate::plugin::PluginTransitionResult::Changed;
use crate::plugin::PluginTransitionResult::NoChange;
use crate::plugins::Plugin;
use crate::plugins::PluginContext;
use crate::plugins::PluginDependency;
use crate::plugins::PluginDeployError;
use crate::plugins::PluginRefreshingState;
use crate::plugins::PluginResolveState;
use crate::plugins::PluginStartError;
use crate::plugins::PluginStartingState;
use crate::plugins::PluginState;
use crate::plugins::PluginStopError;
use crate::plugins::PluginStoppingState;
use crate::plugins::PluginUninstallError;
use crate::plugins::PluginUninstallingState;

#[wrapper]
pub struct PluginContainerStorage(DashMap<Uuid, PluginContainer>);

#[provides]
fn plugin_container_storage() -> PluginContainerStorage {
    PluginContainerStorage(DashMap::new())
}

#[component]
pub struct PluginContainerManagerImpl {
    /// The plugin containers.
    pub plugin_containers: PluginContainerStorage,
}

impl PluginContainerManagerImpl {}

#[async_trait]
#[provides]
impl PluginContainerManager for PluginContainerManagerImpl {
    fn create_and_register_plugin_container(&self, stem: String, path: PathBuf) -> Option<Uuid> {
        if self.has(&stem) {
            return None;
        }
        trace!("Creating plugin container for plugin {} located at {}", &stem, path.display());
        let plugin_container = PluginContainer::new(stem.clone(), path);
        let id = plugin_container.id;
        trace!("Registering plugin container {} located at {}", &id, &stem);
        self.plugin_containers.0.insert(id, plugin_container);
        Some(id)
    }

    fn remove_plugin_container(&self, id: &Uuid) {
        self.plugin_containers.0.remove(id);
    }

    fn has(&self, stem: &str) -> bool {
        self.plugin_containers.0.iter().any(|p| p.stem.eq(stem))
    }

    fn get_id(&self, stem: &str) -> Option<Uuid> {
        self.plugin_containers
            .0
            .iter()
            .find(|p| p.stem.eq(stem) || p.name().map(|name| name.eq(stem)).unwrap_or(false))
            .map(|p| p.id)
    }

    fn get_stem(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.0.get(id).map(|p| p.value().stem.clone())
    }

    fn name(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.0.get(id).and_then(|p| p.value().name())
    }

    fn description(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.0.get(id).and_then(|p| p.value().description())
    }

    fn version(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.0.get(id).and_then(|p| p.value().version())
    }

    fn rustc_version(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.0.get(id).and_then(|p| p.value().rustc_version())
    }

    fn plugin_api_version(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.0.get(id).and_then(|p| p.value().plugin_api_version())
    }

    fn count(&self) -> usize {
        self.plugin_containers.0.len()
    }

    fn count_by_state(&self, state: &PluginState) -> usize {
        self.plugin_containers.0.iter().filter(|p| &p.state == state).count()
    }

    fn count_by_states(&self) -> String {
        let states = vec![
            PluginState::Installed,
            PluginState::Resolving(PluginResolveState::Loaded),
            PluginState::Resolving(PluginResolveState::PluginDeclarationLoaded),
            PluginState::Resolving(PluginResolveState::CompilerVersionMismatch),
            PluginState::Resolving(PluginResolveState::PluginApiVersionMismatch),
            PluginState::Resolving(PluginResolveState::DependenciesNotActive),
            PluginState::Resolving(PluginResolveState::PluginCompatible),
            PluginState::Resolved,
            PluginState::Starting(PluginStartingState::ConstructingProxy),
            PluginState::Starting(PluginStartingState::InjectingContext),
            PluginState::Starting(PluginStartingState::Registering),
            PluginState::Starting(PluginStartingState::Activating),
            PluginState::Active,
            PluginState::Stopping(PluginStoppingState::Deactivating),
            PluginState::Stopping(PluginStoppingState::Unregistering),
            PluginState::Stopping(PluginStoppingState::RemoveContext),
            PluginState::Stopping(PluginStoppingState::RemoveProxy),
            PluginState::Uninstalling(PluginUninstallingState::UnloadDll),
            PluginState::Uninstalling(PluginUninstallingState::UninstallDll),
            PluginState::Uninstalled,
            PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Deactivating)),
            PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Unregistering)),
            PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::RemoveContext)),
            PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::RemoveProxy)),
            PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UnloadDll)),
            PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UninstallDll)),
            PluginState::Refreshing(PluginRefreshingState::Deploying),
            PluginState::Refreshing(PluginRefreshingState::Installed),
            PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::Loaded)),
            PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::PluginDeclarationLoaded)),
            PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::CompilerVersionMismatch)),
            PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::PluginApiVersionMismatch)),
            PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::DependenciesNotActive)),
            PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::PluginCompatible)),
            PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::ConstructingProxy)),
            PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::InjectingContext)),
            PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Registering)),
            PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Activating)),
        ];
        states.iter().map(|state| self.count_by_state_str(state)).collect()
    }

    fn count_by_state_str(&self, state: &PluginState) -> String {
        let count = self.count_by_state(state);
        if count > 0 {
            format!("\n  {:?}: {}", state, self.count_by_state(state))
        } else {
            "".to_owned()
        }
    }

    fn get_plugin_path(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.0.get(id).map(|e| String::from(e.path.to_string_lossy()))
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

    fn get_plugins_with_states(&self, state1: PluginState, state2: PluginState) -> Vec<Uuid> {
        self.plugin_containers
            .0
            .iter()
            .filter(|p| p.state == state1 || p.state == state2)
            .map(|p| p.key().to_owned())
            .collect()
    }

    fn get_plugins_not_having_state(&self, state: PluginState) -> Vec<Uuid> {
        self.plugin_containers
            .0
            .iter()
            .filter(|p| p.state != state)
            .map(|p| p.key().to_owned())
            .collect()
    }

    fn get_plugin_by_dependency(&self, plugin_dependency: &PluginDependency) -> Option<Uuid> {
        let version_requirement = VersionReq::parse(plugin_dependency.version).ok()?;
        self.plugin_containers
            .0
            .iter()
            .find(|e| {
                let reader = e.plugin_declaration.read().unwrap();
                match *reader {
                    Some(plugin_declaration) => {
                        plugin_declaration.name == plugin_dependency.name
                            && Version::parse(plugin_declaration.version)
                                .map(|version| version_requirement.matches(&version))
                                .unwrap_or(false)
                    }
                    None => false,
                }
            })
            .map(|e| e.key().clone())
    }

    fn deploy_dll(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                trace!("Plugin {} is deploying the dynamic linked library", id);
                plugin_container.deploy_dll()
            }
            None => NoChange,
        }
    }

    fn load_dll(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                trace!("Plugin {} is loading the dynamic linked library", id);
                plugin_container.load_dll()
            }
            None => NoChange,
        }
    }

    fn load_plugin_declaration(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                trace!("Plugin {} is loading the plugin declaration", id);
                plugin_container.value_mut().load_plugin_declaration()
            }
            None => NoChange,
        }
    }

    fn check_plugin_compatibility(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                trace!("Plugin {} is checked for compatibility", id);
                plugin_container.value_mut().check_compatibility()
            }
            None => NoChange,
        }
    }

    fn load_plugin_dependencies(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                trace!("Plugin {} is loading the list of dependencies", id);
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

    fn get_unsatisfied_dependencies(&self, id: &Uuid) -> DashSet<PluginDependency> {
        self.get_dependencies(id)
            .iter()
            .filter(|d| self.get_dependency_state(&d) != PluginState::Active)
            .map(|d| d.clone())
            .collect()
    }

    fn get_dependents(&self, id: &Uuid) -> Vec<Uuid> {
        let mut dependents = Vec::new();
        for plugin_container in self.plugin_containers.0.iter() {
            for dependency in plugin_container.dependencies.iter() {
                if let Some(dependency_id) = self.get_plugin_by_dependency(&dependency) {
                    if &dependency_id == id {
                        dependents.push(plugin_container.id);
                    }
                }
            }
        }
        dependents
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

    fn resolve_dependencies_state(&self, id: &Uuid, refreshing: bool) -> PluginTransitionResult {
        if !self.has_unsatisfied_dependencies(id) {
            debug!("Plugin {} has no unsatisfied dependencies", id);
            let new_state = if refreshing {
                PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::ConstructingProxy))
            } else {
                PluginState::Resolved
            };
            self.set_state(id, new_state)
        } else {
            trace!("Plugin {} has unsatisfied dependencies", id);
            let new_state = if refreshing {
                PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::DependenciesNotActive))
            } else {
                PluginState::Resolving(PluginResolveState::DependenciesNotActive)
            };
            self.set_state(id, new_state)
        }
    }

    fn construct_proxy(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => plugin_container.construct_proxy(),
            None => NoChange,
        }
    }

    fn inject_context(&self, id: &Uuid, plugin_context: Arc<dyn PluginContext>) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                return plugin_container.inject_context(plugin_context.clone());
            }
            None => NoChange,
        }
    }

    fn register(
        &self,
        id: &Uuid,
        component_manager: Arc<dyn ComponentManager>,
        entity_type_manager: Arc<dyn EntityTypeManager>,
        relation_type_manager: Arc<dyn RelationTypeManager>,
        flow_type_manager: Arc<dyn FlowTypeManager>,
        reactive_flow_instance_manager: Arc<dyn ReactiveFlowInstanceManager>,
        web_resource_manager: Arc<dyn WebResourceManager>,
    ) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                if plugin_container.state != PluginState::Starting(PluginStartingState::Registering)
                    && plugin_container.state != PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Registering))
                {
                    return NoChange;
                }
                let refreshing = plugin_container.state == PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Registering));
                let mut changed = false;
                {
                    let reader = plugin_container.proxy.read().unwrap();
                    if let Some(proxy) = reader.as_ref().map(|proxy| proxy.clone()) {
                        trace!("Plugin {} is registering providers", id);

                        //
                        // TODO: Move "add_provider" calls to the "activate" or a new "register" method of each plugin?
                        //

                        if let Ok(Some(component_provider)) = proxy.get_component_provider() {
                            component_manager.add_provider(component_provider);
                        }
                        if let Ok(Some(entity_type_provider)) = proxy.get_entity_type_provider() {
                            entity_type_manager.add_provider(entity_type_provider);
                        }
                        if let Ok(Some(relation_type_provider)) = proxy.get_relation_type_provider() {
                            relation_type_manager.add_provider(relation_type_provider);
                        }
                        if let Ok(Some(flow_type_provider)) = proxy.get_flow_type_provider() {
                            flow_type_manager.add_provider(flow_type_provider);
                        }
                        if let Ok(Some(flow_instance_provider)) = proxy.get_flow_instance_provider() {
                            reactive_flow_instance_manager.add_provider(id.clone(), flow_instance_provider);
                        }
                        if let Ok(Some(web_resource_provider)) = proxy.get_web_resource_provider() {
                            web_resource_manager.add_provider(id.clone(), web_resource_provider);
                        }
                        changed = true;
                    }
                }
                if changed {
                    if refreshing {
                        plugin_container.state = PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Activating));
                    } else {
                        plugin_container.state = PluginState::Starting(PluginStartingState::Activating);
                    }
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
            PluginState::Refreshing(PluginRefreshingState::Stopping(_)) => false,
            PluginState::Refreshing(PluginRefreshingState::Uninstalling(_)) => true,
            PluginState::Refreshing(PluginRefreshingState::Deploying) => true,
            PluginState::Refreshing(PluginRefreshingState::Installed) => true,
            PluginState::Refreshing(PluginRefreshingState::Resolving(_)) => true,
            PluginState::Refreshing(PluginRefreshingState::Starting(_)) => false,
            PluginState::Uninstalling(_) => true,
            PluginState::Uninstalled => true,
        })
    }

    fn deactivate(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => plugin_container.deactivate(),
            None => NoChange,
        }
    }

    fn unregister(
        &self,
        id: &Uuid,
        reactive_flow_instance_manager: Arc<dyn ReactiveFlowInstanceManager>,
        web_resource_manager: Arc<dyn WebResourceManager>,
    ) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => {
                if plugin_container.state != PluginState::Stopping(PluginStoppingState::Unregistering)
                    && plugin_container.state != PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Unregistering))
                {
                    return NoChange;
                }
                let refreshing = plugin_container.state == PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Unregistering));

                //
                // TODO: Move "add_provider" calls to the "deactivate" or a new "unregister" method of each plugin?
                //

                // self.component_manager.remove_provider(id);
                // self.component_manager.remove_provider(id);
                // self.entity_type_manager.remove_provider(id);
                // self.relation_type_manager.remove_provider(id);
                // self.flow_type_manager.remove_provider(id);
                reactive_flow_instance_manager.remove_provider(id);
                web_resource_manager.remove_provider(id);
                if refreshing {
                    plugin_container.state = PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::RemoveContext));
                } else {
                    plugin_container.state = PluginState::Stopping(PluginStoppingState::RemoveContext);
                }
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

    fn uninstall_dll(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => match plugin_container.value_mut().uninstall_dll() {
                Changed => {
                    self.plugin_containers.0.remove(id);
                    Changed
                }
                NoChange => NoChange,
            },
            None => NoChange,
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

    fn start_dependent_with_satisfied_dependencies(&self, id: &Uuid) -> bool {
        let mut starting_at_least_one = false;
        for dependent_id in self.get_dependents(id) {
            if let Some(dependency_state) = self.get_plugin_state(&dependent_id) {
                match dependency_state {
                    PluginState::Resolved => {
                        // Starting dependent plugin which is now resolved
                        trace!("Starting {:?} dependent plugin {}", dependency_state, &dependent_id);
                        if self.start(&dependent_id).is_ok() {
                            starting_at_least_one = true;
                        }
                    }
                    PluginState::Active => {
                        // Recursively starting dependent plugins which are now active
                        trace!("Recursively resolving {:?} dependent plugins of {}", dependency_state, &dependent_id);
                        if self.start_dependent_with_satisfied_dependencies(&dependent_id) {
                            starting_at_least_one = true;
                        }
                    }
                    _ => {}
                }
            }
        }
        starting_at_least_one
    }

    fn stop(&self, id: &Uuid) -> Result<(), PluginStopError> {
        let result = match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => plugin_container.stop(),
            None => Err(PluginStopError::Uninstalled),
        };
        while self.stop_active_with_unsatisfied_dependencies() {}
        result
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

    fn stop_active_with_unsatisfied_dependencies(&self) -> bool {
        let mut stopping_at_least_one = false;
        for id in self.get_plugins() {
            if let Some(state) = self.get_plugin_state(&id) {
                if state == PluginState::Active && self.has_unsatisfied_dependencies(&id) && self.stop(&id).is_ok() {
                    stopping_at_least_one = true;
                }
            }
        }
        stopping_at_least_one
    }

    fn uninstall(&self, id: &Uuid) -> Result<(), PluginUninstallError> {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => plugin_container.uninstall(),
            None => Err(PluginUninstallError::AlreadyUninstalled),
        }
    }

    fn redeploy(&self, id: &Uuid) -> Result<(), PluginDeployError> {
        match self.plugin_containers.0.get_mut(id) {
            Some(mut plugin_container) => plugin_container.redeploy(),
            None => Err(PluginDeployError::NotFound),
        }
    }
}

impl Lifecycle for PluginContainerManagerImpl {}
