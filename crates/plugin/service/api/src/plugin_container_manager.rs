use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashSet;
use springtime_di::injectable;
use uuid::Uuid;

use crate::PluginTransitionResult;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_api::PluginContext;
use reactive_graph_plugin_api::PluginDependency;
use reactive_graph_plugin_api::PluginDeployError;
use reactive_graph_plugin_api::PluginDisableError;
use reactive_graph_plugin_api::PluginStartError;
use reactive_graph_plugin_api::PluginState;
use reactive_graph_plugin_api::PluginStopError;
use reactive_graph_plugin_api::PluginUninstallError;

#[injectable]
#[async_trait]
pub trait PluginContainerManager: Send + Sync + Lifecycle {
    // Container Management

    /// Creates a new plugin container for a plugin with the given file stem. The dynamically linked
    /// library is located at the given path which have to be in the plugin installation folder.
    fn create_and_register_plugin_container(&self, stem: String, path: PathBuf) -> Option<Uuid>;

    /// Returns the plugin with the given id from the list of plugin containers.
    fn remove_plugin_container(&self, id: &Uuid);

    /// Returns true, if a plugin with the given file stem exists.
    fn has(&self, stem: &str) -> bool;

    // Getters

    /// Returns the id of the plugin with the given file stem.
    fn get_id(&self, stem: &str) -> Option<Uuid>;

    /// Returns the file stem of the plugin with the given id.
    fn get_stem(&self, id: &Uuid) -> Option<String>;

    /// Returns the name of the plugin with the given id.
    fn name(&self, id: &Uuid) -> Option<String>;

    /// Returns the canonicalized name of the plugin with the given id.
    fn name_canonicalized(&self, id: &Uuid) -> Option<String>;

    /// Returns the canonicalized name and version of the plugin with the given id.
    fn name_version(&self, id: &Uuid) -> Option<String>;

    /// Returns the description of the plugin with the given id.
    fn description(&self, id: &Uuid) -> Option<String>;

    /// Returns the version of the plugin with the given id.
    fn version(&self, id: &Uuid) -> Option<String>;

    /// Returns the version of the rust compiler the plugin with the given id was compiled with.
    fn rustc_version(&self, id: &Uuid) -> Option<String>;

    /// Returns the version of the plugin API the plugin with the given id was compiled with.
    fn plugin_api_version(&self, id: &Uuid) -> Option<String>;

    // Statistics

    /// Returns the count of all plugins.
    fn count(&self) -> usize;

    /// Returns the count of plugins with the given state.
    fn count_by_state(&self, state: &PluginState) -> usize;

    /// Returns a string withs stats abouts the states of the plugins
    fn count_by_states(&self) -> String;

    fn count_by_state_str(&self, state: &PluginState) -> String;

    // Getter

    /// Returns the path of the plugin with the given id.
    fn get_plugin_path(&self, id: &Uuid) -> Option<String>;

    /// Returns the state of the plugin with the given id.
    fn get_plugin_state(&self, id: &Uuid) -> Option<PluginState>;

    // Queries

    /// Returns a list of ids of all plugins.
    fn get_plugins(&self) -> Vec<Uuid>;

    /// Returns a list of ids of plugins with the given state.
    fn get_plugins_with_state(&self, state: PluginState) -> Vec<Uuid>;

    /// Returns a list of ids of plugins which have one of the both states.
    fn get_plugins_with_states(&self, state1: PluginState, state2: PluginState) -> Vec<Uuid>;

    /// Returns a list of ids of plugins which doesn't have the given state.
    fn get_plugins_not_having_state(&self, state: PluginState) -> Vec<Uuid>;

    // Dependency management?
    /// Returns the id of the plugin by a dependency coordinate.
    fn get_plugin_by_dependency(&self, plugin_dependency: &PluginDependency) -> Option<Uuid>;

    // Transitions

    /// Deploys the dynamic linked library file from the plugin hot deploy folder into the
    /// plugin installation folder.
    ///
    /// The target filename will contain a timestamp in order to avoid that a cached previous
    /// version of the DLL will be loaded.
    fn deploy_dll(&self, id: &Uuid) -> PluginTransitionResult;

    /// Loads the dynamic linked library into memory.
    fn load_dll(&self, id: &Uuid) -> PluginTransitionResult;

    /// Loads the plugin declaration of the plugin with the given id.
    ///
    /// The plugin declaration contains the version of the rust compiler and the plugin API in
    /// order to check for ABI compatibility.
    ///
    /// The plugin have to contain a symbol named 'plugin_declaration'.
    fn load_plugin_declaration(&self, id: &Uuid) -> PluginTransitionResult;

    /// Performs a compatibility check on the plugin with the given id.
    ///
    /// The compatibility check depends on the plugin declaration.
    ///
    /// If the plugin is incompatible corresponding error messages will appear and the plugin
    /// will be uninstalled.
    fn check_plugin_compatibility(&self, id: &Uuid) -> PluginTransitionResult;

    /// Loads the dependencies of the plugin with the given id.
    ///
    /// The list of dependencies depends on the plugin declaration.
    fn load_plugin_dependencies(&self, id: &Uuid) -> PluginTransitionResult;

    // Dependency Management

    /// Returns the state of the plugin with the given dependency coordinate.
    fn get_dependency_state(&self, dependency: &PluginDependency) -> PluginState;

    /// Returns true, if the plugin with the given id has one or multiple dependencies.
    fn has_dependencies(&self, id: &Uuid) -> bool;

    /// Returns a list of dependency coordinates for the plugin with the given id.
    fn get_dependencies(&self, id: &Uuid) -> DashSet<PluginDependency>;

    /// Returns true, if the plugin with the given id has unsatisfied dependencies.
    fn has_unsatisfied_dependencies(&self, id: &Uuid) -> bool;

    /// Returns a list of unsatisfied dependencies of the plugin with the given id.
    fn get_unsatisfied_dependencies(&self, id: &Uuid) -> DashSet<PluginDependency>;

    /// Returns a list of plugin ids which are dependents of the plugin with the given id.
    fn get_dependents(&self, id: &Uuid) -> Vec<Uuid>;

    // Transitions 2

    /// Sets the given new state of the plugin with the given id.
    fn set_state(&self, id: &Uuid, new_state: PluginState) -> PluginTransitionResult;

    /// Calculates the state of the dependencies of the plugin with the given id.
    fn resolve_dependencies_state(&self, id: &Uuid, refreshing: bool) -> PluginTransitionResult;

    /// Constructs a plugin proxy object for the plugin with the given id.
    ///
    /// The plugin proxy makes sure it can't outlive the library it came from.
    fn construct_proxy(&self, id: &Uuid, plugin_context: Arc<dyn PluginContext + Send + Sync>) -> PluginTransitionResult;

    /// Registers providers of the plugin with the given id.
    fn register(&self, id: &Uuid) -> PluginTransitionResult;

    /// Calls the activate method of the plugin with the given id.
    async fn activate(&self, id: &Uuid) -> PluginTransitionResult;

    // Lifecycle management

    /// Returns true, if all plugins are stopped.
    ///
    /// Plugins are considered as stopped if they are not starting, not active and not stopping.
    fn are_all_stopped(&self) -> bool;

    // Transitions 3

    /// Calls the deactivate method of the plugin with the given id.
    async fn deactivate(&self, id: &Uuid) -> PluginTransitionResult;

    /// Unregisters the providers of the plugin with the given id.
    fn unregister(&self, id: &Uuid) -> PluginTransitionResult;

    /// Removes the plugin proxy of the plugin with the given id.
    fn remove_proxy(&self, id: &Uuid) -> PluginTransitionResult;

    /// Closes the library by dropping it.
    fn unload_dll(&self, id: &Uuid) -> PluginTransitionResult;

    /// Deletes the dynamically linked library file.
    fn uninstall_dll(&self, id: &Uuid) -> PluginTransitionResult;

    // High Level API

    /// Start the plugin with the given id.
    fn start(&self, id: &Uuid) -> Result<(), PluginStartError>;

    /// Starts the plugin with the given file stem.
    fn start_by_stem(&self, stem: &str) -> Result<(), PluginStartError>;

    /// Starts all plugins which are dependent of the plugin with the given id and
    /// have no unsatisfied dependencies.
    fn start_dependent_with_satisfied_dependencies(&self, id: &Uuid) -> bool;

    /// Stops the given plugin.
    /// Recursively stops all plugins which depends on the stopped plugin.
    fn stop(&self, id: &Uuid) -> Result<(), PluginStopError>;

    /// Stops the plugin with the given file stem.
    fn stop_by_stem(&self, stem: &str) -> Result<(), PluginStopError>;

    /// Stops all plugins.
    fn stop_all(&self);

    /// Stops active plugins which have unsatisfied dependencies.
    fn stop_active_with_unsatisfied_dependencies(&self) -> bool;

    /// Uninstalls the plugin with the given id.
    fn uninstall(&self, id: &Uuid) -> Result<(), PluginUninstallError>;

    /// Redeploys the plugin with the given id.
    fn redeploy(&self, id: &Uuid) -> Result<(), PluginDeployError>;

    /// Disables the plugin with the given id.
    fn disable(&self, id: &Uuid) -> Result<(), PluginDisableError>;
}
