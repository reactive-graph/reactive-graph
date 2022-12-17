use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use async_trait::async_trait;
use log::debug;
use log::trace;
use log::warn;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::FlowTypeManager;
use crate::api::Lifecycle;
use crate::api::PluginContainerManager;
use crate::api::PluginContextFactory;
use crate::api::PluginResolver;
use crate::api::ReactiveFlowInstanceManager;
use crate::api::RelationTypeManager;
use crate::api::WebResourceManager;
use crate::di::*;
use crate::plugin::PluginResolverMode;
use crate::plugin::PluginTransitionResult;
use crate::plugin::PluginTransitionResult::Changed;
use crate::plugin::PluginTransitionResult::NoChange;
use crate::plugins::PluginRefreshingState;
use crate::plugins::PluginResolveState;
use crate::plugins::PluginStartingState;
use crate::plugins::PluginState;
use crate::plugins::PluginStoppingState;
use crate::plugins::PluginUninstallingState;

#[wrapper]
pub struct PluginResolverModeState(RwLock<PluginResolverMode>);

#[provides]
fn create_plugin_registry_mode() -> PluginResolverModeState {
    PluginResolverModeState(RwLock::new(PluginResolverMode::Neutral))
}

#[component]
pub struct PluginResolverImpl {
    // Type System
    component_manager: Wrc<dyn ComponentManager>,
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    relation_type_manager: Wrc<dyn RelationTypeManager>,
    flow_type_manager: Wrc<dyn FlowTypeManager>,
    // Instance System
    reactive_flow_instance_manager: Wrc<dyn ReactiveFlowInstanceManager>,
    // System Services
    web_resource_manager: Wrc<dyn WebResourceManager>,

    plugin_container_manager: Wrc<dyn PluginContainerManager>,

    plugin_context_factory: Wrc<dyn PluginContextFactory>,

    /// The mode.
    pub mode: PluginResolverModeState,
}

impl PluginResolverImpl {}

#[async_trait]
#[provides]
impl PluginResolver for PluginResolverImpl {
    fn resolve_until_idle(&self) {
        let mut i = 0;
        while self.resolve() == Changed && i < 1000 {
            // TODO: timeout + circuit breaker
            i += 1;
        }
        if i == 1000 {
            warn!("Plugin resolver stopped after {} iterations", 1000);
        }
    }

    fn resolve(&self) -> PluginTransitionResult {
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
        // Starting(ConstructingProxy) --> Starting(InjectingContext)
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Starting(PluginStartingState::ConstructingProxy),
            PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::ConstructingProxy)),
        ) {
            if self.plugin_container_manager.construct_proxy(&id) == Changed {
                return Changed;
            }
        }
        // Starting(InjectingContext) --> Starting(Registering)
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Starting(PluginStartingState::InjectingContext),
            PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::InjectingContext)),
        ) {
            if let Some(plugin_context) = self.plugin_context_factory.get() {
                if self.plugin_container_manager.inject_context(&id, plugin_context) == Changed {
                    return Changed;
                }
            }
        }
        // Starting(Registering) --> Starting(Activating)
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Starting(PluginStartingState::Registering),
            PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Registering)),
        ) {
            if self.plugin_container_manager.register(
                &id,
                self.component_manager.clone(),
                self.entity_type_manager.clone(),
                self.relation_type_manager.clone(),
                self.flow_type_manager.clone(),
                self.reactive_flow_instance_manager.clone(),
                self.web_resource_manager.clone(),
            ) == Changed
            {
                return Changed;
            }
        }
        // Starting(Activating) --> Active
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Starting(PluginStartingState::Activating),
            PluginState::Refreshing(PluginRefreshingState::Starting(PluginStartingState::Activating)),
        ) {
            if self.plugin_container_manager.activate(&id) == Changed {
                return Changed;
            }
        }
        // Stopping(Deactivating) --> Stopping(Unregistering)
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Stopping(PluginStoppingState::Deactivating),
            PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Deactivating)),
        ) {
            if self.plugin_container_manager.deactivate(&id) == Changed {
                return Changed;
            }
        }
        // Stopping(Unregistering) --> Stopping(RemoveContext)
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Stopping(PluginStoppingState::Unregistering),
            PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::Unregistering)),
        ) {
            if self
                .plugin_container_manager
                .unregister(&id, self.reactive_flow_instance_manager.clone(), self.web_resource_manager.clone())
                == Changed
            {
                return Changed;
            }
        }
        // Stopping(RemoveContext) --> Stopping(RemoveProxy)
        for id in self.plugin_container_manager.get_plugins_with_states(
            PluginState::Stopping(PluginStoppingState::RemoveContext),
            PluginState::Refreshing(PluginRefreshingState::Stopping(PluginStoppingState::RemoveContext)),
        ) {
            if self.plugin_container_manager.remove_context(&id) == Changed {
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
        debug!("Plugin resolver finished\n{}\n", self.plugin_container_manager.count_by_states());
        NoChange
    }

    fn set_mode(&self, mode: PluginResolverMode) {
        let mut writer = self.mode.0.write().unwrap();
        *writer = mode;
    }

    fn get_mode(&self) -> PluginResolverMode {
        let reader = self.mode.0.read().unwrap();
        *reader
    }
}

impl Lifecycle for PluginResolverImpl {
    fn init(&self) {
        self.set_mode(PluginResolverMode::Starting);
        self.resolve_until_idle();
        for id in self.plugin_container_manager.get_plugins_not_having_state(PluginState::Active) {
            let stem = self.plugin_container_manager.get_stem(&id).unwrap_or_default();
            for d in self.plugin_container_manager.get_unsatisfied_dependencies(&id) {
                warn!("Plugin {} {} has unsatisfied dependency: {}:{}", &id, &stem, d.name, d.version);
            }
        }
        self.set_mode(PluginResolverMode::Neutral);
    }

    fn shutdown(&self) {
        self.set_mode(PluginResolverMode::Stopping);
        self.plugin_container_manager.stop_all();
        let mut i = 0;
        while !self.plugin_container_manager.are_all_stopped() && i < 100 {
            self.resolve_until_idle();
            thread::sleep(Duration::from_millis(100));
            i += 1;
            // TODO: force stop after timeout
        }
        if i == 100 {
            warn!("Plugin resolver stopped after {} iterations", 100);
        }
    }
}
