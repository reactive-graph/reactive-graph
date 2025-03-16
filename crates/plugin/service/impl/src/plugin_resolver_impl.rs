use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use async_trait::async_trait;
use log::info;
use log::trace;
use log::warn;
use springtime_di::Component;
use springtime_di::component_alias;
use tokio::task::yield_now;
use uuid::Uuid;

use reactive_graph_config_api::ConfigManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_api::PluginRefreshingState;
use reactive_graph_plugin_api::PluginResolveState;
use reactive_graph_plugin_api::PluginStartingState;
use reactive_graph_plugin_api::PluginState;
use reactive_graph_plugin_api::PluginStoppingState;
use reactive_graph_plugin_api::PluginUninstallingState;
use reactive_graph_plugin_service_api::PluginContainerManager;
use reactive_graph_plugin_service_api::PluginContextFactory;
use reactive_graph_plugin_service_api::PluginResolver;
use reactive_graph_plugin_service_api::PluginResolverMode;
use reactive_graph_plugin_service_api::PluginTransitionResult;
use reactive_graph_plugin_service_api::PluginTransitionResult::Changed;
use reactive_graph_plugin_service_api::PluginTransitionResult::NoChange;

const MAX_ITERATIONS: u32 = 1000;

// pub struct PluginResolverModeState(RwLock<PluginResolverMode>);

fn create_plugin_resolver_mode() -> RwLock<PluginResolverMode> {
    RwLock::new(PluginResolverMode::Neutral)
}

#[derive(Component)]
pub struct PluginResolverImpl {
    plugin_container_manager: Arc<dyn PluginContainerManager + Send + Sync>,

    plugin_context_factory: Arc<dyn PluginContextFactory + Send + Sync>,

    config_manager: Arc<dyn ConfigManager + Send + Sync>,

    /// The resolver can be in three modes: Starting, Neutral and Stopping.
    #[component(default = "create_plugin_resolver_mode")]
    pub mode: RwLock<PluginResolverMode>,
}

impl PluginResolverImpl {
    fn is_disabled(&self) -> bool {
        self.config_manager.get_plugins_config().disabled.unwrap_or(false)
    }

    fn is_plugin_disabled(&self, id: Uuid) -> bool {
        let stem = self.plugin_container_manager.get_stem(&id);
        let name = self.plugin_container_manager.name(&id);
        let short_name = self.plugin_container_manager.name_canonicalized(&id);

        if let Some(enabled_plugins) = self.config_manager.get_plugins_config().enabled_plugins {
            if let (Some(name), Some(short_name)) = (name.clone(), short_name.clone()) {
                if !enabled_plugins.contains(&name) && !enabled_plugins.contains(&short_name) {
                    return true;
                }
            }
            return false;
        }

        if let Some(disabled_plugins) = self.config_manager.get_plugins_config().disabled_plugins {
            if let Some(stem) = stem {
                if disabled_plugins.contains(&stem) {
                    return true;
                }
            }
            if let Some(name) = name {
                if disabled_plugins.contains(&name) {
                    return true;
                }
            }
            if let Some(short_name) = short_name {
                if disabled_plugins.contains(&short_name) {
                    return true;
                }
            }
        }
        false
    }

    fn log_unsatisfied_dependencies(&self) {
        for id in self.plugin_container_manager.get_plugins_not_having_state(PluginState::Active) {
            let name = self.plugin_container_manager.name_canonicalized(&id).unwrap_or(id.to_string());
            for d in self.plugin_container_manager.get_unsatisfied_dependencies(&id) {
                trace!("Plugin {} {} has unsatisfied dependency: {}", id, &name, d.name_version());
                match self.plugin_container_manager.get_plugin_by_dependency(&d) {
                    Some(dependency_id) => {
                        let dependency_name_version = self.plugin_container_manager.name_version(&dependency_id).unwrap_or(dependency_id.to_string());
                        // let dependency_name = self.plugin_container_manager.name_canonicalized(&dependency_id).unwrap_or_default();
                        // let dependency_version = self.plugin_container_manager.version(&dependency_id).unwrap_or_default();
                        let dependency_state = self
                            .plugin_container_manager
                            .get_plugin_state(&dependency_id)
                            .unwrap_or(PluginState::Uninstalled);
                        warn!(
                            "Plugin {} has unsatisfied dependency: {} - which exists ({}) but has state {:?}",
                            &name,
                            d.name_version(),
                            dependency_name_version,
                            dependency_state
                        );
                    }
                    None => {
                        warn!("Plugin {} has unsatisfied dependency: {} - which doesn't exist", &name, d.name_version());
                    }
                }
            }
        }
    }
}

#[async_trait]
#[component_alias]
impl PluginResolver for PluginResolverImpl {
    async fn resolve_until_idle(&self) {
        if self.is_disabled() {
            trace!("Skipping all plugins");
            return;
        }
        let mut i = 0;
        while self.resolve().await == Changed && i < MAX_ITERATIONS {
            i += 1;
            if i % 50 == 0 {
                yield_now().await
            }
        }

        if i >= MAX_ITERATIONS {
            warn!("Plugin resolver force stopped after {i} iterations");
        } else {
            trace!("Plugin resolver finished after {i} iterations");
        }
    }

    async fn stop_until_all_stopped(&self) {
        self.transition_to_fallback_states().await;
        let mut i = 0;
        while !self.plugin_container_manager.are_all_stopped() && i < MAX_ITERATIONS {
            self.resolve_until_idle().await;
            tokio::time::sleep(Duration::from_millis(10)).await;
            i += 1;
            if i % 50 == 0 {
                yield_now().await
            }
            // TODO: force stop after timeout
        }
        if i >= MAX_ITERATIONS {
            warn!("Plugin resolver force stopped after {i} iterations");
        } else {
            trace!("Plugin resolver finished after {i} iterations");
        }
    }

    async fn resolve(&self) -> PluginTransitionResult {
        let mode = self.get_mode();
        trace!("Resolving plugins (mode: {:?})", mode);
        // PluginUninstallingState::UnloadDll --> PluginUninstallingState::UninstallDll
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Uninstalling(PluginUninstallingState::UnloadDll),
            PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UnloadDll)),
        ) {
            if self.plugin_container_manager.unload_dll(&id) == Changed {
                return Changed;
            };
        }
        // PluginUninstallingState::UninstallDll --> Uninstalled
        if let Some(id) = self
            .plugin_container_manager
            .get_plugins_with_states(
                PluginState::Uninstalling(PluginUninstallingState::UninstallDll),
                PluginState::Refreshing(PluginRefreshingState::Uninstalling(PluginUninstallingState::UninstallDll)),
            )
            .into_iter()
            .next()
        {
            self.plugin_container_manager.uninstall_dll(&id);
            return Changed;
        }
        // Uninstalled --> Removed
        if let Some(id) = self
            .plugin_container_manager
            .get_plugins_with_state(PluginState::Uninstalled)
            .into_iter()
            .next()
        {
            self.plugin_container_manager.remove_plugin_container(&id);
            return Changed;
        }
        // * --> PluginState::Disabled
        for id in self.plugin_container_manager.get_plugins_not_having_state(PluginState::Disabled) {
            if self.is_plugin_disabled(id) && self.plugin_container_manager.disable(&id).is_ok() {
                return Changed;
            }
        }
        // PluginResolveState::CompilerVersionMismatch --> Uninstalling
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Resolving(PluginResolveState::CompilerVersionMismatch),
            PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::CompilerVersionMismatch)),
        ) {
            // TODO: Add configuration property: auto_uninstall_incompatible_plugins
            if self
                .plugin_container_manager
                .set_state(&id, PluginState::Uninstalling(PluginUninstallingState::UnloadDll))
                == Changed
            {
                return Changed;
            }
        }
        // PluginResolveState::PluginApiVersionMismatch --> Uninstalling
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Resolving(PluginResolveState::PluginApiVersionMismatch),
            PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::PluginApiVersionMismatch)),
        ) {
            // TODO: Add configuration property: auto_uninstall_incompatible_plugins
            if self
                .plugin_container_manager
                .set_state(&id, PluginState::Uninstalling(PluginUninstallingState::UnloadDll))
                == Changed
            {
                return Changed;
            }
        }
        // Deploying --> Installed
        for id in self
            .plugin_container_manager
            .get_plugins_with_state(PluginState::Refreshing(PluginRefreshingState::Deploying))
        {
            if self.plugin_container_manager.deploy_dll(&id) == Changed {
                return Changed;
            }
        }
        // Installed --> PluginResolveState::Loaded
        //           --> Uninstalling
        for id in self
            .plugin_container_manager
            .get_plugins_with_states(PluginState::Installed, PluginState::Refreshing(PluginRefreshingState::Installed))
        {
            if self.plugin_container_manager.load_dll(&id) == Changed {
                return Changed;
            }
        }
        // PluginResolveState::Loaded --> PluginResolveState::PluginDeclarationLoaded
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Resolving(PluginResolveState::Loaded),
            PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::Loaded)),
        ) {
            if self.plugin_container_manager.load_plugin_declaration(&id) == Changed {
                return Changed;
            }
        }
        // PluginResolveState::PluginDeclarationLoaded --> PluginResolveState::PluginCompatible
        //                                             --> PluginResolveState::CompilerVersionMismatch
        //                                             --> PluginResolveState::PluginApiVersionMismatch
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Resolving(PluginResolveState::PluginDeclarationLoaded),
            PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::PluginDeclarationLoaded)),
        ) {
            if self.plugin_container_manager.check_plugin_compatibility(&id) == Changed {
                return Changed;
            }
        }
        // PluginResolveState::PluginCompatible --> PluginResolveState::DependenciesNotActive
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Resolving(PluginResolveState::PluginCompatible),
            PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::PluginCompatible)),
        ) {
            if self.plugin_container_manager.load_plugin_dependencies(&id) == Changed {
                return Changed;
            }
        }
        // PluginResolveState::DependenciesNotActive --> Resolved
        for id in self
            .plugin_container_manager
            .get_plugins_with_state(PluginState::Resolving(PluginResolveState::DependenciesNotActive))
        {
            if self.plugin_container_manager.resolve_dependencies_state(&id, false) == Changed {
                return Changed;
            }
        }
        // Refreshing::PluginResolveState::DependenciesNotActive --> Starting(ConstructingProxy)
        for id in self
            .plugin_container_manager
            .get_plugins_with_state(PluginState::Refreshing(PluginRefreshingState::Resolving(PluginResolveState::DependenciesNotActive)))
        {
            if self.plugin_container_manager.resolve_dependencies_state(&id, true) == Changed {
                return Changed;
            }
        }
        // Resolved --> Starting(ConstructingProxy)
        //          --> PluginResolveState::DependenciesNotActive
        match mode {
            PluginResolverMode::Starting => {
                for id in self.plugin_container_manager.get_plugins_with_state(PluginState::Resolved) {
                    if self.plugin_container_manager.start(&id).map_err(|_| ()).is_ok() {
                        return Changed;
                    }
                }
            }
            PluginResolverMode::Neutral => {
                for id in self.plugin_container_manager.get_plugins_with_state(PluginState::Resolved) {
                    if self.plugin_container_manager.resolve_dependencies_state(&id, false) == Changed {
                        return Changed;
                    }
                }
            }
            PluginResolverMode::Stopping => {}
        }
        // Starting(ConstructingProxy) --> Starting(Registering)
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Starting(PluginStartingState::ConstructingProxy),
            PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::ConstructingProxy)),
        ) {
            if let Some(plugin_context) = self.plugin_context_factory.get() {
                if self.plugin_container_manager.construct_proxy(&id, plugin_context) == Changed {
                    return Changed;
                }
            }
        }
        // Starting(Registering) --> Starting(Activating)
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Starting(PluginStartingState::Registering),
            PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Registering)),
        ) {
            if self.plugin_container_manager.register(&id) == Changed {
                return Changed;
            }
        }
        // Starting(Activating) --> Active
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Starting(PluginStartingState::Activating),
            PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Activating)),
        ) {
            if self.plugin_container_manager.activate(&id).await == Changed {
                return Changed;
            }
        }
        // Stopping(Deactivating) --> Stopping(Unregistering)
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Stopping(PluginStoppingState::Deactivating),
            PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Deactivating)),
        ) {
            if self.plugin_container_manager.deactivate(&id).await == Changed {
                return Changed;
            }
        }
        // Stopping(Unregistering) --> Stopping(RemoveProxy)
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Stopping(PluginStoppingState::Unregistering),
            PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Unregistering)),
        ) {
            if self.plugin_container_manager.unregister(&id) == Changed {
                return Changed;
            }
        }
        // Stopping(RemoveProxy) --> Resolved
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Stopping(PluginStoppingState::RemoveProxy),
            PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::RemoveProxy)),
        ) {
            if self.plugin_container_manager.remove_proxy(&id) == Changed {
                return Changed;
            }
        }
        // Active --> Deactivating
        for id in self.plugin_container_manager.get_plugins_with_state(PluginState::Active) {
            if mode == PluginResolverMode::Stopping {
                return match self.plugin_container_manager.stop(&id) {
                    Ok(_) => Changed,
                    Err(_) => NoChange,
                };
            }
        }
        // No more actions possible
        info!("Plugin resolver finished\n{}\n", self.plugin_container_manager.count_by_states());
        NoChange
    }

    async fn transition_to_fallback_states(&self) {
        // Stop any failed transitions
        for id in self.plugin_container_manager.get_plugins() {
            if let Some(PluginState::Starting(_) | PluginState::Refreshing(PluginRefreshingState::Starting(_))) =
                self.plugin_container_manager.get_plugin_state(&id)
            {
                info!("Plugin {id} Starting -> Resolved");
                self.plugin_container_manager.set_state(&id, PluginState::Resolved);
            }
        }
    }

    fn set_mode(&self, mode: PluginResolverMode) {
        let mut writer = self.mode.write().unwrap();
        *writer = mode;
    }

    fn get_mode(&self) -> PluginResolverMode {
        let reader = self.mode.read().unwrap();
        *reader
    }
}

#[async_trait]
impl Lifecycle for PluginResolverImpl {
    async fn init(&self) {
        self.set_mode(PluginResolverMode::Starting);
        self.resolve_until_idle().await;
        self.log_unsatisfied_dependencies();
        self.set_mode(PluginResolverMode::Neutral);
    }

    async fn shutdown(&self) {
        self.set_mode(PluginResolverMode::Stopping);
        self.plugin_container_manager.stop_all();
        self.stop_until_all_stopped().await;
    }
}
