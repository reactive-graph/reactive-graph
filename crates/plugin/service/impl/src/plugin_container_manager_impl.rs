use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use dashmap::DashSet;
use log::debug;
use log::trace;
use semver::Version;
use semver::VersionReq;
use springtime_di::Component;
use springtime_di::component_alias;
use uuid::Uuid;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_api::PluginContext;
use reactive_graph_plugin_api::PluginDependency;
use reactive_graph_plugin_api::PluginDeployError;
use reactive_graph_plugin_api::PluginDisableError;
use reactive_graph_plugin_api::PluginRefreshingState;
use reactive_graph_plugin_api::PluginResolveState;
use reactive_graph_plugin_api::PluginStartError;
use reactive_graph_plugin_api::PluginStartingState;
use reactive_graph_plugin_api::PluginState;
use reactive_graph_plugin_api::PluginStopError;
use reactive_graph_plugin_api::PluginStoppingState;
use reactive_graph_plugin_api::PluginUninstallError;
use reactive_graph_plugin_api::PluginUninstallingState;
use reactive_graph_plugin_service_api::PluginContainerManager;
use reactive_graph_plugin_service_api::PluginTransitionResult;
use reactive_graph_plugin_service_api::PluginTransitionResult::Changed;
use reactive_graph_plugin_service_api::PluginTransitionResult::NoChange;

use crate::PluginContainer;

#[derive(Component)]
pub struct PluginContainerManagerImpl {
    /// The plugin containers.
    #[component(default = "DashMap::new")]
    pub plugin_containers: DashMap<Uuid, PluginContainer>,
}

impl PluginContainerManagerImpl {}

#[async_trait]
#[component_alias]
impl PluginContainerManager for PluginContainerManagerImpl {
    fn create_and_register_plugin_container(&self, stem: String, path: PathBuf) -> Option<Uuid> {
        if self.has(&stem) {
            return None;
        }
        trace!("Creating plugin container for plugin {} located at {}", &stem, path.display());
        let plugin_container = PluginContainer::new(stem.clone(), path);
        let id = plugin_container.id;
        trace!("Registering plugin container {} located at {}", &id, &stem);
        self.plugin_containers.insert(id, plugin_container);
        Some(id)
    }

    fn remove_plugin_container(&self, id: &Uuid) {
        self.plugin_containers.remove(id);
    }

    fn has(&self, stem: &str) -> bool {
        self.plugin_containers.iter().any(|p| p.stem.eq(stem))
    }

    fn get_id(&self, stem: &str) -> Option<Uuid> {
        self.plugin_containers
            .iter()
            .find(|p| p.stem.eq(stem) || p.name().map(|name| name.eq(stem)).unwrap_or(false))
            .map(|p| p.id)
    }

    fn get_stem(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.get(id).map(|p| p.value().stem.clone())
    }

    fn name(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.get(id).and_then(|p| p.value().name())
    }

    fn name_canonicalized(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.get(id).and_then(|p| p.value().name_canonicalized())
    }

    fn name_version(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.get(id).and_then(|p| p.value().name_version())
    }

    fn description(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.get(id).and_then(|p| p.value().description())
    }

    fn version(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.get(id).and_then(|p| p.value().version())
    }

    fn rustc_version(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.get(id).and_then(|p| p.value().rustc_version())
    }

    fn plugin_api_version(&self, id: &Uuid) -> Option<String> {
        self.plugin_containers.get(id).and_then(|p| p.value().plugin_api_version())
    }

    fn count(&self) -> usize {
        self.plugin_containers.len()
    }

    fn count_by_state(&self, state: &PluginState) -> usize {
        self.plugin_containers.iter().filter(|p| &p.state == state).count()
    }

    fn count_by_states(&self) -> String {
        let states = [
            PluginState::Installed,
            PluginState::Resolving(PluginResolveState::Loaded),
            PluginState::Resolving(PluginResolveState::PluginDeclarationLoaded),
            PluginState::Resolving(PluginResolveState::CompilerVersionMismatch),
            PluginState::Resolving(PluginResolveState::PluginApiVersionMismatch),
            PluginState::Resolving(PluginResolveState::DependenciesNotActive),
            PluginState::Resolving(PluginResolveState::PluginCompatible),
            PluginState::Resolved,
            PluginState::Starting(PluginStartingState::ConstructingProxy),
            PluginState::Starting(PluginStartingState::Registering),
            PluginState::Starting(PluginStartingState::Activating),
            PluginState::Starting(PluginStartingState::ActivationFailed),
            PluginState::Active,
            PluginState::Stopping(PluginStoppingState::Deactivating),
            PluginState::Stopping(PluginStoppingState::Unregistering),
            PluginState::Stopping(PluginStoppingState::RemoveProxy),
            PluginState::Uninstalling(PluginUninstallingState::UnloadDll),
            PluginState::Uninstalling(PluginUninstallingState::UninstallDll),
            PluginState::Uninstalled,
            PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Deactivating)),
            PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Unregistering)),
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
            PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Registering)),
            PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Activating)),
            PluginState::Disabled,
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
        self.plugin_containers.get(id).map(|e| String::from(e.path.to_string_lossy()))
    }

    fn get_plugin_state(&self, id: &Uuid) -> Option<PluginState> {
        self.plugin_containers.get(id).map(|e| e.state)
    }

    fn get_plugins(&self) -> Vec<Uuid> {
        self.plugin_containers.iter().map(|p| *p.key()).collect()
    }

    fn get_plugins_with_state(&self, state: PluginState) -> Vec<Uuid> {
        self.plugin_containers.iter().filter(|p| p.state == state).map(|p| p.key().to_owned()).collect()
    }

    fn get_plugins_with_states(&self, state1: PluginState, state2: PluginState) -> Vec<Uuid> {
        self.plugin_containers
            .iter()
            .filter(|p| p.state == state1 || p.state == state2)
            .map(|p| p.key().to_owned())
            .collect()
    }

    fn get_plugins_not_having_state(&self, state: PluginState) -> Vec<Uuid> {
        self.plugin_containers.iter().filter(|p| p.state != state).map(|p| p.key().to_owned()).collect()
    }

    fn get_plugin_by_dependency(&self, plugin_dependency: &PluginDependency) -> Option<Uuid> {
        let version_requirement = VersionReq::parse(plugin_dependency.version).ok()?;
        self.plugin_containers
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
            .map(|e| *e.key())
    }

    fn deploy_dll(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => {
                trace!("Plugin {id} is deploying the dynamic linked library");
                plugin_container.deploy_dll()
            }
            None => NoChange,
        }
    }

    fn load_dll(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => {
                trace!("Plugin {id} is loading the dynamic linked library");
                plugin_container.load_dll()
            }
            None => NoChange,
        }
    }

    fn load_plugin_declaration(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => {
                trace!("Plugin {id} is loading the plugin declaration");
                plugin_container.value_mut().load_plugin_declaration()
            }
            None => NoChange,
        }
    }

    fn check_plugin_compatibility(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => {
                trace!("Plugin {id} is checked for compatibility");
                plugin_container.value_mut().check_compatibility()
            }
            None => NoChange,
        }
    }

    fn load_plugin_dependencies(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => {
                trace!("Plugin {id} is loading the list of dependencies");
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
        self.plugin_containers.get(id).iter().any(|e| !e.dependencies.is_empty())
    }

    fn get_dependencies(&self, id: &Uuid) -> DashSet<PluginDependency> {
        self.plugin_containers.get(id).map(|e| e.dependencies.clone()).unwrap_or_default()
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
            .filter(|d| self.get_dependency_state(d) != PluginState::Active)
            .map(|d| *d)
            .collect()
    }

    fn get_dependents(&self, id: &Uuid) -> Vec<Uuid> {
        let mut dependents = Vec::new();
        for plugin_container in self.plugin_containers.iter() {
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
        match self.plugin_containers.get_mut(id) {
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
            debug!("Plugin {id} has no unsatisfied dependencies");
            let new_state = if refreshing {
                PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::ConstructingProxy))
            } else {
                PluginState::Resolved
            };
            self.set_state(id, new_state)
        } else {
            trace!("Plugin {id} has unsatisfied dependencies");
            let new_state = if refreshing {
                PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::DependenciesNotActive))
            } else {
                PluginState::Resolving(PluginResolveState::DependenciesNotActive)
            };
            self.set_state(id, new_state)
        }
    }

    fn construct_proxy(&self, id: &Uuid, plugin_context: Arc<dyn PluginContext + Send + Sync>) -> PluginTransitionResult {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => plugin_container.construct_proxy(plugin_context.clone()),
            None => NoChange,
        }
    }

    fn register(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => {
                if plugin_container.state != PluginState::Starting(PluginStartingState::Registering)
                    && plugin_container.state != PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Registering))
                {
                    return NoChange;
                }
                let refreshing = plugin_container.state == PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Registering));
                if refreshing {
                    plugin_container.state = PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Activating));
                } else {
                    plugin_container.state = PluginState::Starting(PluginStartingState::Activating);
                }
                Changed
            }
            None => NoChange,
        }
    }

    async fn activate(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => plugin_container.activate().await,
            None => NoChange,
        }
    }

    fn are_all_stopped(&self) -> bool {
        self.plugin_containers.iter().all(|p| match p.state {
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
            PluginState::Disabled => true,
        })
    }

    async fn deactivate(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => plugin_container.deactivate().await,
            None => NoChange,
        }
    }

    fn unregister(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => {
                if plugin_container.state != PluginState::Stopping(PluginStoppingState::Unregistering)
                    && plugin_container.state != PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Unregistering))
                {
                    return NoChange;
                }
                let refreshing = plugin_container.state == PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Unregistering));
                if refreshing {
                    plugin_container.state = PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::RemoveProxy));
                } else {
                    plugin_container.state = PluginState::Stopping(PluginStoppingState::RemoveProxy);
                }
                Changed
            }
            None => NoChange,
        }
    }

    fn remove_proxy(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => plugin_container.remove_proxy(),
            None => NoChange,
        }
    }

    fn unload_dll(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => plugin_container.value_mut().unload_dll(),
            None => NoChange,
        }
    }

    fn uninstall_dll(&self, id: &Uuid) -> PluginTransitionResult {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => match plugin_container.value_mut().uninstall_dll() {
                Changed => {
                    self.plugin_containers.remove(id);
                    Changed
                }
                NoChange => NoChange,
            },
            None => NoChange,
        }
    }

    fn start(&self, id: &Uuid) -> Result<(), PluginStartError> {
        match self.plugin_containers.get_mut(id) {
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
        let result = match self.plugin_containers.get_mut(id) {
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
            let _ = self.stop(&id);
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
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => plugin_container.uninstall(),
            None => Err(PluginUninstallError::AlreadyUninstalled),
        }
    }

    fn redeploy(&self, id: &Uuid) -> Result<(), PluginDeployError> {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => plugin_container.redeploy(),
            None => Err(PluginDeployError::NotFound),
        }
    }

    fn disable(&self, id: &Uuid) -> Result<(), PluginDisableError> {
        match self.plugin_containers.get_mut(id) {
            Some(mut plugin_container) => plugin_container.disable(),
            None => Err(PluginDisableError::NotFound),
        }
    }
}

#[async_trait]
impl Lifecycle for PluginContainerManagerImpl {}
